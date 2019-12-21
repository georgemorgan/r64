use super::*;

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

#[derive(Copy, Clone)]
pub struct Pls {
    pub st: usize,
    pub interlock: bool,
    pub ic : Ic,
    pub rf: Rf,
    pub ex: Ex,
    pub dc: Dc,
    pub wb: Wb
}

impl Pls {
    pub fn new() -> Pls {
        Pls {

            st: 0,

            interlock: false,

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

pub struct Pl {
    /* pipeline cycle */
    pcycle: usize,
    /* pipeline stages */
    st: [Pls; 5],
    /* delay slot program counter */
    ds_pc: u64
}

impl Pl {
    pub fn new() -> Pl {
        let mut pl = Pl {
            pcycle: 0,
            st: [Pls::new(); 5],
            ds_pc: 0
        };

        pl.st[0].st = 4; // wb
        pl.st[1].st = 3; // dc
        pl.st[2].st = 2; // ex
        pl.st[3].st = 1; // rf
        pl.st[4].st = 0; // ic

        pl
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
    cpu.pl.st[4].ic.op = Inst(val);

    println!("IC {}", cpu.pl.st[4].ic.op);

    cpu.pl.st[3] = cpu.pl.st[4]; // IC -> RF
}

/* RF - Register Fetch */
pub fn rf(cpu: &mut VR4300) {

    println!("RF {}", cpu.pl.st[3].ic.op);

    cpu.pc += 4;

    match cpu.pl.st[3].ic.op.class() {
        OpC::C => {
            cpu.pl.st[3].rf.rs = cpu.cp0.rgpr(cpu.pl.st[3].ic.op._rd()) as u64
        }, _ => {
            cpu.pl.st[3].rf.rs = cpu.rgpr(cpu.pl.st[3].ic.op._rs());
        }
    }

    cpu.pl.st[3].rf.rt = cpu.rgpr(cpu.pl.st[3].ic.op._rt());

    cpu.pl.st[2] = cpu.pl.st[3]; // RF -> EX
}

/* EX - Execution */
pub fn ex(cpu: &mut VR4300) {

    println!("EX {}", cpu.pl.st[2].ic.op);

    /* stall if the register is the dest of the RF instruction */
    match cpu.pl.st[1].ic.op.class() {

        OpC::I | OpC::L => {
            if cpu.pl.st[2].ic.op._rs() == cpu.pl.st[3].ic.op._rt() || cpu.pl.st[2].ic.op._rt() == cpu.pl.st[3].ic.op._rt() {
                cpu.pl.st[2].ex.stalled = true;
                cpu.pl.st[1] = Pls::new();
                return;
            }
        }, OpC::R => {
            if cpu.pl.st[2].ic.op._rs() == cpu.pl.st[3].ic.op._rd() || cpu.pl.st[2].ic.op._rt() == cpu.pl.st[3].ic.op._rd() {
                cpu.pl.st[2].ex.stalled = true;
                cpu.pl.st[1] = Pls::new();
                return;
            }
        } _=> {

        }
    }

    match cpu.pl.st[2].ic.op.op() {
        Op::Syscall => {
            if cpu.pl.st[2].ic.op.sa() > 0 {
                let result = if cpu.pl.st[2].ic.op._rt() == 16 { "Pass" }  else { "Fail" };
                println!("Test Result - ISA:{:X}  Set:{:X}  Test:{:X}  Result:{:?}", cpu.pl.st[2].ic.op._rs(), cpu.pl.st[2].ic.op._rd(), cpu.pl.st[2].ic.op.sa(), result);
            }
        }, _ => {
            match cpu.pl.st[2].ic.op.class() {
                OpC::L => {

                }, OpC::C => {

                }, OpC::B => {
                    cpu.pl.st[2].ic.op.ex()(&mut cpu.pl.st[2]);

                    /* if a branch will occur, set the delay slot program counter */
                    if cpu.pl.st[2].ex.br {
                        let offset = ((cpu.pl.st[2].ic.op.offset() as i16 as i32) << 2) as i64;
                        cpu.pl.ds_pc = (cpu.pc as i64 + offset) as u64;
                    }

                }, OpC::J => {


                }, _ => {
                    cpu.pl.st[2].ic.op.ex()(&mut cpu.pl.st[2]);
                }
            }
        }
    }

    cpu.pl.st[1] = cpu.pl.st[2];
}

/* DC - Data Cache Fetch */
pub fn dc(cpu: &mut VR4300, mc: &MC) {

    println!("DC {}", cpu.pl.st[1].ic.op);

    match cpu.pl.st[1].ic.op.class() {
        OpC::L => {
            let base = cpu.pl.st[1].rf.rs as i64;
            let offset = cpu.pl.st[1].ic.op.offset() as i16 as i64;
            cpu.pl.st[1].dc.dc = mc.read((base + offset) as u32) as u64;
            /* need to call the ex function as a hack to get ol populated */
            cpu.pl.st[1].ic.op.ex()(&mut cpu.pl.st[1]);
        }, _ => {

        }
    }

    cpu.pl.st[0] = cpu.pl.st[1];
}

/* WB - Write Back */
pub fn wb(cpu: &mut VR4300, mc: &mut MC) {

    println!("WB {}", cpu.pl.st[0].ic.op);

    match cpu.pl.st[0].ic.op.class() {

        /* decode instruction types I, L, S, J, B, R, C for writeback */

        OpC::I | OpC::L => {
            /* I and L instructions write back to the rt register */
            cpu.wgpr(cpu.pl.st[0].ex.ol, cpu.pl.st[0].ic.op._rt());
        }, OpC::S => {
            /* S instructions write back to memory */
            let base = cpu.pl.st[0].rf.rs as i64;
            let offset = cpu.pl.st[0].ic.op.offset() as i16 as i64;
            mc.write((base + offset) as u64 as u32, cpu.pl.st[0].ex.ol as u32);
        }, OpC::J | OpC::B => {
            /* J and B instructions wrote to the delay slot program counter and link register */
            if cpu.pl.st[0].ex.wlr {
                cpu.wgpr(cpu.pc + 8, 31);
            }
        }, OpC::R => {
            /* write back to rd */
            cpu.wgpr(cpu.pl.st[0].ex.ol, cpu.pl.st[0].ic.op._rd());
        }, OpC::C => {
            /* write back to rt on the coprocessor */
            cpu.cp0.wgpr(cpu.pl.st[0].ex.ol as u32, cpu.pl.st[0].ic.op._rt());
        }
    }

    if cpu.pl.st[2].ex.stalled {
        cpu.pl.st[2].ex.stalled = false;
    }

    cpu.pl.st[0] = Pls::new();
}

pub fn clock(cpu: &mut VR4300, mc: &mut MC) {

    wb(cpu, mc);
    dc(cpu, mc);
    ex(cpu);
    rf(cpu);
    ic(cpu, mc);

}

impl fmt::Debug for Pls {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {



        Ok(())

    }
}

impl fmt::Debug for Pl {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut i = 1;

        for j in 0..5 {

            try!(write!(f, "{0: <10}", self.st[j].ic.op));

            for k in 0..5 {

                match k {
                    0 => {
                        try!(write!(f, "IC"));
                    }, 1 => {
                        try!(write!(f, "RF"));
                    }, 2 => {
                        try!(write!(f, "EX"));
                    }, 3 => {
                        try!(write!(f, "DC"));
                    }, 4 => {
                        try!(write!(f, "WB"));
                    }, _ => {
                        panic!("invalid stage");
                    }
                }

                try!(write!(f, " "));
            }

            try!(write!(f, "\n"));

            i += 1;
        }

        Ok(())

    }
}
