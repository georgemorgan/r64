use n64::mc::PIF_RAM_START;
use n64::mc::PIF_RAM_END;

pub struct PIF {
	/* PIF memories. */

	/* The PIF's internal RAM. */
	pub pram: Box<[u8]>,
	/* The PIF's internal ROM from which the console boots. */
	pub prom: Box<[u8]>
}

impl PIF {
	pub fn new(pr: Box<[u8]>) -> PIF {
		PIF {
			/* Allocate the PRAM. */
			pram: vec![0; (PIF_RAM_END - PIF_RAM_START) as usize].into_boxed_slice(),
			/* Transfer ownership of the PROM. */
			prom: pr
		}
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
