/* cp1.rs - The CP1 (Co-Processor 1) module. */

use super::*;

const CP1_CONFIG: usize = 0x10;

pub struct CP1 {
    /* the 32-bit cop0 general purpose registers */
    regs: [u32; GPR_SIZE],
}

impl CP1 {

    pub fn new() -> CP1 {
        CP1 {
            /* zero-initialize the cop0 registers */
            regs: [0; GPR_SIZE]
        }
    }

    /* reads from a cop0 register */
    pub fn rfpr(&self, reg: usize) -> u64 {
        self.regs[reg] as u64
    }

    /* writes to a cop0 register */
    pub fn wfpr(&mut self, val: u64, reg: usize) {
        self.regs[reg] = val as u32;
    }

    pub fn exec(&mut self, i: Inst) {
        unimplemented!();
    }

}
