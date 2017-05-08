/* mc.rs - N64 Memory Controller */

/*
	Aggregates N64 system constants.
	Handles reading from and writing to the console's memory map.
 */

/* Access to the N64's memories. */
use n64::N64;

/* N64 memory sizes. */
pub const N64_IRAM_SIZE: usize = 0x400000;
pub const N64_ERAM_SIZE: usize = 0x400000;

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

pub const PIF_ROM_START: usize = 0x1FC0_0000;
const PIF_ROM_END: usize = 0x1FC0_07BF;

pub const PIF_RAM_START: usize = 0x1FC0_07C0;
pub const PIF_RAM_END: usize = 0x1FC0_07FF;

const RESERVED_START: usize = 0x1FC0_0800;
const RESERVED_END: usize = 0x1FCF_FFFF;

const CART_DOM1_A3_START: usize = 0x1FD0_0000;
const CART_DOM1_A3_END: usize = 0x7FFF_FFFF;

const SYSAD_START: usize = 0x8000_0000;
const SYSAD_END: usize = 0xFFFF_FFFF;

fn read32(addr: usize, mem: &Box<[u8]>) -> u32 {
	println!("Reading word from relative address {:#x}", addr);
	/* Obtain a slice starting at the read address. */
	let b: &[u8] = &mem[addr .. addr + 4];
	/* Extract each of the word's bytes and use them to create a u32. */
	let w = ((b[0] as u32) << 24) | ((b[1] as u32) << 16) | ((b[2] as u32) << 8) | b[3] as u32;
	/* Byte swap and adjust the endianness of the read word. */
	u32::from_be(w.swap_bytes())
}

/* Reads a word from the provided N64's memory map. */
pub fn read(n64: &N64, addr: usize) -> u32 {
	println!("Reading word from physical address 0x{:08x}", addr);
	/* Match the memory address to a peripheral address range. */
	match addr {
		RDRAM_MEM_START ... RDRAM_MEM_END       =>
			read32(addr - RDRAM_MEM_START, &n64.iram),
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
			read32(addr - CART_DOM2_A1_START, &n64.crom),
		CART_DOM1_A1_START ... CART_DOM1_A1_END =>
			read32(addr - CART_DOM1_A1_START, &n64.crom),
		CART_DOM2_A2_START ... CART_DOM2_A2_END =>
			read32(addr - CART_DOM2_A2_START, &n64.crom),
		CART_DOM1_A2_START ... CART_DOM1_A2_END =>
			read32(addr - CART_DOM1_A2_START, &n64.crom),
		PIF_ROM_START ... PIF_ROM_END           =>
			read32(addr - PIF_ROM_START, &n64.pif.prom),
		PIF_RAM_START ... PIF_RAM_END           =>
			read32(addr - PIF_RAM_START, &n64.pif.pram),
		RESERVED_START ... RESERVED_END         => 0,
		CART_DOM1_A3_START ... CART_DOM1_A3_END =>
			read32(addr - CART_DOM1_A3_START, &n64.crom),
		SYSAD_START ... SYSAD_END               => 0,
		_ => panic!("Unrecognized physical address: {:#x}", addr)
	}
}

use std::io::Write;

fn write32(val: u32, addr: usize, mem: &mut Box<[u8]>) {
	println!("Writing word to relative address {:#x}", addr);
	/* Obtain a slice of bytes from the u32. */
	let from: &[u8] = &[(val >> 24) as u8, (val >> 16) as u8, (val >> 8) as u8, val as u8];
	/* Write the slice into memory. */
	mem[addr .. addr + 4].copy_from_slice(from);
}

/* Writes a word to the provided N64's memory map. */
pub fn write(n64: &mut N64, val: u32, addr: usize) {
	println!("Reading word from physical address 0x{:08x}", addr);
	/* Match the memory address to a peripheral address range. */
	match addr {
		RDRAM_MEM_START ... RDRAM_MEM_END       =>
			write32(val, addr - RDRAM_MEM_START, &mut n64.iram),
		RDRAM_REG_START ... RDRAM_REG_END       =>
			return,
		SP_REG_START ... SP_REG_END             =>
			return,
		DP_CMD_START ... DP_CMD_END             =>
			return,
		DP_SPAN_START ... DP_SPAN_END           =>
			return,
		MI_REG_START ... MI_REG_END             =>
			return,
		VI_REG_START ... VI_REG_END             =>
			return,
		AI_REG_START ... AI_REG_END             =>
			return,
		PI_REG_START ... PI_REG_END             =>
			return,
		RI_REG_START ... RI_REG_END             =>
			return,
		SI_REG_START ... SI_REG_END             =>
			return,
		UNUSED_START ... UNUSED_END             =>
			return,
		CART_DOM2_A1_START ... CART_DOM2_A1_END =>
			panic!("Attempt to write to a read-only location {:#x}.", addr),
		CART_DOM1_A1_START ... CART_DOM1_A1_END =>
			panic!("Attempt to write to a read-only location {:#x}.", addr),
		CART_DOM2_A2_START ... CART_DOM2_A2_END =>
			panic!("Attempt to write to a read-only location {:#x}.", addr),
		CART_DOM1_A2_START ... CART_DOM1_A2_END =>
			panic!("Attempt to write to a read-only location {:#x}.", addr),
		PIF_ROM_START ... PIF_ROM_END           =>
			panic!("Attempt to write to a read-only location {:#x}.", addr),
		PIF_RAM_START ... PIF_RAM_END           =>
			write32(val, addr - PIF_RAM_START, &mut n64.pif.pram),
		RESERVED_START ... RESERVED_END         =>
			return,
		CART_DOM1_A3_START ... CART_DOM1_A3_END =>
			panic!("Attempt to write to a read-only location {:#x}.", addr),
		SYSAD_START ... SYSAD_END               =>
			return,
		_ => panic!("Unrecognized physical address: {:#x}", addr)
	}
}
