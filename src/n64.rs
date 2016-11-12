pub struct N64 {
	/* Holds the ROM data. */
	rom: Box<[u8]>
}

impl N64 {
	pub fn begin(&self) {
		println!("Starting emulation.");
		
	}
	pub fn new(r: Box<[u8]>) -> N64 {
		N64 { rom: r }
	}
}
