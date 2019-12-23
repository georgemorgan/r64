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
}

/* IC - Instruction Cache Fetch */
pub fn ic(cpu: &mut VR4300, mc: &MC) {

    /* if there is a branch waiting in the delay slot, we need to update the PC here */
    if cpu.pl.ds_pc != 0 {
        cpu.pc = cpu.pl.ds_pc;
        cpu.pl.ds_pc = 0;
    }

    let val = mc.read((cpu.pc) as u32);
    cpu.pl.ic.op = Inst(val);

    println!("IC {}", cpu.pl.ic.op);

    cpu.pc += 4;
}

/* RF - Register Fetch */
pub fn rf(cpu: &mut VR4300) {

    println!("RF {}", cpu.pl.ic.op);

    match cpu.pl.ic.op.class() {
        OpC::C => {
            cpu.pl.rf.rs = cpu.cp0.rgpr(cpu.pl.ic.op._rd()) as u64
        }, _ => {
            cpu.pl.rf.rs = cpu.rgpr(cpu.pl.ic.op._rs());
        }
    }

    cpu.pl.rf.rt = cpu.rgpr(cpu.pl.ic.op._rt());
}

/* EX - Execution */
pub fn ex(cpu: &mut VR4300) {

    println!("EX {}", cpu.pl.ic.op);

    /* stall if the register is the dest of the RF instruction */
    match cpu.pl.ic.op.class() {

        OpC::I | OpC::L => {
            if cpu.pl.ic.op._rs() == cpu.pl.ic.op._rt() || cpu.pl.ic.op._rt() == cpu.pl.ic.op._rt() {
                cpu.pl.ex.stalled = true;
                return;
            }
        }, OpC::R => {
            if cpu.pl.ic.op._rs() == cpu.pl.ic.op._rd() || cpu.pl.ic.op._rt() == cpu.pl.ic.op._rd() {
                cpu.pl.ex.stalled = true;
                return;
            }
        } _=> {

        }
    }

    match cpu.pl.ic.op.op() {
        Op::Syscall => {
            if cpu.pl.ic.op.sa() > 0 {
                let result = if cpu.pl.ic.op._rt() == 16 { "Pass" }  else { "Fail" };
                println!("Test Result - ISA:{:X}  Set:{:X}  Test:{:X}  Result:{:?}", cpu.pl.ic.op._rs(), cpu.pl.ic.op._rd(), cpu.pl.ic.op.sa(), result);
            }
        }, _ => {
            match cpu.pl.ic.op.class() {
                OpC::L => {

                }, OpC::C => {

                }, OpC::B => {
                    cpu.pl.ic.op.ex()(&mut cpu.pl);

                    /* if a branch will occur, set the delay slot program counter */
                    if cpu.pl.ex.br {
                        let offset = ((cpu.pl.ic.op.offset() as i16 as i32) << 2) as i64;
                        cpu.pl.ds_pc = (cpu.pc as i64 + offset) as u64;
                    }

                }, OpC::J => {


                }, _ => {
                    cpu.pl.ic.op.ex()(&mut cpu.pl);
                }
            }
        }
    }
}

/* DC - Data Cache Fetch */
pub fn dc(cpu: &mut VR4300, mc: &MC) {

    println!("DC {}", cpu.pl.ic.op);

    match cpu.pl.ic.op.class() {
        OpC::L => {
            let base = cpu.pl.rf.rs as i64;
            let offset = cpu.pl.ic.op.offset() as i16 as i64;
            cpu.pl.dc.dc = mc.read((base + offset) as u32) as u64;
            /* need to call the ex function as a hack to get ol populated */
            cpu.pl.ic.op.ex()(&mut cpu.pl);
        }, _ => {

        }
    }

}

/* WB - Write Back */
pub fn wb(cpu: &mut VR4300, mc: &mut MC) {

    println!("WB {}", cpu.pl.ic.op);

    match cpu.pl.ic.op.class() {

        /* decode instruction types I, L, S, J, B, R, C for writeback */

        OpC::I | OpC::L => {
            /* I and L instructions write back to the rt register */
            cpu.wgpr(cpu.pl.ex.ol, cpu.pl.ic.op._rt());
        }, OpC::S => {
            /* S instructions write back to memory */
            let base = cpu.pl.rf.rs as i64;
            let offset = cpu.pl.ic.op.offset() as i16 as i64;
            mc.write((base + offset) as u64 as u32, cpu.pl.ex.ol as u32);
        }, OpC::J | OpC::B => {
            /* J and B instructions wrote to the delay slot program counter and link register */
            if cpu.pl.ex.wlr {
                cpu.wgpr(cpu.pc + 4, 31);
            }
        }, OpC::R => {
            /* write back to rd */
            cpu.wgpr(cpu.pl.ex.ol, cpu.pl.ic.op._rd());
        }, OpC::C => {
            /* write back to rt on the coprocessor */
            cpu.cp0.wgpr(cpu.pl.ex.ol as u32, cpu.pl.ic.op._rt());
        }
    }

    if cpu.pl.ex.stalled {
        cpu.pl.ex.stalled = false;
    }

}

pub fn clock(cpu: &mut VR4300, mc: &mut MC) {
    ic(cpu, mc);
    rf(cpu);
    ex(cpu);
    dc(cpu, mc);
    wb(cpu, mc);
}
