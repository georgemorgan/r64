/* self.rs - A top-level module that concatenates the sub-modules of the emulator. */

mod vi;
mod ai;
mod mi;
mod pi;
mod ri;
mod rsp;
mod rdp;

use self::vi::VI;
use self::ai::AI;
use self::mi::MI;
use self::ri::RI;
use self::pi::PI;
use self::rsp::RSP;
use self::rdp::RDP;

/* RCP-NUS */

pub struct RCP {
    /* Virtual VI. (Video Interface) */
    pub vi: VI,
    /* Virtual AI. (Audio Interface) */
    pub ai: AI,
    /* Virtual MI (MIPS Interface) */
    pub mi: MI,
    /* Virtual RI (RAM Interface) */
    pub ri: RI,
    /* Virtual PI. (Peripheral Interface) */
    pub pi: PI,
    /* Virtual RSP. (Reality Signal Processor) */
    pub rsp: RSP,
    /* Virtual RDP. (Reality Display Processor) */
    pub rdp: RDP
}

impl RCP {
    /* Initializer for the N64 umbrella module. */
    /* > Accepts a cartridge ROM slice (cr) and a PIF ROM slice (pr). */
    pub fn new() -> RCP {
        RCP {
            vi: VI::new(),
            ai: AI::new(),
            mi: MI::new(),
            ri: RI::new(),
            pi: PI::new(),
            rsp: RSP::new(),
            rdp: RDP::new()
        }
    }
}
