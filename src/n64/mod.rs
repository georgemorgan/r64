/* n64.rs - Umbrella Module */

/* A top-level module that concatenates the sub-modules of the emulator. */

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
mod cpu;
use self::cpu::CPU;

mod mc;
use self::mc::MC;

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

	/* Virtual memory map controller. */
	mc: MC,

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
	cpu: CPU,
}

impl N64 {
	pub fn begin(&mut self) {
		for x in 0 .. 50 {
			self.cpu.cycle(&mut self.mc);
		}
	}
	/* Initializer for the N64 umbrella module. */
	/* > Accepts a cartridge ROM slice (cr) and a PIF ROM slice (pr). */
	pub fn new(cr: Box<[u8]>, pr: Box<[u8]>) -> N64 {
		N64 {

			/* Memory map controller. */
			mc: MC::new(cr, pr),

			/* RCP-NUS */
			vi: VI::new(),
			ai: AI::new(),
			si: SI::new(),
			pi: PI::new(),
			rsp: RSP::new(),
			rdp: RDP::new(),
			pif: PIF::new(),

			/* CPU-NUS */
			cpu: CPU::new(mc::PIF_ROM_START as u64),
		}
	}
}
