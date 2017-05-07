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

pub struct N64 {

    /* MC (Memory Controller) */
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
        self.cpu.cycle(&mut self.mc)
	}
	/* Initializer for the N64 umbrella module. */
	pub fn new(cr: Box<[u8]>) -> N64 {
		N64 {

            /* Memory Controller */
            mc: MC::new(cr),

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
