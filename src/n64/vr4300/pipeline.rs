use super::*;

pub struct Pipeline {

    pc: u64,

    /* IC */
    pub op: Inst,

    /* RF */
    pub rt: u64,
    pub rs: u64,

    /* EX */
    pub ol: u64,
    pub br: bool,
    pub wlr: bool,

    /* DC */
    pub dc: u64,

    /* delay slot */
    pub ds: bool,
    dswlr: bool,
    dsol: u64

}

impl Pipeline {

    pub fn new(pc: u64) -> Pipeline {
        Pipeline {

            pc: pc,

            op: Inst(0),

            rt: 0,
            rs: 0,

            ol: 0,
            br: false,
            wlr: false,

            dc: 0,

            ds: false,
            dswlr: false,
            dsol: 0,

        }
    }

}

/* IC - Instruction Cache Fetch */
pub fn ic(cpu: &mut VR4300, mc: &MC) {

    let val = mc.read(cpu.pipeline.pc as u32);
    cpu.pipeline.op = Inst(val);
    //println!("{:#x}: ({:#x}) {}", cpu.pipeline.pc, val, cpu.pipeline.op);
}

/* RF - Register Fetch */
pub fn rf(cpu: &mut VR4300) {

    cpu.pipeline.pc += 4;

    match cpu.pipeline.op.class() {
        OpC::C => {
            cpu.pipeline.rs = cpu.cp0.rgpr(cpu.pipeline.op._rd()) as u64
        }, _ => {
            cpu.pipeline.rs = cpu.rgpr(cpu.pipeline.op._rs());
        }
    }

    cpu.pipeline.rt = cpu.rgpr(cpu.pipeline.op._rt());
}

/* EX - Execution */
pub fn ex(cpu: &mut VR4300) {

    match cpu.pipeline.op.op() {
        Op::Syscall => {
            if cpu.pipeline.op.sa() > 0 {
                let result = if cpu.pipeline.op._rt() == 16 { "Pass" }  else { "Fail" };
                println!("Test Result - ISA:{:X}  Set:{:X}  Test:{:X}  Result:{:?}", cpu.pipeline.op._rs(), cpu.pipeline.op._rd(), cpu.pipeline.op.sa(), result);
            }
        }, _ => {
            match cpu.pipeline.op.class() {
                OpC::L => {

                }, OpC::C => {

                }, OpC::B => {
                    cpu.pipeline.op.ex()(&mut cpu.pipeline);

                    if cpu.pipeline.br {
                        let offset = ((cpu.pipeline.op.offset() as i16 as i32) << 2) as i64;
                        cpu.pipeline.ol = (cpu.pipeline.pc as i64 + offset) as u64;
                    }

                }, _ => {
                    cpu.pipeline.op.ex()(&mut cpu.pipeline);
                }
            }
        }
    }
}

/* DC - Data Cache Fetch */
pub fn dc(cpu: &mut VR4300, mc: &mut MC) {

    match cpu.pipeline.op.class() {
        OpC::L => {
            let base = cpu.pipeline.rs as i64;
            let offset = cpu.pipeline.op.offset() as i16 as i64;
            cpu.pipeline.dc = mc.read((base + offset) as u32) as u64;
            /* need to call the ex function as a hack to get ol populated */
            cpu.pipeline.op.ex()(&mut cpu.pipeline);
        }, _ => {

        }
    }
}

/* WB - Write Back */
pub fn wb(cpu: &mut VR4300, mc: &mut MC) {

    if cpu.pipeline.dsol > 0 {
        if cpu.pipeline.dswlr {
            cpu.wgpr(cpu.pipeline.pc, 31);
        }
        cpu.pipeline.pc = cpu.pipeline.dsol;

        cpu.pipeline.dswlr = false;
        cpu.pipeline.dsol = 0;
    }

    if cpu.pipeline.ds {
        cpu.pipeline.dswlr = cpu.pipeline.wlr;
        cpu.pipeline.dsol = cpu.pipeline.ol;
    }

    match cpu.pipeline.op.class() {
        OpC::S => {
            let base = cpu.pipeline.rs as i64;
            let offset = cpu.pipeline.op.offset() as i16 as i64;
            mc.write((base + offset) as u64 as u32, cpu.pipeline.ol as u32);
        }, OpC::C => {
            /* write back to rt on the coprocessor */
            cpu.cp0.wgpr(cpu.pipeline.ol as u32, cpu.pipeline.op._rt());
        }, OpC::B => {
            /* invalidate the instruction in the delay slot if the branch is not taken */
            if !cpu.pipeline.br {
                cpu.pipeline.dswlr = false;
                cpu.pipeline.dsol = 0;
            }
            else if !cpu.pipeline.ds
            {
                cpu.pipeline.pc = cpu.pipeline.ol;
            }
        }, OpC::J => {
            // nop
        }, OpC::L | OpC::I => {
            /* write back to rt */
            cpu.wgpr(cpu.pipeline.ol, cpu.pipeline.op._rt());
        }, OpC::R => {
            /* write back to rd */
            cpu.wgpr(cpu.pipeline.ol, cpu.pipeline.op._rd());
        }
    }

    cpu.pipeline.ds = false;
    cpu.pipeline.wlr = false;

}
