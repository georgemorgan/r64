/* n64.rs - A top-level module that concatenates the sub-modules of the emulator. */

mod vi;
use self::vi::VI;
mod ai;
use self::ai::AI;
mod si;
use self::si::SI;
mod pi;
use self::pi::PI;
mod rsp;
use self::rsp::RSP;
mod rdp;
use self::rdp::RDP;
mod pif;
use self::pif::PIF;
pub mod cpu;
use self::cpu::CPU;

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

/* RDRAM memory. */
const RDRAM_MEM_START:		u32 = 0x0000_0000;
const RDRAM_MEM_END:		u32 = 0x003E_FFFF;

/* RDRAM registers. */
const RDRAM_REG_START:		u32 = 0x03F0_0000;
const RDRAM_REG_END:		u32 = 0x03FF_FFFF;

/* RSP memory. */
const SP_DMEM_START:		u32 = 0x0400_0000;
const SP_DMEM_END:			u32 = 0x0400_0FFF;
const SP_IMEM_START:		u32 = 0x0400_1000;
const SP_IMEM_END:			u32 = 0x0400_1FFF;

/* RSP registers. */
const SP_REG_START:			u32 = 0x0404_0000;
const SP_REG_END:			u32 = 0x0404_0018;

/* RDP registers. */
const RDP_CMD_START:		u32 = 0x0410_0000;
const RDP_CMD_END:			u32 = 0x041F_FFFF;
const RDP_SPAN_START:		u32 = 0x0420_0000;
const RDP_SPAN_END:			u32 = 0x042F_FFFF;

/* MI registers. */
const MI_REG_START:			u32 = 0x0430_0000;
const MI_REG_END:			u32 = 0x043F_FFFF;

/* VI registers. */
const VI_REG_START:			u32 = 0x0440_0000;
const VI_REG_END:			u32 = 0x044F_FFFF;

/* AI registers. */
const AI_REG_START:			u32 = 0x0450_0000;
const AI_REG_END:			u32 = 0x045F_FFFF;

/* PI registers. */
const PI_REG_START:			u32 = 0x0460_0000;
const PI_REG_END:			u32 = 0x046F_FFFF;

/* RI registers. */
const RI_REG_START:			u32 = 0x0470_0000;
const RI_REG_END:			u32 = 0x047F_FFFF;

/* SI registers. */
const SI_REG_START:			u32 = 0x0480_0000;
const SI_REG_END:			u32 = 0x048F_FFFF;

/* Unused memory segment	. */
const UNUSED_START:			u32 = 0x0490_0000;
const UNUSED_END:			u32 = 0x04FF_FFFF;

/* Cartridge memory. */
const CART_DOM2_A1_START:	u32 = 0x0500_0000;
const CART_DOM2_A1_END:		u32 = 0x05FF_FFFF;
const CART_DOM1_A1_START:	u32 = 0x0600_0000;
const CART_DOM1_A1_END:		u32 = 0x07FF_FFFF;
const CART_DOM2_A2_START:	u32 = 0x0800_0000;
const CART_DOM2_A2_END:		u32 = 0x0FFF_FFFF;
const CART_DOM1_A2_START:	u32 = 0x1000_0000;
const CART_DOM1_A2_END:		u32 = 0x1FBF_FFFF;
const CART_DOM1_A3_START:	u32 = 0x1FD0_0000;
const CART_DOM1_A3_END:		u32 = 0x7FFF_FFFF;

/* PIF memory. */
pub const PIF_ROM_START:	u32 = 0x1FC0_0000;
const PIF_ROM_END:			u32 = 0x1FC0_07BF;
pub const PIF_RAM_START:	u32 = 0x1FC0_07C0;
pub const PIF_RAM_END:		u32 = 0x1FC0_07FF;

/* Reserved memory space. */
const RESERVED_START:		u32 = 0x1FC0_0800;
const RESERVED_END:			u32 = 0x1FCF_FFFF;

/* External device memory .*/
const SYSAD_START:			u32 = 0x8000_0000;
const SYSAD_END:			u32 = 0xFFFF_FFFF;

/* Virtual memory spaces. */
pub const KSEG0_START:		u32 = 0x8000_0000;
const KSEG0_END:			u32 = 0x9FFF_FFFF;
const KSEG1_START:			u32 = 0xA000_0000;
const KSEG1_END:			u32 = 0xBFFF_FFFF;

#[repr(C, packed)]
pub struct N64_ROM_HEADER {
	pub pi_bsd_dom1: u32,
	pub clock: u32,
	pub pc: u32,
	pub release: u32,
	pub crc1: u32,
	pub crc2: u32,
	pub unknown1: u64,
	pub name: [u8; 20],
	pub unknown2: u32,
	pub format: u32,
	pub id: u16,
	pub country: u8,
	pub version: u8
}
pub const N64_ROM_HEADER_SIZE: usize = 0x40;

