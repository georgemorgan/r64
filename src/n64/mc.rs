/* mc.rs - N64 Memory Controller */

/* Handles reading from and writing to the console's memory map. */

/* Access to the N64's cartridge memory. */
use n64::N64;
/* Access to the N64's PIF memory. */
use n64::pif;

/* N64 hardware constants. */
const N64_IRAM_SIZE: usize = 0x400000;
const N64_ERAM_SIZE: usize = 0x400000;

pub struct MC {
    /* ~ System memories. ~ */

	/* 4MB internal RDRAM. */
	iram: Box<[u8]>,
	/* 4MB expansion RDRAM. */
	eram: Box<[u8]>,
	/* Cartridge ROM. */
	crom: Box<[u8]>,
}

impl MC {

    pub fn new(cr: Box<[u8]>) -> MC {
        MC {
            /* Allocate the IRAM. */
            iram: vec![0; N64_IRAM_SIZE].into_boxed_slice(),
            /* Allocate the ERAM. */
            eram: vec![0; N64_ERAM_SIZE].into_boxed_slice(),
            /* Transfer ownership of the CROM. */
            crom: cr,
        }
    }

    /* Reads a word from the memory map. */
    pub fn read(addr: u32) -> u32 {
        0
    }

    /* Writes a word to the memory map. */
    pub fn write(val: u32) {

    }
}
