/* n64.rs - A top-level module that concatenates the sub-modules of the emulator. */

mod vi;
mod ai;
mod mi;
mod pi;
mod ri;
mod rsp;
mod rdp;
mod pif;
mod mc;

use n64::vi::VI;
use n64::ai::AI;
use n64::mi::MI;
use n64::ri::RI;
use n64::pi::PI;
use n64::rsp::RSP;
use n64::rdp::RDP;
use n64::pif::PIF;

use self::mc::MC;
pub mod vr4300;
use self::vr4300::VR4300;

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
    pub mc: MC,

    /* CPU-NUS */

    /* Virtual VR4300 MIPS 64-bit VR4300. */
    pub cpu: VR4300,

    /* RCP-NUS */

    /* Virtual VI. (Video Interface) */
    vi: VI,
    /* Virtual AI. (Audio Interface) */
    ai: AI,
    /* Virtual MI (MIPS Interface) */
    mi: MI,
    /* Virtual RI (RAM Interface) */
    ri: RI,
    /* Virtual PI. (Peripheral Interface) */
    pi: PI,
    /* Virtual RSP. (Reality Signal Processor) */
    rsp: RSP,
    /* Virtual RDP. (Reality Display Processor) */
    rdp: RDP,

    /* PIF-NUS */

    /* PIF (Peripheral InterFace) */
    pif: PIF
}

impl N64 {
    /* Initializer for the N64 umbrella module. */
    /* > Accepts a cartridge ROM slice (cr) and a PIF ROM slice (pr). */
    pub fn new(cr: Box<[u8]>, pr: Box<[u8]>) -> N64 {
        N64 {
            mc: MC::new(cr, pr),
            /* VR4300-NUS */
            cpu: VR4300::new((mc::PIF_ROM_START | mc::KSEG0_START) as u64),

            /* RCP-NUS */
            vi: VI::new(),
            ai: AI::new(),
            mi: MI::new(),
            ri: RI::new(),
            pi: PI::new(),
            rsp: RSP::new(),
            rdp: RDP::new(),

            /* PIF-NUS */
            pif: PIF::new(pr),
        }
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.mc)
    }
}
