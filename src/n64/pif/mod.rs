use n64::mc;

pub struct PIF {
	/* The PIF's internal RAM. */
	pub pram: Box<[u8]>,
	/* The PIF's internal ROM from which the console boots. */
	pub prom: Box<[u8]>
}

impl PIF {
	pub fn new(pr: Box<[u8]>) -> PIF {
		PIF {
			pram: vec![0; mc::PIF_RAM_END - mc::PIF_RAM_START].into_boxed_slice(),
			prom: pr
		}
	}
}
