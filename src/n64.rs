use cpu::CPU;
use pif::PIF;
use rsp::RSP;
use rdp::RDP;

/* N64 hardware constants. */
static N64_IRAM_SIZE: usize = 0x400000;
static N64_ERAM_SIZE: usize = 0x400000;

pub struct N64 {
	/* Virtual representation of the console's 4MB internal RDRAM. */
	iram: Box<[u8]>,
	/* Virtual representation of the console's 4MB expansion RDRAM. */
	eram: Box<[u8]>,
	/* Virtual representation of the console's cartridge ROM. */
	crom: Box<[u8]>,
	/* Virtual VR4300 MIPS 64-bit CPU. */
	cpu: CPU,
	/* Virtual PIF. (Peripheral Interface). */
	pif: PIF,
	/* Virtual RSP. (Reality Signal Processor) */
	rsp: RSP,
	/* Virtual RDP. (Reality Display Processor) */
	rdp: RDP
}

impl N64 {
	pub fn begin(&self) {
		println!("Starting emulation.");

	}
	/* Initializer for the N64 implementation. */
	pub fn new(r: Box<[u8]>) -> N64 {
		N64 {
			iram: vec![0; N64_IRAM_SIZE].into_boxed_slice(),
			eram: vec![0; N64_ERAM_SIZE].into_boxed_slice(),
			crom: r,
			cpu: CPU::new(),
			pif: PIF::new(),
			rsp: RSP::new(),
			rdp: RDP::new()
		}
	}
}
