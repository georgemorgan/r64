use super::*;

enum PlStage {
    IC,
    RF,
    EX,
    DC,
    WB
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
    pub ic : Ic,
    pub rf: Rf,
    pub ex: Ex,
    pub dc: Dc,
    pub wb: Wb
}

impl Pls {
    pub fn new() -> Pls {
        Pls {
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
    /* pipeline stage */
    st: [Pls; 5]
}

impl Pl {
    pub fn new() -> Pl {
        Pl {
            st: [Pls::new(); 5]
        }
    }
}

/* IC - Instruction Cache Fetch */
pub fn ic(i: usize, cpu: &mut VR4300, mc: &MC) {

    let val = mc.read(cpu.pc as u32);
    cpu.pl.st[i].ic.op = Inst(val);
    println!("IC - {:#x}: ({:#x}) {}", cpu.pc, val, cpu.pl.st[i].ic.op);
}

/* RF - Register Fetch */
pub fn rf(i: usize, cpu: &mut VR4300) {

    cpu.pc += 4;

    match cpu.pl.st[i].ic.op.class() {
        OpC::C => {
            cpu.pl.st[i].rf.rs = cpu.cp0.rgpr(cpu.pl.st[i].ic.op._rd()) as u64
        }, _ => {
            cpu.pl.st[i].rf.rs = cpu.rgpr(cpu.pl.st[i].ic.op._rs());
        }
    }

    cpu.pl.st[i].rf.rt = cpu.rgpr(cpu.pl.st[i].ic.op._rt());

    println!("RF - {}", cpu.pl.st[i].ic.op);
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

                    if cpu.pl.st[i].ex.br {
                        let offset = ((cpu.pl.st[i].ic.op.offset() as i16 as i32) << 2) as i64;
                        cpu.pl.st[i].ex.ol = (cpu.pc as i64 + offset) as u64;
                    }

                }, _ => {
                    cpu.pl.st[i].ic.op.ex()(&mut cpu.pl.st[i]);
                }
            }
        }
    }

    println!("EX - {}", cpu.pl.st[i].ic.op);
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

    println!("DC - {}", cpu.pl.st[i].ic.op);
}

/* WB - Write Back */
pub fn wb(i: usize, cpu: &mut VR4300, mc: &mut MC) {

    match cpu.pl.st[i].ic.op.class() {
        OpC::S => {
            let base = cpu.pl.st[i].rf.rs as i64;
            let offset = cpu.pl.st[i].ic.op.offset() as i16 as i64;
            mc.write((base + offset) as u64 as u32, cpu.pl.st[i].ex.ol as u32);
        }, OpC::C => {
            /* write back to rt on the coprocessor */
            cpu.cp0.wgpr(cpu.pl.st[i].ex.ol as u32, cpu.pl.st[i].ic.op._rt());
        }, OpC::B => {
            cpu.pc = cpu.pl.st[i].ex.ol;
        }, OpC::J => {
            // nop
        }, OpC::L | OpC::I => {
            /* write back to rt */
            cpu.wgpr(cpu.pl.st[i].ex.ol, cpu.pl.st[i].ic.op._rt());
        }, OpC::R => {
            /* write back to rd */
            cpu.wgpr(cpu.pl.st[i].ex.ol, cpu.pl.st[i].ic.op._rd());
        }
    }

    println!("WB - {}", cpu.pl.st[i].ic.op);
}

pub fn clock(cpu: &mut VR4300, mc: &mut MC) {
    ic(0, cpu, mc);
    rf(1, cpu);
    ex(2, cpu);
    dc(3, cpu, mc);
    wb(4, cpu, mc);

    cpu.pl.st[4] = cpu.pl.st[3];
    cpu.pl.st[3] = cpu.pl.st[2];
    cpu.pl.st[2] = cpu.pl.st[1];
    cpu.pl.st[1] = cpu.pl.st[0];
    cpu.pl.st[0] = Pls::new();
}
