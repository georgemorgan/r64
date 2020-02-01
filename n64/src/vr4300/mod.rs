/* cpu.rs - Exposes all of the implementation needed to organize and execute VR4300i opcodes. */

/*

VR43000

  00h = r0/reg0     08h = t0/reg8     10h = s0/reg16    18h = t8/reg24
  01h = at/reg1     09h = t1/reg9     11h = s1/reg17    19h = t9/reg25
  02h = v0/reg2     0Ah = t2/reg10    12h = s2/reg18    1Ah = k0/reg26
  03h = v1/reg3     0Bh = t3/reg11    13h = s3/reg19    1Bh = k1/reg27
  04h = a0/reg4     0Ch = t4/reg12    14h = s4/reg20    1Ch = gp/reg28
  05h = a1/reg5     0Dh = t5/reg13    15h = s5/reg21    1Dh = sp/reg29
  06h = a2/reg6     0Eh = t6/reg14    16h = s6/reg22    1Eh = s8/reg30
  07h = a3/reg7     0Fh = t7/reg15    17h = s7/reg23    1Fh = ra/reg31

*/

const GPR_NAMES: [&'static str; GPR_SIZE] = [
    "r0", "at", "v0", "v1", "a0", "a1", "a2", "a3",
    "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
    "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
    "t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra",
];

use std::fmt;

mod cp0;
mod cp1;
mod instruction;
mod op;

use self::cp0::*;
use self::cp1::*;
use self::op::*;
use self::instruction::*;

use crate::N64;

// Implementation of the VR4300 pipeline
// Does the emulator need to emulate the pipeline?
// Emulating the pipleine makes it easier to write the lambdas that
// implement each instruction. The instruction can access pre-loaded
// variables from the previous pipeline stages instead of needing
// to directly access the memory controller which eliminates lots of
// illegal borrowing. It's also more true to the emulation of the
// VR4300 itself.

// Without the implemetation of the pipeline it would also be very
// difficult to model the emulation of the branch delay slot in a
// way that is naturally compliant with Rust's borrow checker.

#[derive(Copy, Clone)]
pub struct Ic {
    pub op: Inst
}

impl fmt::Debug for Ic {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.op)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rf {
    /* RF stage */
    pub rt: u64,
    pub rs: u64
}

#[derive(Copy, Clone, Debug)]
pub struct Ex {
    /* EX stage */
    pub stalled: bool,
    pub ol: u64,
    pub br: bool,
    pub wlr: bool
}

#[derive(Copy, Clone, Debug)]
pub struct Dc {
    pub dc: u64
}

#[derive(Copy, Clone, Debug)]
pub struct Wb {

}

pub struct Pl {
    /* delay slot program counter */
    ds_pc: u64,

    pub ic : Ic,
    pub rf: Rf,
    pub ex: Ex,
    pub dc: Dc,
    pub wb: Wb
}

/* Size of the general purpose register file. */
const GPR_SIZE: usize = 32;

pub struct VR4300 {

    /* 5 stage pipeline */
    pub pl: Pl,

    /* mmu / tlb co-processor */
    pub cp0: CP0,
    /* floating point co-processor */
    pub cp1: CP1,

    pub gpr: [u64; GPR_SIZE],

    pub hi: u64,
    pub lo: u64,
    pub ll: u8,

    pub pc: u64
}

impl VR4300 {

    pub fn new(pc: u64) -> VR4300 {
        VR4300 {

            pl: Pl {
                ds_pc: 0,

                /* IC stage */
                ic: Ic {
                    op: Inst(0)
                },

                /* RF stage */
                rf: Rf {
                    rt: 0,
                    rs: 0
                },

                /* EX stage */
                ex: Ex {
                    stalled: false,
                    ol: 0,
                    br: false,
                    wlr: false
                },

                /* DC stage */
                dc: Dc {
                    dc: 0
                },

                /* WB stage */
                wb: Wb {

                }
            },

            cp0: CP0::new(),
            cp1: CP1::new(),

            gpr: [0; GPR_SIZE],

            hi: 0,
            lo: 0,
            ll: 0,

            pc: pc
        }
    }

    fn rgpr(&self, reg: usize) -> u64 {
        self.gpr[reg]
    }

    fn wgpr(&mut self, val: u64, reg: usize) {
        match reg {
            0 => {

            }, _ => {
                self.gpr[reg] = val;
            },
        }
    }

    /* IC - Instruction Cache Fetch */
    pub fn ic<F>(&mut self, rmem: F) where F: Fn(u32) -> u32 {

        /* if there is a branch waiting in the delay slot, we need to update the PC here */
        if self.pl.ds_pc != 0 {
            self.pc = self.pl.ds_pc;
            self.pl.ds_pc = 0;
        }

        let val = rmem((self.pc) as u32);
        self.pl.ic.op = Inst(val);

        println!("{:#?}\n", self.pl.ic);

        self.pc += 4;
    }