/* Reads a 32-bit word from a boxed slice of u8s. */
fn rmem(addr: u32, mem: &Box<[u8]>) -> u32 {
	/* Obtain a slice starting at the read address. */
	let b: &[u8] = &mem[addr as usize .. addr as usize + 4];
	/* Extract each of the word's bytes and use them to create a u32. */
	let w = ((b[0] as u32) << 24) | ((b[1] as u32) << 16) | ((b[2] as u32) << 8) | b[3] as u32;
	/* Byte swap and adjust the endianness of the read word. */
	u32::from_be(w.swap_bytes())
}

/* Writes a 32-bit word to a boxed slice of u8s. */
fn wmem(val: u32, addr: u32, mem: &mut Box<[u8]>) {
	/* Obtain a slice of bytes from the u32. */
	let from: &[u8] = &[(val >> 24) as u8, (val >> 16) as u8, (val >> 8) as u8, val as u8];
	/* Write the slice into memory. */
	mem[addr as usize .. addr as usize + 4].copy_from_slice(from)
}

/* Convers a virtual address to a physical address. */
fn vtop(vaddr: u32) -> u32 {
	match vaddr {
		KSEG0_START ... KSEG0_END => {
			/* Direct mapped segment KSEG0. */
			return vaddr - KSEG0_START;
		}, KSEG1_START ... KSEG1_END => {
			/* Direct mapped segment KSEG1. */
			return vaddr - KSEG1_START;
		}, _ => panic!("Unrecognized virtual address: {:#x}", vaddr)
	}
}

pub struct N64 {

	/* Memories */

	/* 4MB internal RDRAM. */
	iram: Box<[u8]>,
	/* 4MB expansion RDRAM. */
	eram: Box<[u8]>,
	/* Cartridge ROM. */
	crom: Box<[u8]>,

	/* RCP-NUS */

	/* Virtual VI. (Video Interface) */
	vi: VI,
	/* Virtual AI. (Audio Interface) */
	ai: AI,
	/* Virtual SI. (Serial Interface) */
	si: SI,
	/* Virtual PI. (Peripheral Interface) */
	pi: PI,
	/* Virtual RSP. (Reality Signal Processor) */
	rsp: RSP,
	/* Virtual RDP. (Reality Display Processor) */
	rdp: RDP,
	/* Virtual PIF. (Peripheral Interface). */
	pif: PIF,

	/* CPU-NUS */

	/* Virtual VR4300 MIPS 64-bit CPU. */
	pub cpu: CPU
}

impl N64 {
	/* Initializer for the N64 umbrella module. */
	/* > Accepts a cartridge ROM slice (cr) and a PIF ROM slice (pr). */
	pub fn new(cr: Box<[u8]>, pr: Box<[u8]>) -> N64 {
		N64 {

			/* Allocate the IRAM. */
			iram: vec![0; N64_IRAM_SIZE].into_boxed_slice(),
			/* Allocate the ERAM. */
			eram: vec![0; N64_ERAM_SIZE].into_boxed_slice(),
			/* Transfer ownership of the CROM. */
			crom: cr,

			/* RCP-NUS */
			vi: VI::new(),
			ai: AI::new(),
			si: SI::new(),
			pi: PI::new(),
			rsp: RSP::new(),
			rdp: RDP::new(),
			pif: PIF::new(pr),

			/* CPU-NUS */
			cpu: CPU::new((PIF_ROM_START | KSEG0_START) as u64),
		}
	}

