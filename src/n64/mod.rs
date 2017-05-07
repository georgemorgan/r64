mod mc;
use self::mc::MC;

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

use std::str;

/* N64 hardware constants. */
const N64_IRAM_SIZE: usize = 0x400000;
const N64_ERAM_SIZE: usize = 0x400000;

pub struct N64 {

	/* ~ System memories. ~ */

	/* Virtual representation of the console's 4MB internal RDRAM. */
	iram: Box<[u8]>,
	/* Virtual representation of the console's 4MB expansion RDRAM. */
	eram: Box<[u8]>,
	/* Virtual representation of the console's cartridge ROM. */
	crom: Box<[u8]>,

    /* N64 */
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
	pub fn begin(&self) {
		println!("Starting emulation.");
		println!("ROM is {} bytes.", self.crom.len());
		let name = str::from_utf8(&self.crom[0x20..0x20+20]).unwrap().trim();
		println!("The ROM is '{}'.", name);
	}
	/* Initializer for the N64 implementation. */
	pub fn new(r: Box<[u8]>) -> N64 {
		N64 {
			iram: vec![0; N64_IRAM_SIZE].into_boxed_slice(),
			eram: vec![0; N64_ERAM_SIZE].into_boxed_slice(),
			crom: r,

            /* N64 */
            mc: MC::new(),

            /* RCP-NUS */
			vi: VI::new(),
			ai: AI::new(),
			si: SI::new(),
			pi: PI::new(),
			rsp: RSP::new(),
			rdp: RDP::new(),
			pif: PIF::new(),

            /* CPU-NUS */
			cpu: CPU::new(),
		}
	}
}
