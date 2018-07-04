use n64::mc::SP_DMEM_START;
use n64::mc::SP_DMEM_END;
use n64::mc::SP_IMEM_START;
use n64::mc::SP_IMEM_END;

/* LO registers */
const SP_REG_MEM_ADDR: u32 = 0x0404_0000;
const SP_REG_DRAM_ADDR: u32 = 0x0404_0004;
const SP_REG_RD_LEN: u32 = 0x0404_0008;
const SP_REG_WR_LEN: u32 = 0x0404_000C;
const SP_REG_STATUS: u32 = 0x0404_0010;
const SP_REG_DMA_FULL: u32 = 0x0404_0014;
const SP_REG_DMA_BUSY: u32 = 0x0404_0018;
const SP_REG_SEMAPHORE: u32 = 0x0404_001C;
const SP_REG_PC: u32 = 0x0408_0000;

/* SP_REG_STATUS */
const SP_STATUS_HALT: u32 = (1 << 0);
const SP_STATUS_BROKE: u32 = (1 << 1);
const SP_STATUS_DMABUSY: u32 = (1 << 2);
const SP_STATUS_DMAFULL: u32 = (1 << 3);
const SP_STATUS_IOFULL: u32 = (1 << 4);
const SP_STATUS_SSTEP: u32 = (1 << 5);
const SP_STATUS_INTR_BREAK: u32 = (1 << 6);
const SP_STATUS_SIGNAL0: u32 = (1 << 7);
const SP_STATUS_SIGNAL1: u32 = (1 << 8);
const SP_STATUS_SIGNAL2: u32 = (1 << 9);
const SP_STATUS_SIGNAL3: u32 = (1 << 10);
const SP_STATUS_SIGNAL4: u32 = (1 << 11);
const SP_STATUS_SIGNAL5: u32 = (1 << 12);
const SP_STATUS_SIGNAL6: u32 = (1 << 13);
const SP_STATUS_SIGNAL7: u32 = (1 << 14);

pub struct RSP {
    /* 4KB data memory. */
    pub dmem: Box<[u8]>,
    /* 4KB instruction memory. */
    pub imem: Box<[u8]>,

    /* Registers. */
    mem_addr: u32,
    dram_addr: u32,
    rd_len: u32,
    wr_len: u32,
    status: u32,
    dma_full: u32,
    dma_busy: u32,
    semaphore: u32,
    pc: u32
}

impl RSP {
    pub fn new() -> RSP {
        RSP {
            /* Allocate the DMEM. */
            dmem: vec![0; (SP_DMEM_END - SP_DMEM_START) as usize].into_boxed_slice(),
            /* Allocate the IMEM. */
            imem: vec![0;  (SP_IMEM_END - SP_IMEM_START) as usize].into_boxed_slice(),

            mem_addr: 0,
            dram_addr: 0,
            rd_len: 0,
            wr_len: 0,
            status: SP_STATUS_HALT,
            dma_full: 0,
            dma_busy: 0,
            semaphore: 0,
            pc: 0
        }
    }

    pub fn dma_read(&mut self) {

    }

    pub fn dma_write(&mut self) {

    }

    /* Reads from the RSP's registers. */
    pub fn rreg(&self, reg: u32) -> u32 {
        match reg {
            SP_REG_MEM_ADDR => {
                self.mem_addr
            }, SP_REG_DRAM_ADDR => {
                self.dram_addr
            }, SP_REG_RD_LEN => {
                self.rd_len
            }, SP_REG_WR_LEN => {
                self.wr_len
            }, SP_REG_STATUS => {
                self.status
            }, SP_REG_DMA_FULL => {
                self.dma_full
            }, SP_REG_DMA_BUSY => {
                self.dma_busy
            }, _ => panic!("Read from unrecognized RSP register address: {:#x}", reg)
        }
    }

    /* Writes to the RSP's registers. */
    pub fn wreg(&mut self, reg: u32, value: u32) {
        match reg {
            SP_REG_MEM_ADDR => {
                self.mem_addr = value
            }, SP_REG_DRAM_ADDR => {
                self.dram_addr = value
            }, SP_REG_RD_LEN => {
                self.rd_len = value
            }, SP_REG_WR_LEN => {
                self.wr_len = value
            }, SP_REG_STATUS => {
                self.status = value
            }, SP_REG_DMA_FULL => {
                self.dma_full = value
            }, SP_REG_DMA_BUSY => {
                self.dma_busy = value
            }, _ => panic!("Write to unrecognized RSP register address: {:#x}", reg)
        }
    }
}
