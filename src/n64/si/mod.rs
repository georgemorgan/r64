const SI_REG_DRAM_ADDR: u32 = 0x0480_0000;
const SI_REG_PIF_ADDR_RD64B: u32 = 0x0480_0004;
const SI_REG_PIF_ADDR_WR64B: u32 = 0x0480_0010;
const SI_REG_STATUS: u32 = 0x0480_0018;

pub struct SI {
	dram_addr: u32,
	pif_addr_rd64b: u32,
	pif_addr_wr64b: u32,
	status: u32
}

impl SI {
	pub fn new() -> SI {
		SI {
			dram_addr: 0,
			pif_addr_rd64b: 0,
			pif_addr_wr64b: 0,
			status: 0
		}
	}
	/* Reads from the SI's registers. */
	pub fn rreg(&self, reg: u32) -> u32 {
		match reg {
			SI_REG_DRAM_ADDR => {
				self.dram_addr
			}, SI_REG_PIF_ADDR_RD64B => {
				self.pif_addr_rd64b
			}, SI_REG_PIF_ADDR_WR64B => {
				self.pif_addr_wr64b
			}, SI_REG_STATUS => {
				self.status
			}, _ => panic!("Read from unrecognized SI register address: {:#x}", reg)
		}
	}

	/* Writes to the SI's registers. */
	pub fn wreg(&mut self, reg: u32, value: u32) {
		match reg {
			SI_REG_DRAM_ADDR => {
				self.dram_addr = value
			}, SI_REG_PIF_ADDR_RD64B => {
				self.pif_addr_rd64b = value
			}, SI_REG_PIF_ADDR_WR64B => {
				self.pif_addr_wr64b = value
			}, SI_REG_STATUS => {
				self.status = value
			}, _ => panic!("Write to unrecognized SI register address: {:#x}", reg)
		}
	}
}
