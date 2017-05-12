const RSP_DMEM_START: u32 = 0x04000000;
const RSP_DMEM_END: u32 = 0x040000FFF;
const RSP_IMEM_START: u32 = 0x04001000;
const RSP_IMEM_END: u32 = 0x040001FFF;
const RSP_REG_MEM_ADDR: u32 = 0x04040000;
const RSP_REG_DRAM_ADDR: u32 = 0x04040004;
const RSP_REG_RD_LEN: u32 = 0x04040008;
const RSP_REG_WR_LEN : u32 = 0x0404000C;
const RSP_REG_STATUS: u32 = 0x04040010;
const RSP_REG_DMA_FULL: u32 = 0x04040014;
const RSP_REG_DMA_BUSY: u32 = 0x04040018;
const RSP_REG_SEMAPHORE: u32 = 0x0404001C;
const RSP_REG_PC: u32 = 0x04080000;
const RSP_REG_IBIST: u32 = 0x04080004;

pub struct RSP {

}

impl RSP {
	pub fn new() -> RSP {
		RSP { }
	}
	/* Reads from the RSP's memory map. */
	pub fn read(&self, addr: u32) -> u32 {
		0
	}
}
