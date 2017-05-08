/* mc.rs - N64 Memory Controller */

/* Handles reading from and writing to the console's memory map. */

/*
    0x0000 0000 to 0x03EF FFFF RDRAM Memory
    0x03F0 0000 to 0x03FF FFFF RDRAM Registers
    0x0400 0000 to 0x040F FFFF SP Registers
    0x0410 0000 to 0x041F FFFF DP Command Registers
    0x0420 0000 to 0x042F FFFF DP Span Registers
    0x0430 0000 to 0x043F FFFF MIPS Interface (MI) Registers
    0x0440 0000 to 0x044F FFFF Video Interface (VI) Registers
    0x0450 0000 to 0x045F FFFF Audio Interface (AI) Registers
    0x0460 0000 to 0x046F FFFF Peripheral Interface (PI) Registers
    0x0470 0000 to 0x047F FFFF RDRAM Interface (RI) Registers
    0x0480 0000 to 0x048F FFFF Serial Interface (SI) Registers
    0x0490 0000 to 0x04FF FFFF Unused
    0x0500 0000 to 0x05FF FFFF Cartridge Domain 2 Address 1
    0x0600 0000 to 0x07FF FFFF Cartridge Domain 1 Address 1
    0x0800 0000 to 0x0FFF FFFF Cartridge Domain 2 Address 2
    0x1000 0000 to 0x1FBF FFFF Cartridge Domain 1 Address 2
    0x1FC0 0000 to 0x1FC0 07BF PIF Boot ROM
    0x1FC0 07C0 to 0x1FC0 07FF PIF RAM
    0x1FC0 0800 to 0x1FCF FFFF Reserved
    0x1FD0 0000 to 0x7FFF FFFF Cartridge Domain 1 Address 3
    0x8000 0000 to 0xFFFF FFFF External SysAD Device
*/

const RDRAM_MEM_START: usize = 0x0000_0000;
const RDRAM_MEM_END: usize = 0x003E_FFFF;

const RDRAM_REG_START: usize = 0x03F0_0000;
const RDRAM_REG_END: usize = 0x03FF_FFFF;

const SP_REG_START: usize = 0x0400_0000;
const SP_REG_END: usize = 0x040F_FFFF;

const DP_CMD_START: usize = 0x0410_0000;
const DP_CMD_END: usize = 0x041F_FFFF;

const DP_SPAN_START: usize = 0x0420_0000;
const DP_SPAN_END: usize = 0x042F_FFFF;

const MI_REG_START: usize = 0x0430_0000;
const MI_REG_END: usize = 0x043F_FFFF;

const VI_REG_START: usize = 0x0440_0000;
const VI_REG_END: usize = 0x044F_FFFF;

const AI_REG_START: usize = 0x0450_0000;
const AI_REG_END: usize = 0x045F_FFFF;

const PI_REG_START: usize = 0x0460_0000;
const PI_REG_END: usize = 0x046F_FFFF;

const RI_REG_START: usize = 0x0470_0000;
const RI_REG_END: usize = 0x047F_FFFF;

const SI_REG_START: usize = 0x0480_0000;
const SI_REG_END: usize = 0x048F_FFFF;

const UNUSED_START: usize = 0x0490_0000;
const UNUSED_END: usize = 0x04FF_FFFF;

const CART_DOM2_A1_START: usize = 0x0500_0000;
const CART_DOM2_A1_END: usize = 0x05FF_FFFF;

const CART_DOM1_A1_START: usize = 0x0600_0000;
const CART_DOM1_A1_END: usize = 0x07FF_FFFF;

const CART_DOM2_A2_START: usize = 0x0800_0000;
const CART_DOM2_A2_END: usize = 0x0FFF_FFFF;

const CART_DOM1_A2_START: usize = 0x1000_0000;
const CART_DOM1_A2_END: usize = 0x1FBF_FFFF;

const PIF_ROM_START: usize = 0x1FC0_0000;
const PIF_ROM_END: usize = 0x1FC0_07BF;

const PIF_RAM_START: usize = 0x1FC0_07C0;
const PIF_RAM_END: usize = 0x1FC0_07FF;

const RESERVED_START: usize = 0x1FC0_0800;
const RESERVED_END: usize = 0x1FCF_FFFF;

const CART_DOM1_A3_START: usize = 0x1FD0_0000;
const CART_DOM1_A3_END: usize = 0x7FFF_FFFF;

const SYSAD_START: usize = 0x8000_0000;
const SYSAD_END: usize = 0xFFFF_FFFF;

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
	crom: Box<[u8]>
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
    pub fn read(&self, addr: usize) -> u32 {
        println!("Reading word from physical address {:#x}", addr);

		match addr {
			RDRAM_MEM_START ... RDRAM_MEM_END       => 0,
			RDRAM_REG_START ... RDRAM_REG_END       => 0,
			SP_REG_START ... SP_REG_END             => 0,
			DP_CMD_START ... DP_CMD_END             => 0,
			DP_SPAN_START ... DP_SPAN_END           => 0,
			MI_REG_START ... MI_REG_END             => 0,
			VI_REG_START ... VI_REG_END             => 0,
			AI_REG_START ... AI_REG_END             => 0,
			PI_REG_START ... PI_REG_END             => 0,
			RI_REG_START ... RI_REG_END             => 0,
			SI_REG_START ... SI_REG_END             => 0,
			UNUSED_START ... UNUSED_END             => 0,
			CART_DOM2_A1_START ... CART_DOM2_A1_END =>
				read32(addr - CART_DOM2_A1_START, &self.crom),
			CART_DOM1_A1_START ... CART_DOM1_A1_END =>
				read32(addr - CART_DOM1_A1_START, &self.crom),
			CART_DOM2_A2_START ... CART_DOM2_A2_END =>
				read32(addr - CART_DOM2_A2_START, &self.crom),
			CART_DOM1_A2_START ... CART_DOM1_A2_END =>
				read32(addr - CART_DOM1_A2_START, &self.crom),
			PIF_ROM_START ... PIF_ROM_END           => 0,
			PIF_RAM_START ... PIF_RAM_END           => 0,
			RESERVED_START ... RESERVED_END         => 0,
			CART_DOM1_A3_START ... CART_DOM1_A3_END =>
				read32(addr - CART_DOM1_A3_START, &self.crom),
			SYSAD_START ... SYSAD_END               => 0,
			_ => panic!("Unrecognized physical address: {:#x}", addr)
		}
    }

    /* Writes a word to the memory map. */
    pub fn write(&mut self, val: u32) {
        self.iram[0] = val as u8
    }
}

fn read32(addr: usize, mem: &Box<[u8]>) -> u32 {
	println!("Reading word from relative address {:#x}", addr);
	/* Obtain a slice starting at the read address. */
	let b: &[u8] = &mem[addr .. addr + 4];
	/* Extract each of the word's bytes and use them to create a u32. */
	(((b[0] as u32) << 24) | ((b[1] as u32) << 16) | ((b[2] as u32) << 8) | (b[3] as u32))
}
