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

use n64::*;

mod cp0;
mod cp1;
mod instruction;
mod op;
mod pipeline;

use self::cp0::*;
use self::cp1::*;
use self::op::*;
use self::instruction::*;
use self::pipeline::*;

/* Size of the general purpose register file. */
const GPR_SIZE: usize = 32;

pub struct VR4300 {

    /* 5 stage pipeline */
    pl: Pl,

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

            pl: Pl::new(),

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

    pub fn cycle(&mut self, mc: &mut MC) {
        self.pl.ic(mc);
        self.pl.rf();
        self.pl.ex();
        self.pl.dc(mc);
        self.pl.wb(mc);
    }
}

impl fmt::Debug for VR4300 {

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

        Ok(())

    }
}
