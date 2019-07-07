/* cpu.rs - Exposes all of the implementation needed to organize and execute VR4300i opcodes. */

/*

CPU0

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

use n64::MC;

mod op;
use self::op::*;

mod instruction;
use self::instruction::Inst;

mod cp0;
use self::cp0::*;

mod cp1;
use self::cp1::*;

/* Size of the general purpose register file. */
const GPR_SIZE: usize = 32;

pub struct CPU {
    /* mmu / tlb co-processor */
    pub cp0: CP0,
    /* floating point co-processor */
    pub cp1: CP1,

    pub gpr: [u64; GPR_SIZE],

    pub hi: u64,
    pub lo: u64,
    pub ll: u8,
    pub pc: u64,

    /* last executed instruction for debugging */
    pub last: u32
}

impl CPU {

    pub fn new(pc: u64) -> CPU {
        CPU {
            cp0: CP0::new(),
            cp1: CP1::new(),

            gpr: [0; GPR_SIZE],

            hi: 0,
            lo: 0,
            ll: 0,
            pc: pc,

            last: 0
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

    pub fn exec(&mut self, i: Inst, mc: &mut MC) {

        match i.class() {
            OpC::I => {

                let rs = self.rgpr(i.rs());
                let imm = i.imm();
                let rt = i.function()(0, rs, imm);
                self.wgpr(rt, i.rt());

            }, OpC::L => {

                let base = self.rgpr(i.rs()) as i64;
                let offset = i.offset() as i16 as i64;
                let val = mc.read((base + offset) as u32) as u64;
                let rt = i.function()(val, 0, 0);
                self.wgpr(rt, i.rt());

            }, OpC::S => {

                let base = self.rgpr(i.rs()) as i64;
                let offset = i.offset() as i16 as i64;
                let rt = self.rgpr(i.rt());
                let val = i.function()(rt, 0, 0) as u32;
                mc.write((base + offset) as u32, val);

            }, OpC::J => {

                match i.op() {

                    Op::J => {
                        let target = i.target();
                        /* sub 4 here because we will inc the pc by 4 later */
                        self.pc = target - 4;
                    }, Op::Jal => {
                        let target = i.target();
                        let pc = self.pc;
                        self.wgpr(pc, 31);
                        /* sub 4 here because we will inc the pc by 4 later */
                        self.pc = target - 4;
                    }, Op::Jr => {
                        let target = self.rgpr(i.rs());
                        /* sub 4 here because we will inc the pc by 4 later */
                        self.pc = target - 4;
                    }, Op::Jalr => {
                        let target = self.rgpr(i.rs());
                        let pc = self.pc;
                        self.wgpr(pc, i.rd());
                        /* sub 4 here because we will inc the pc by 4 later */
                        self.pc = target - 4;
                    }, _ => {

                    }

                }

            }, OpC::B => {

                let rs = self.rgpr(i.rs());
                let rt = self.rgpr(i.rt());
                let offset = ((i.offset() as i16 as i32) << 2) as i64;

                let should_branch = i.function()(rt, rs, 0);
                if should_branch > 0 {
                    self.pc = (self.pc as i64 + offset) as u64;
                }

            }, OpC::R => {

                let rs = self.rgpr(i.rs());
                let rt = self.rgpr(i.rt());
                let rd = i.function()(rt, rs, i.sa());
                self.wgpr(rd, i.rd())

            } _ => {
                panic!("Invalid instruction class {:#x}", i.class() as u32);
            }
        }

    }

    pub fn cycle(&mut self, mc: &mut MC) {

        let op = mc.read(self.pc as u32);

        let i = Inst(op);

        /* quick way to store the last instruction */
        self.last = op;

        println!("{:#x}: ({:#x}) {}", self.pc, i.0, i);

        match i.kind() {
            Op::Cop0 => {
                self.cp0.exec(i);
            }, Op::Cop1 => {
                self.cp1.exec(i);
            }, Op::Cop2 => {
                panic!("Attempt to perfrom a coprocessor instruction on an invalid coprocessor.");
            }, Op::Reserved => {
                panic!("Attempt made to execute a reserved instruction {:#x}.", i.opcode());
            }, _ => {
                self.exec(i, mc);
            }
        };

        self.pc += 4;

    }
}

pub fn print_last(cpu: &CPU) {

    let i = Inst(cpu.last);

    println!("{}", i);
    println!("{:02} ({}): {:#018X} ", i.rd(), GPR_NAMES[i.rd()], cpu.rgpr(i.rd()));
    println!("{:02} ({}): {:#018X} ", i.rt(), GPR_NAMES[i.rt()], cpu.rgpr(i.rt()));
    println!("{:02} ({}): {:#018X} ", i.rs(), GPR_NAMES[i.rs()], cpu.rgpr(i.rs()));
}

use self::cp0::CP0_NAMES;

impl fmt::Debug for CPU {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        const REGS_PER_LINE: usize = 2;

        for r in 0..GPR_SIZE {
            if (r % REGS_PER_LINE) == 0 {
                try!(writeln!(f, ""))
            }

            try!(write!(f, "{:02} ({}): {:#018X} ", r, GPR_NAMES[r], self.rgpr(r)))
        }

        try!(writeln!(f, ""));

        for r in 0..GPR_SIZE {
            if (r % REGS_PER_LINE) == 0 {
                try!(writeln!(f, ""))
            }

            try!(write!(f, "{:02} ({:8}): {:#018X} ", r, cp0::CP0_NAMES[r], self.cp0.rgpr(r)))
        }

        try!(write!(f, "\n\nCPU Floating Point Registers:"));

        // for r in 0..GPR_SIZE {
        //     if (r % REGS_PER_LINE) == 0 {
        //         try!(writeln!(f, ""))
        //     }
        //
        //     try!(write!(f, "fpr{:02}: {:21} ", r, self.rfpr(r)))
        // }

        Ok(())

    }
}
