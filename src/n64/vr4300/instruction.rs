/* instruction.rs - Exposes a parsing object for a VR4300i instruction. */

use std::fmt;

use super::*;

pub struct Inst(pub u32);

impl Inst {
    /* Returns the instruction's opcode. */
    pub fn opcode(&self) -> u8 {
        return ((self.0 >> 26) & 0b111111) as u8;
    }

    /* The top-level opcode tuple. */
    fn op_tup_top(&self) -> &OpTup {
        return OP_TABLE[((self.opcode() >> 3) & 0b111) as usize][(self.opcode() & 0b111) as usize];
    }

    /* Returns the opcode's tuple. */
    fn op_tup(&self) -> &OpTup {

        let t = self.op_tup_top();

        match t.0 {
            Op::Special => {
                return SP_OP_TABLE[((self.funct() >> 3) & 0b111) as usize][(self.funct() & 0b111) as usize];
            }, Op::RegImm => {
                return RI_OP_TABLE[((self._rt() >> 3) & 0b11) as usize][(self._rt() & 0b111) as usize];
            }, Op::Cop0 => {
                let t = COP_OP_RS_TABLE[((self._rs() >> 3) & 0b11) as usize][(self._rs() & 0b111) as usize];
                match t.0 {
                    Op::Bc => {
                        /* If the instruction is Bc, we have to get the extended opcode from the RT table. */
                        return COP_OP_RT_TABLE[((self._rt() >> 3) & 0b11) as usize][(self._rt() & 0b111) as usize];
                    }, Op::Co => {
                        /* If the instruction is a Co instruction, then access it from the FN table. */
                        return COP_OP_FN_TABLE[((self.funct() >> 3) & 0b11) as usize][(self.funct() & 0b111) as usize];
                    }, _ => t
                }
            }, Op::Cop1 => {
                panic!("Attempt to resolve Cop1 instruction.")
            }, Op::Cop2 => {
                panic!("Attempt to resolve Cop2 instruction.")
            }, _ => t,
        }
    }

    /* Returns the kind of operation. */
    pub fn kind(&self) -> Op {
        return self.op_tup_top().0;
    }

    /* Returns the enumerated operation type. */
    pub fn op(&self) -> Op {
        return self.op_tup().0;
    }

    /* Returns a string of the opcode for debugging. */
    pub fn op_str(&self) -> String {
        let s = self.op_tup().1;
        match self.kind() {
            Op::Cop0 => {
                format!("{}c0", s)
            }, Op::Cop1 => {
                format!("{}c1", s)
            }, Op::Cop2 => {
                format!("{}c2", s)
            }, _ => s.to_owned()
        }
    }

    /* Returns the class of the opcode. */
    pub fn class(&self) -> OpC {
        return self.op_tup().2;
    }

    /* Returns the opcode's function. */
    pub fn function(&self) -> OpF {
        return self.op_tup().3;
    }

    /* Returns the instruction's source register. */
    pub fn _rs(&self) -> usize {
        ((self.0 >> 21) & 0b11111) as usize
    }

    /* Returns the value of the CPU's rs register. */
    pub fn rs<T: MIPS64>(&self, cpu: &T) -> u64 {
        cpu.rgpr(self._rs())
    }

    /* Writes a value to the CPU's rs register. */
    pub fn wrs<T: MIPS64>(&self, cpu: &mut T, val: u64) {
        cpu.wgpr(val, self._rs());
    }

    /* Returns the instruction's target register. */
    pub fn _rt(&self) -> usize {
        ((self.0 >> 16) & 0b11111) as usize
    }

    /* Returns the value of the CPU's rt register. */
    pub fn rt<T: MIPS64>(&self, cpu: &T) -> u64 {
        cpu.rgpr(self._rt())
    }

    /* Writes a value to the CPU's rt register. */
    pub fn wrt<T: MIPS64>(&self, cpu: &mut T, val: u64) {
        cpu.wgpr(val, self._rt());
    }

    /* Returns the instruciton's destination register. */
    pub fn _rd(&self) -> usize {
        return ((self.0 >> 11) & 0b11111) as usize;
    }

    /* Returns the value of the CPU's rd register. */
    pub fn rd<T: MIPS64>(&self, cpu: &T) -> u64 {
        return cpu.rgpr(self._rd());
    }

    /* Writes a value to the CPU's rd register. */
    pub fn wrd<T: MIPS64>(&self, cpu: &mut T, val: u64) {
        cpu.wgpr(val, self._rd());
    }

    /* Returns the instruction's shift amount. */
    pub fn sa(&self) -> u16 {
        return ((self.0 >> 6) & 0b11111) as u16;
    }

    /* Returns the instruction's immediate value. */
    pub fn imm(&self) -> u16 {
        return (self.0 & 0xffff) as u16;
    }

    /* Return's the function's funct field. */
    pub fn funct(&self) -> u8 {
        return (self.0 & 0b111111) as u8;
    }

    /* The offset in a LD/ST instruction. */
    pub fn offset(&self) -> u16 {
        return (self.0 & 0xffff) as u16;
    }

    /* Returns the function's target. */
    pub fn target(&self) -> u64 {
        return (self.0 & 0x3ffffff) as u64;
    }
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            Op::Cop0 | Op::Cop1 | Op::Cop2 => {
                write!(f, "{} {}, {}", self.op_str(), GPR_NAMES[self._rt()],  CP0_NAMES[self._rd()])
            }, _ => {
                match self.class() {
                    OpC::I => {
                        write!(f, "{} {}, {}, {:#x}", self.op_str(), GPR_NAMES[self._rt()], GPR_NAMES[self._rs()], self.imm())
                    }, OpC::L | OpC::S => {
                        write!(f, "{} {}, {}({})", self.op_str(), GPR_NAMES[self._rt()], self.offset(), GPR_NAMES[self._rs()])
                    }, OpC::J => {

                        match self.op() {

                            Op::J | Op::Jal  => {
                                write!(f, "{} {:#x}\n", self.op_str(), self.target())
                            }, Op::Jr | Op::Jalr => {
                                write!(f, "{} {}\n", self.op_str(),GPR_NAMES[self._rs()])
                            }, _ => {
                                panic!("Unimplemented jump kind {:#x}", self.op() as u32)
                            }

                        }
                    }, OpC::B => {
                        write!(f, "{} {}, {}, {}\n", self.op_str(), GPR_NAMES[self._rs()], GPR_NAMES[self._rt()], (self.offset() as i16 as i32) << 2)
                    }, OpC::R => {
                        write!(f, "{} {}, {}, {}", self.op_str(), GPR_NAMES[self._rd()], GPR_NAMES[self._rs()], GPR_NAMES[self._rt()])
                    }, OpC::C => {
                        write!(f, "coprocessor inst... ")
                    }
                }
            }
        }
    }
}
