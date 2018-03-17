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
}
