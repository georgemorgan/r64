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

            dc: 0

        }
    }

    /* IC - Instruction Cache Fetch */
    pub fn ic(&mut self, mc: &MC) {
        let val = mc.read(self.pc as u32);
        self.op = Inst(val);
        println!("{:#x}: ({:#x}) {}", self.pc, val, self.op);
    }

    /* RF - Register Fetch */
    pub fn rf(&mut self) {
        self.pc += 4;
        self.rt = self.gpr[self.op._rt()];
        self.rs = self.gpr[self.op._rs()];
    }

    /* EX - Execution */
    pub fn ex(&mut self) {
        match self.op.class() {
            OpC::L => {

            }, _ => {
                self.op.ex()(self);
            }
        }
    }

    /* DC - Data Cache Fetch */
    pub fn dc(&mut self, mc: &mut MC) {
        match self.op.class() {
            OpC::L => {
                self.dc = mc.read(self.ol as u32) as u64;
                /* need to call the ex function as a hack to get ol populated */
                self.op.ex()(self);
            }, OpC::S => {

            }, _ => {

            }
        }
    }

    /* WB - Write Back */
    pub fn wb(&mut self, mc: &mut MC) {

        match self.op.class() {
            OpC::L => {
                self.dc = mc.read(self.ol as u32) as u64;
                /* need to call the ex function as a hack to get ol populated */
                self.op.ex()(self);
            }, OpC::S => {

            }, _ => {
                /* write back to rt */
                self.gpr[self.op._rt()] = self.ol;
                /* write back the program counter */
                self.gpr[31] = self.pc;
            }
        }

    }

}
