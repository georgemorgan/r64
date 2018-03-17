pub struct RDP {

}

impl RDP {
	pub fn new() -> RDP {
		RDP { }
	}
	
	/* Reads from the PIF's registers. */
	pub fn rreg(&self, reg: u32) -> u32 {
		match reg {
			_ => panic!("Read from unrecognized PIF register address: {:#x}", reg)
		}
	}

	/* Writes to the PIF's registers. */
	pub fn wreg(&mut self, reg: u32, value: u32) {
		match reg {
			_ => panic!("Write to unrecognized PIF register address: {:#x}", reg)
		}
	}
}
