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
    pub cpu: VR4300
}

impl N64 {
    /* Initializer for the N64 umbrella module. */
    /* > Accepts a cartridge ROM slice (cr) and a PIF ROM slice (pr). */
    pub fn new(cr: Box<[u8]>, pr: Box<[u8]>) -> N64 {
        N64 {
            mc: MC::new(cr, pr),
            /* VR4300-NUS */
            cpu: VR4300::new((mc::PIF_ROM_START | mc::KSEG0_START) as u64),
        }
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.mc)
    }
}
