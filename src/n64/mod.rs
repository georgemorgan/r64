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
mod mc;
use self::mc::MC;
pub mod cpu;
use self::cpu::CPU;

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

pub struct N64 {

	/* Virtual MC (Memeory Controller) */
	mc: MC,

	/* CPU-NUS */

	/* Virtual VR4300 MIPS 64-bit CPU. */
	cpu: CPU
}

impl N64 {
	/* Initializer for the N64 umbrella module. */
	/* > Accepts a cartridge ROM slice (cr) and a PIF ROM slice (pr). */
	pub fn new(cr: Box<[u8]>, pr: Box<[u8]>) -> N64 {
		N64 {
			mc: MC::new(cr, pr),
			/* CPU-NUS */
			cpu: CPU::new((mc::PIF_ROM_START | mc::KSEG0_START) as u64),
		}
	}

	pub fn cycle(&mut self) {
		self.cpu.cycle(&mut self.mc)
	}
}
