use super::*;

pub struct Pipeline {

    pub gpr: [u64; GPR_SIZE],

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

}

impl Pipeline {

    pub fn new(pc: u64) -> Pipeline {
        Pipeline {

            gpr: [0; GPR_SIZE],

            pc: pc,

            op: Inst(0),

            rt: 0,
            rs: 0,

            ol: 0,
            br: false,
            wlr: false,

            dc: 0

        }
    }

}

/* IC - Instruction Cache Fetch */
pub fn ic(cpu: &mut VR4300, mc: &MC) {

    let p = &mut cpu.pipeline;

    let val = mc.read(p.pc as u32);
    p.op = Inst(val);
    println!("{:#x}: ({:#x}) {}", p.pc, val, p.op);
}

/* RF - Register Fetch */
pub fn rf(cpu: &mut VR4300) {

    let p = &mut cpu.pipeline;

    p.pc += 4;

    match p.op.class() {
        OpC::C => {
            p.rs = cpu.cp0.gpr[p.op._rd()] as u64
        }, _ => {
            p.rs = p.gpr[p.op._rs()];
        }
    }

    p.rt = p.gpr[p.op._rt()];
}

/* EX - Execution */
pub fn ex(cpu: &mut VR4300) {

    let p = &mut cpu.pipeline;

    match p.op.class() {
        OpC::L => {

        }, OpC::C => {

        }, OpC::B => {
            p.op.ex()(p);

            if p.br {
                let offset = ((p.op.offset() as i16 as i32) << 2) as i64;
                p.pc = (p.pc as i64 + offset) as u64;
            }

        }, _ => {
            p.op.ex()(p);
        }
    }
}

/* DC - Data Cache Fetch */
pub fn dc(cpu: &mut VR4300, mc: &mut MC) {

    let p = &mut cpu.pipeline;

    match p.op.class() {
        OpC::L => {
            let base = p.rs as i64;
            let offset = p.op.offset() as i16 as i64;
            p.dc = mc.read((base + offset) as u32) as u64;
            /* need to call the ex function as a hack to get ol populated */
            p.op.ex()(p);
        }, _ => {

        }
    }
}

/* WB - Write Back */
pub fn wb(cpu: &mut VR4300, mc: &mut MC) {

    let p = &mut cpu.pipeline;

    match p.op.class() {
        OpC::S => {
            let base = p.rs as i64;
            let offset = p.op.offset() as i16 as i64;
            mc.write((base + offset) as u32, p.ol as u32);
        }, OpC::C => {
            /* write back to rt on the coprocessor */
            cpu.cp0.gpr[p.op._rt()] = p.ol as u32;
        }, OpC::B => {
            // nop
        }  OpC::J => {
            if p.wlr {
                p.gpr[31] = p.pc + 8;
            }
            p.pc = p.ol;
        } _ => {
            /* write back to rt */
            p.gpr[p.op._rt()] = p.ol;
        }
    }

    /* write back the program counter */
    p.gpr[31] = p.pc;

}