    /* RF - Register Fetch */
    pub fn rf(&mut self) {
        match self.pl.ic.op.class() {
            OpC::C => {
                self.pl.rf.rs = self.cp0.rgpr(self.pl.ic.op._rd()) as u64
            }, _ => {
                self.pl.rf.rs = self.rgpr(self.pl.ic.op._rs());
            }
        }

        self.pl.rf.rt = self.rgpr(self.pl.ic.op._rt());

        println!("{:#?}\n", self.pl.rf);
    }

    /* EX - Execution */
    pub fn ex(&mut self) {
        // /* stall if the register is the dest of the RF instruction */
        // match self.pl.ic.op.class() {
        //
        //     OpC::I | OpC::L => {
        //         if self.pl.ic.op._rs() == self.pl.ic.op._rt() || self.pl.ic.op._rt() == self.pl.ic.op._rt() {
        //             self.pl.ex.stalled = true;
        //             return;
        //         }
        //     }, OpC::R => {
        //         if self.pl.ic.op._rs() == self.pl.ic.op._rd() || self.pl.ic.op._rt() == self.pl.ic.op._rd() {
        //             self.pl.ex.stalled = true;
        //             return;
        //         }
        //     } _=> {
        // 
        //     }
        // }

        println!("CLASS: {:?}", self.pl.ic.op.class());

        match self.pl.ic.op.op() {
            Op::Syscall => {
                if self.pl.ic.op.sa() > 0 {
                    let result = if self.pl.ic.op._rt() == 16 { "Pass" }  else { "Fail" };
                    println!("t #Result - ISA:{:X}  Set:{:X}  Test:{:X}  Result:{:?}", self.pl.ic.op._rs(), self.pl.ic.op._rd(), self.pl.ic.op.sa(), result);
                }
            }, _ => {
                match self.pl.ic.op.class() {
                    OpC::L => {

                    }, OpC::C => {

                    }, OpC::B => {
                        self.pl.ic.op.ex()(&mut self.pl);

                        /* if a branch will occur, set the delay slot program counter */
                        if self.pl.ex.br {
                            let offset = ((self.pl.ic.op.offset() as i16 as i32) << 2) as i64;
                            self.pl.ds_pc = (self.pc as i64 + offset) as u64;
                        }

                    }, OpC::J => {


                    }, _ => {
                        self.pl.ic.op.ex()(&mut self.pl);
                    }
                }
            }
        }

        println!("{:#?}\n", self.pl.ex);
    }

    /* DC - Data Cache Fetch */
    pub fn dc<F>(&mut self, rmem: F) where F: Fn(u32) -> u32 {

        match self.pl.ic.op.class() {
            OpC::L => {
                let base = self.pl.rf.rs as i64;
                let offset = self.pl.ic.op.offset() as i16 as i64;
                self.pl.dc.dc = rmem((base + offset) as u32) as u64;
                /* need to call the ex function as a hack to get ol populated */
                self.pl.ic.op.ex()(&mut self.pl);
            }, _ => {

            }
        }

        println!("{:#?}\n", self.pl.dc);
    }

    /* WB - Write Back */
    pub fn wb<F>(&mut self, mut wmem: F) where F: FnMut(u32, u32) {

        match self.pl.ic.op.class() {

            /* decode instruction types I, L, S, J, B, R, C for writeback */

            OpC::I | OpC::L => {
                /* I and L instructions write back to the rt register */
                self.wgpr(self.pl.ex.ol, self.pl.ic.op._rt());
            }, OpC::S => {
                /* S instructions write back to memory */
                let base = self.pl.rf.rs as i64;
                let offset = self.pl.ic.op.offset() as i16 as i64;
                wmem((base + offset) as u64 as u32, self.pl.ex.ol as u32);
            }, OpC::J | OpC::B => {
                /* J and B instructions wrote to the delay slot program counter and link register */
                if self.pl.ex.wlr {
                    self.wgpr(self.pc + 4, 31);
                }
            }, OpC::R => {
                /* write back to rd */
                self.wgpr(self.pl.ex.ol, self.pl.ic.op._rd());
            }, OpC::C => {
                /* write back to rt on the coprocessor */
                self.cp0.wgpr(self.pl.ex.ol as u32, self.pl.ic.op._rt());
            }
        }

        if self.pl.ex.stalled {
            self.pl.ex.stalled = false;
        }

        println!("{:#?}\n", self.pl.wb);
    }
}

impl fmt::Debug for VR4300 {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        const REGS_PER_LINE: usize = 2;

        for r in 0..GPR_SIZE {
            if (r % REGS_PER_LINE) == 0 {
                writeln!(f, "")?;
            }

            write!(f, "{:02} ({}): {:#018X} ", r, GPR_NAMES[r], self.rgpr(r))?;
        }

        writeln!(f, "")?;

        for r in 0..GPR_SIZE {
            if (r % REGS_PER_LINE) == 0 {
                writeln!(f, "")?;
            }

            write!(f, "{:02} ({:8}): {:#018X} ", r, cp0::CP0_NAMES[r], self.cp0.rgpr(r))?;
        }

        Ok(())

    }
}
