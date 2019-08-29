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
pub fn ic(i: usize, cpu: &mut VR4300, mc: &MC) {

    /* if there is a branch waiting in the delay slot, we need to update the PC here */
    if cpu.pl.ds_pc != 0 {
        cpu.pc = cpu.pl.ds_pc;
        cpu.pl.ds_pc = 0;
    }

    let val = mc.read((cpu.pc + (4*i as u64)) as u32);
    cpu.pl.st[i].ic.op = Inst(val);

    /* go to the rf stage */
    cpu.pl.st[i].st = 1;
}

/* RF - Register Fetch */
pub fn rf(i: usize, cpu: &mut VR4300) {

    // match cpu.pl.st[i - 1].ic.op.class() {
    //     OpC::L | OpC::S => {
    //         /* interlock until the load delay slot is written back */
    //         if cpu.pl.st[i - 1].rf.rt == cpu.pl.st[i].rf.rt {
    //             cpu.pl.st[i].interlock = true;
    //             return;
    //         }
    //     }, _ => {
    //
    //     }
    // }

    cpu.pc += 4;

    match cpu.pl.st[i].ic.op.class() {
        OpC::C => {
            cpu.pl.st[i].rf.rs = cpu.cp0.rgpr(cpu.pl.st[i].ic.op._rd()) as u64
        }, _ => {
            cpu.pl.st[i].rf.rs = cpu.rgpr(cpu.pl.st[i].ic.op._rs());
        }
    }

    cpu.pl.st[i].rf.rt = cpu.rgpr(cpu.pl.st[i].ic.op._rt());

    /* go to the ex stage */
    cpu.pl.st[i].st = 2;
}

/* EX - Execution */
pub fn ex(i: usize, cpu: &mut VR4300) {

    match cpu.pl.st[i].ic.op.op() {
        Op::Syscall => {
            if cpu.pl.st[i].ic.op.sa() > 0 {
                let result = if cpu.pl.st[i].ic.op._rt() == 16 { "Pass" }  else { "Fail" };
                println!("Test Result - ISA:{:X}  Set:{:X}  Test:{:X}  Result:{:?}", cpu.pl.st[i].ic.op._rs(), cpu.pl.st[i].ic.op._rd(), cpu.pl.st[i].ic.op.sa(), result);
            }
        }, _ => {
            match cpu.pl.st[i].ic.op.class() {
                OpC::L => {

                }, OpC::C => {

                }, OpC::B => {
                    cpu.pl.st[i].ic.op.ex()(&mut cpu.pl.st[i]);

                    /* if a branch will occur, set the delay slot program counter */
                    if cpu.pl.st[i].ex.br {
                        let offset = ((cpu.pl.st[i].ic.op.offset() as i16 as i32) << 2) as i64;
                        cpu.pl.ds_pc = (cpu.pc as i64 + offset) as u64;
                    }

                }, OpC::J => {


                }, _ => {
                    cpu.pl.st[i].ic.op.ex()(&mut cpu.pl.st[i]);
                }
            }
        }
    }

    /* go to the dc stage */
    cpu.pl.st[i].st = 3;
}

/* DC - Data Cache Fetch */
pub fn dc(i: usize, cpu: &mut VR4300, mc: &MC) {

    match cpu.pl.st[i].ic.op.class() {
        OpC::L => {
            let base = cpu.pl.st[i].rf.rs as i64;
            let offset = cpu.pl.st[i].ic.op.offset() as i16 as i64;
            cpu.pl.st[i].dc.dc = mc.read((base + offset) as u32) as u64;
            /* need to call the ex function as a hack to get ol populated */
            cpu.pl.st[i].ic.op.ex()(&mut cpu.pl.st[i]);
        }, _ => {

        }
    }

    /* go to the wb stage */
    cpu.pl.st[i].st = 4;
}

/* WB - Write Back */
pub fn wb(i: usize, cpu: &mut VR4300, mc: &mut MC) {

    match cpu.pl.st[i].ic.op.class() {

        /* decode instruction types I, L, S, J, B, R, C for writeback */

        OpC::I | OpC::L => {
            /* I and L instructions write back to the rt register */
            cpu.wgpr(cpu.pl.st[i].ex.ol, cpu.pl.st[i].ic.op._rt());
        }, OpC::S => {
            /* S instructions write back to memory */
            let base = cpu.pl.st[i].rf.rs as i64;
            let offset = cpu.pl.st[i].ic.op.offset() as i16 as i64;
            mc.write((base + offset) as u64 as u32, cpu.pl.st[i].ex.ol as u32);
        }, OpC::J | OpC::B => {
            /* J and B instructions wrote to the delay slot program counter and link register */
            if cpu.pl.st[i].ex.wlr {
                cpu.wgpr(cpu.pc + 8, 31);
            }
        }, OpC::R => {
            /* write back to rd */
            cpu.wgpr(cpu.pl.st[i].ex.ol, cpu.pl.st[i].ic.op._rd());
        }, OpC::C => {
            /* write back to rt on the coprocessor */
            cpu.cp0.wgpr(cpu.pl.st[i].ex.ol as u32, cpu.pl.st[i].ic.op._rt());
        }
    }

    /* go to the ic stage */
    cpu.pl.st[i].st = 0;
}

pub fn clock(cpu: &mut VR4300, mc: &mut MC) {

    println!("{:?}", cpu.pl);

    /* run 5 pcycles per clock */
    for i in 0..5 {
        match cpu.pl.st[i].st {
            0 => {
                ic(i, cpu, mc);
            }, 1 => {
                rf(i, cpu);
            }, 2 => {
                ex(i, cpu);
            }, 3 => {
                dc(i, cpu, mc);
            }, 4 => {
                wb(i, cpu, mc);
            } _ => {
                panic!("invalid pipeline stage");
            }
        }
    }

    cpu.pl.pcycle += 5;
}

impl fmt::Debug for Pls {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self.st {
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

        Ok(())

    }
}

impl fmt::Debug for Pl {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for i in 0..5 {
            try!(writeln!(f, "{}: ({:?}) - {}", i, self.st[i], self.st[i].ic.op));
        }

        Ok(())

    }
}
