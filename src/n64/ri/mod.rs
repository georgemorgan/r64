const RI_REG_MODE: u32 = 0x0470_0000;
const RI_REG_CONFIG: u32 = 0x0470_0004;
const RI_REG_CURRENT_LOAD: u32 = 0x0470_0008;
const RI_REG_SELECT: u32 = 0x0470_000C;
const RI_REG_REFRESH: u32 = 0x0470_0010;
const RI_REG_LATENCY: u32 = 0x0470_0014;
const RI_REG_RERROR: u32 = 0x0470_0018;
const RI_REG_WERROR: u32 = 0x0470_001C;

pub struct RI {
	mode: u32,
	config: u32,
	current_load: u32,
	select: u32,
	refresh: u32,
	latency: u32,
	rerror: u32,
	werror: u32
}

impl RI {
	pub fn new() -> RI {
		RI {
			mode: 0,
			config: 0,
			current_load: 0,
			select: 0,
			refresh: 0,
			latency: 0,
			rerror: 0,
			werror: 0
		}
	}

	/* Reads from the RI's registers. */
	pub fn rreg(&self, reg: u32) -> u32 {
		match reg {
			RI_REG_MODE => {
				self.mode
			}, RI_REG_CONFIG => {
				self.config
			}, RI_REG_CURRENT_LOAD => {
				self.current_load
			}, RI_REG_SELECT => {
				self.select
			}, RI_REG_REFRESH => {
				self.refresh
			}, RI_REG_LATENCY => {
				self.latency
			}, RI_REG_RERROR => {
				self.rerror
			}, RI_REG_WERROR => {
				self.werror
			}, _ => panic!("Read from unrecognized RI register address: {:#x}", reg)
		}
	}

	/* Writes to the RI's registers. */
	pub fn wreg(&mut self, reg: u32, value: u32) {
		match reg {
			RI_REG_MODE => {
				self.mode = value
			}, RI_REG_CONFIG => {
				self.config = value
			}, RI_REG_CURRENT_LOAD => {
				self.current_load = value
			}, RI_REG_SELECT => {
				self.select = value
			}, RI_REG_REFRESH => {
				self.refresh = value
			}, RI_REG_LATENCY => {
				self.latency = value
			}, RI_REG_RERROR => {
				self.rerror = value
			}, RI_REG_WERROR => {
				self.werror = value
			}, _ => panic!("Write to unrecognized RI register address: {:#x}", reg)
		}
	}
}
