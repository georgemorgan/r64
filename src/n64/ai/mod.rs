const AI_REG_DRAM_ADDR: u32 = 0x0450_0000;
const AI_REG_LEN: u32 = 0x0450_0004;
const AI_REG_CONTROL: u32 = 0x0450_0008;
const AI_REG_STATUS: u32 = 0x0450_000C;
const AI_REG_DACRATE: u32 = 0x0450_0010;
const AI_REG_BITRATE: u32 = 0x0450_0014;

pub struct AI {
	dram_addr: u32,
	len: u32,
	control: u32,
	status: u32,
	dacrate: u32,
	bitrate: u32
}

impl AI {
	pub fn new() -> AI {
		AI {
			dram_addr: 0,
			len: 0,
			control: 0,
			status: 0,
			dacrate: 0,
			bitrate: 0
		}
	}

	/* Reads from the AI's registers. */
	pub fn rreg(&self, reg: u32) -> u32 {
		match reg {
			AI_REG_DRAM_ADDR => {
				self.dram_addr
			}, AI_REG_LEN => {
				self.len
			}, AI_REG_CONTROL => {
				self.control
			}, AI_REG_STATUS => {
				self.status
			}, AI_REG_DACRATE => {
				self.dacrate
			}, AI_REG_BITRATE => {
				self.bitrate
			}, _ => panic!("Read from unrecognized AI register address: {:#x}", reg)
		}
	}

	/* Writes to the AI's registers. */
	pub fn wreg(&mut self, reg: u32, value: u32) {
		match reg {
			AI_REG_DRAM_ADDR => {
				self.dram_addr = value
			}, AI_REG_LEN => {
				self.len = value
			}, AI_REG_CONTROL => {
				self.control = value
			}, AI_REG_STATUS => {
				self.status = value
			}, AI_REG_DACRATE => {
				self.dacrate = value
			}, AI_REG_BITRATE => {
				self.bitrate = value
			}, _ => panic!("Write to unrecognized AI register address: {:#x}", reg)
		}
	}
}
