use super::*;

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
pub enum PlStage {
    IC = 0,
    RF = 1,
    EX = 2,
    DC = 3,
    WB = 4
}

#[derive(Copy, Clone)]
pub struct Ic {
    pub op: Inst
}

#[derive(Copy, Clone)]
pub struct Rf {
    /* RF stage */
    pub rt: u64,
    pub rs: u64
}

#[derive(Copy, Clone)]
pub struct Ex {
    /* EX stage */
    pub stalled: bool,
    pub ol: u64,
    pub br: bool,
    pub wlr: bool
}

#[derive(Copy, Clone)]
pub struct Dc {
    pub dc: u64
}

#[derive(Copy, Clone)]
pub struct Wb {

}

pub struct Pl {
    /* pipeline cycle */
    pcycle: usize,
    /* delay slot program counter */
    ds_pc: u64,

    pub ic : Ic,
    pub rf: Rf,
    pub ex: Ex,
    pub dc: Dc,
    pub wb: Wb
}

impl Pl {
    pub fn new() -> Pl {
        Pl {
            pcycle: 0,

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
        }
    }

    /* IC - Instruction Cache Fetch */
    pub fn ic(&mut self, mc: &MC) {

        /* if there is a branch waiting in the delay slot, we need to update the PC here */
        if self.ds_pc != 0 {
            cpu.pc = self.ds_pc;
            self.ds_pc = 0;
        }

        let val = mc.read((cpu.pc) as u32);
        self.ic.op = Inst(val);

        println!("IC {}", self.ic.op);

        cpu.pc += 4;
    }

    /* RF - Register Fetch */
    pub fn rf(&mut self) {

        println!("RF {}", self.ic.op);

        match self.ic.op.class() {
            OpC::C => {
                self.rf.rs = cpu.cp0.rgpr(self.ic.op._rd()) as u64
            }, _ => {
                self.rf.rs = cpu.rgpr(self.ic.op._rs());
            }
        }

        self.rf.rt = cpu.rgpr(self.ic.op._rt());
    }

    /* EX - Execution */
    pub fn ex(&mut self) {

        println!("EX {}", self.ic.op);

        /* stall if the register is the dest of the RF instruction */
        match self.ic.op.class() {

            OpC::I | OpC::L => {
                if self.ic.op._rs() == self.ic.op._rt() || self.ic.op._rt() == self.ic.op._rt() {
                    self.ex.stalled = true;
                    return;
                }
            }, OpC::R => {
                if self.ic.op._rs() == self.ic.op._rd() || self.ic.op._rt() == self.ic.op._rd() {
                    self.ex.stalled = true;
                    return;
                }
            } _=> {

            }
        }

        match self.ic.op.op() {
            Op::Syscall => {
                if self.ic.op.sa() > 0 {
                    let result = if self.ic.op._rt() == 16 { "Pass" }  else { "Fail" };
                    println!("Test Result - ISA:{:X}  Set:{:X}  Test:{:X}  Result:{:?}", self.ic.op._rs(), self.ic.op._rd(), self.ic.op.sa(), result);
                }
            }, _ => {
                match self.ic.op.class() {
                    OpC::L => {

                    }, OpC::C => {

                    }, OpC::B => {
                        self.ic.op.ex()(&mut self);

                        /* if a branch will occur, set the delay slot program counter */
                        if self.ex.br {
                            let offset = ((self.ic.op.offset() as i16 as i32) << 2) as i64;
                            self.ds_pc = (cpu.pc as i64 + offset) as u64;
                        }

                    }, OpC::J => {


                    }, _ => {
                        self.ic.op.ex()(&mut self);
                    }
                }
            }
        }
    }

    /* DC - Data Cache Fetch */
    pub fn dc(&mut self, mc: &MC) {

        println!("DC {}", self.ic.op);

        match self.ic.op.class() {
            OpC::L => {
                let base = self.rf.rs as i64;
                let offset = self.ic.op.offset() as i16 as i64;
                self.dc.dc = mc.read((base + offset) as u32) as u64;
                /* need to call the ex function as a hack to get ol populated */
                self.ic.op.ex()(&mut self);
            }, _ => {

            }
        }

    }

    /* WB - Write Back */
    pub fn wb(&mut self, mc: &mut MC) {

        println!("WB {}", self.ic.op);

        match self.ic.op.class() {

            /* decode instruction types I, L, S, J, B, R, C for writeback */

            OpC::I | OpC::L => {
                /* I and L instructions write back to the rt register */
                cpu.wgpr(self.ex.ol, self.ic.op._rt());
            }, OpC::S => {
                /* S instructions write back to memory */
                let base = self.rf.rs as i64;
                let offset = self.ic.op.offset() as i16 as i64;
                mc.write((base + offset) as u64 as u32, self.ex.ol as u32);
            }, OpC::J | OpC::B => {
                /* J and B instructions wrote to the delay slot program counter and link register */
                if self.ex.wlr {
                    cpu.wgpr(cpu.pc + 4, 31);
                }
            }, OpC::R => {
                /* write back to rd */
                cpu.wgpr(self.ex.ol, self.ic.op._rd());
            }, OpC::C => {
                /* write back to rt on the coprocessor */
                cpu.cp0.wgpr(self.ex.ol as u32, self.ic.op._rt());
            }
        }

        if self.ex.stalled {
            self.ex.stalled = false;
        }

    }
}
