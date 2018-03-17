const MI_REG_INIT_MODE: u32 = 0x0430_0000;
const MI_REG_VERSION: u32 = 0x0430_0004;
const MI_REG_INTR: u32 = 0x0430_0008;
const MI_REG_INTR_MASK: u32 = 0x0430_000C;

pub struct MI {
	init_mode: u32,
	version: u32,
	intr: u32,
	intr_mask: u32
}

impl MI {
	pub fn new() -> MI {
		MI {
			init_mode: 0,
			version: 0,
			intr: 0,
			intr_mask: 0
		}
	}

	/* Reads from the MI's registers. */
	pub fn rreg(&self, reg: u32) -> u32 {
		match reg {
			MI_REG_INIT_MODE => {
				self.init_mode
			}, MI_REG_VERSION => {
				self.version
			}, MI_REG_INTR => {
				self.intr
			}, MI_REG_INTR_MASK => {
				self.intr_mask
			}, _ => panic!("Read from unrecognized MI register address: {:#x}", reg)
		}
	}

	/* Writes to the MI's registers. */
	pub fn wreg(&mut self, reg: u32, value: u32) {
		match reg {
			MI_REG_INIT_MODE => {
				self.init_mode = value
			}, MI_REG_VERSION => {
				self.version = value
			}, MI_REG_INTR => {
				self.intr = value
			}, MI_REG_INTR_MASK => {
				self.intr_mask = value
			}, _ => panic!("Write to unrecognized MI register address: {:#x}", reg)
		}
	}
}