	/* Reads a word from the memory map. */
	pub fn read(&self, vaddr: u32) -> u32 {
		/* Convert the virtual address to a physical address. */
		let paddr = vtop(vaddr);
		/* Match the memory address to a peripheral address range. */
		match paddr {
			RDRAM_MEM_START ... RDRAM_MEM_END => {
				return rmem(paddr - RDRAM_MEM_START, &self.iram);
			}, RDRAM_REG_START ... RDRAM_REG_END => {
				unimplemented!()
			}, SP_REG_START ... SP_REG_END => {
				return self.rsp.rreg(paddr);
			}, SP_DMEM_START ... SP_DMEM_END => {
				return rmem(paddr - SP_DMEM_START, &self.rsp.dmem);
			}, SP_IMEM_START ... SP_IMEM_END => {
				return rmem(paddr - SP_DMEM_START, &self.rsp.imem);
			}, RDP_CMD_START ... RDP_CMD_END => {
				unimplemented!()
			}, RDP_SPAN_START ... RDP_SPAN_END => {
				unimplemented!()
			}, MI_REG_START ... MI_REG_END => {
				unimplemented!()
			}, VI_REG_START ... VI_REG_END => {
				unimplemented!()
			}, AI_REG_START ... AI_REG_END => {
				unimplemented!()
			}, PI_REG_START ... PI_REG_END => {
				unimplemented!()
			}, RI_REG_START ... RI_REG_END => {
				unimplemented!()
			}, SI_REG_START ... SI_REG_END => {
				unimplemented!()
			}, UNUSED_START ... UNUSED_END => {
				panic!("Attempt to read from unused address space.")
			}, CART_DOM2_A1_START ... CART_DOM2_A1_END => {
				return rmem(paddr - CART_DOM2_A1_START, &self.crom);
			}, CART_DOM1_A1_START ... CART_DOM1_A1_END => {
				return rmem(paddr - CART_DOM1_A1_START, &self.crom);
			}, CART_DOM2_A2_START ... CART_DOM2_A2_END => {
				return rmem(paddr - CART_DOM2_A2_START, &self.crom);
			}, CART_DOM1_A2_START ... CART_DOM1_A2_END => {
				return rmem(paddr - CART_DOM1_A2_START, &self.crom);
			}, PIF_ROM_START ... PIF_ROM_END => {
				return rmem(paddr - PIF_ROM_START, &self.pif.prom);
			}, PIF_RAM_START ... PIF_RAM_END => {
				return rmem(paddr - PIF_RAM_START, &self.pif.pram);
			}, RESERVED_START ... RESERVED_END => {
				panic!("Attempt to read from a reserved location {:#x}.", paddr)
			}, CART_DOM1_A3_START ... CART_DOM1_A3_END => {
				return rmem(paddr - CART_DOM1_A3_START, &self.crom);
			}, SYSAD_START ... SYSAD_END => {
				unimplemented!()
			}, _ => panic!("Read from unrecognized physical address: {:#x}", paddr)
		}
	}

	/* Writes a word to the provided N64's memory map. */
	pub fn write(&mut self, val: u32, vaddr: u32) {
		/* Convert the virtual address to the physical address. */
		let paddr = vtop(vaddr);
		/* Match the memory address to a peripheral address range. */
		match paddr {
			RDRAM_MEM_START ... RDRAM_MEM_END => {
				wmem(val, paddr - RDRAM_MEM_START, &mut self.iram)
			}, RDRAM_REG_START ... RDRAM_REG_END => {
				unimplemented!()
			}, SP_DMEM_START ... SP_DMEM_END => {
				wmem(paddr - SP_DMEM_START, val, &mut self.rsp.dmem)
			}, SP_IMEM_START ... SP_IMEM_END => {
				wmem(paddr - SP_DMEM_START, val, &mut self.rsp.imem)
			}, SP_REG_START ... SP_REG_END => {
				self.rsp.wreg(paddr - SP_REG_START)
			}, RDP_CMD_START ... RDP_CMD_END => {
				unimplemented!()
			}, RDP_SPAN_START ... RDP_SPAN_END => {
				unimplemented!()
			}, MI_REG_START ... MI_REG_END => {
				unimplemented!()
			}, VI_REG_START ... VI_REG_END => {
				unimplemented!()
			}, AI_REG_START ... AI_REG_END => {
				unimplemented!()
			}, PI_REG_START ... PI_REG_END => {
				unimplemented!()
			}, RI_REG_START ... RI_REG_END => {
				unimplemented!()
			}, SI_REG_START ... SI_REG_END => {
				unimplemented!()
			}, UNUSED_START ... UNUSED_END => {
				panic!("Attempt to write to unused address space.")
			}, CART_DOM2_A1_START ... CART_DOM2_A1_END |
			CART_DOM1_A1_START ... CART_DOM1_A1_END => {
				unimplemented!()
			}, CART_DOM2_A2_START ... CART_DOM2_A2_END => {
				unimplemented!()
			}, CART_DOM1_A2_START ... CART_DOM1_A2_END => {
				unimplemented!()
			}, CART_DOM1_A3_START ... CART_DOM1_A3_END => {
				unimplemented!()
			}, PIF_ROM_START ... PIF_ROM_END => {
				panic!("Attempt to write to a read-only location {:#x}.", paddr)
			}, PIF_RAM_START ... PIF_RAM_END => {
				wmem(val, paddr - PIF_RAM_START, &mut self.pif.pram)
			}, RESERVED_START ... RESERVED_END => {
				panic!("Attempt to write to a reserved location {:#x}.", paddr)
			}, SYSAD_START ... SYSAD_END => {
				unimplemented!()
			}, _ => panic!("Write to unrecognized physical address: {:#x}", paddr)
		}
	}
}
