use n64::SP_DMEM_START;
use n64::SP_DMEM_END;
use n64::SP_IMEM_START;
use n64::SP_IMEM_END;

const SP_REG_NUM: usize = 10;

const SP_REG_MEM_ADDR: u32 = 0x04040000;
const SP_REG_DRAM_ADDR: u32 = 0x04040004;
const SP_REG_RD_LEN: u32 = 0x04040008;
const SP_REG_WR_LEN : u32 = 0x0404000C;
const SP_REG_STATUS: u32 = 0x04040010;
const SP_REG_DMA_FULL: u32 = 0x04040014;
const SP_REG_DMA_BUSY: u32 = 0x04040018;

pub struct RSP {
	/* 4KB data memory. */
	pub dmem: Box<[u8]>,
	/* 4KB instruction memory. */
	pub imem: Box<[u8]>,

	/* Registers. */
	regs: [u32; SP_REG_NUM]
}

impl RSP {
	pub fn new() -> RSP {
		RSP {
			/* Allocate the DMEM. */
			dmem: vec![0; (SP_DMEM_END - SP_DMEM_START) as usize].into_boxed_slice(),
			/* Allocate the IMEM. */
			imem: vec![0;  (SP_IMEM_END - SP_IMEM_START) as usize].into_boxed_slice(),

			regs: [0; SP_REG_NUM]
		}
	}
	/* Reads from the RSP's memory map. */
	pub fn rreg(&self, reg: u32) -> u32 {
		/* Obtain the value of the register. */
		self.regs[((reg & 0xff) % 4) as usize]
	}
	/* Reads from the RSP's memory map. */
	pub fn wreg(&self, reg: u32) {
		match reg {
			SP_REG_MEM_ADDR =>
				unimplemented!(),
			SP_REG_DRAM_ADDR =>
				unimplemented!(),
			SP_REG_RD_LEN =>
				unimplemented!(),
			SP_REG_WR_LEN =>
				unimplemented!(),
			SP_REG_STATUS =>
				unimplemented!(),
			SP_REG_DMA_FULL =>
				unimplemented!(),
			SP_REG_DMA_BUSY =>
				unimplemented!(),
			_ => panic!("Write to unrecognized RSP register address: {:#x}", reg)
		}
	}
}
