use crate::mc::SP_DMEM_START;
use crate::mc::SP_DMEM_END;
use crate::mc::SP_IMEM_START;
use crate::mc::SP_IMEM_END;

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

/*

SP_BASE_REG - 0x04040000

0x04000000 to 0x04000FFF R/W SP_DMEM read/write (4KB)
0x04001000 to 0x04001FFF R/W SP_IMEM read/write (4KB)
0x04002000 to 0x0403FFFF *   Unused

0x04040000 to 0x04040003  SP_MEM_ADDR_REG //Master, SP memory address
   (RW): [11:0] DMEM/IMEM address
         [12]   0=DMEM,1=IMEM

0x04040004 to 0x04040007  SP_DRAM_ADDR_REG //Slave, SP DRAM DMA address
   (RW): [23:0] RDRAM address

0x04040008 to 0x0404000B  SP_RD_LEN_REG //SP read DMA length
   (RW): [11:0] length
         [19:12] count
         [31:20] skip
              direction: I/DMEM <- RDRAM

0x0404000C to 0x0404000F  SP_WR_LEN_REG //SP write DMA length
   (RW): [11:0] length
         [19:12] count
         [31:20] skip
              direction: I/DMEM to RDRAM

0x04040010 to 0x04040013  SP_STATUS_REG //SP status
    (W): [0]  clear halt          (R): [0]  halt
         [1]  set halt                 [1]  broke
         [2]  clear broke              [2]  dma busy
         [3]  clear intr               [3]  dma full
         [4]  set intr                 [4]  io full
         [5]  clear sstep              [5]  single step
         [6]  set sstep                [6]  interrupt on break
         [7]  clear intr on break      [7]  signal 0 set
         [8]  set intr on break        [8]  signal 1 set
         [9]  clear signal 0           [9]  signal 2 set
         [10] set signal 0             [10] signal 3 set
         [11] clear signal 1           [11] signal 4 set
         [12] set signal 1             [12] signal 5 set
         [13] clear signal 2           [13] signal 6 set
         [14] set signal 2             [14] signal 7 set
         [15] clear signal 3
         [16] set signal 3
         [17] clear signal 4
         [18] set signal 4
         [19] clear signal 5
         [20] set signal 5
         [21] clear signal 6
         [22] set signal 6
         [23] clear signal 7
         [24] set signal 7

0x04040014 to 0x04040017  SP_DMA_FULL_REG //SP DMA full
    (R): [0] valid bit
             dma full

0x04040018 to 0x0404001B  SP_DMA_BUSY_REG //SP DMA busy
    (R): [0] valid bit
             dma busy

0x0404001C to 0x0404001F  SP_SEMAPHORE_REG //SP semaphore
    (R): [0] semaphore flag (set on read)
    (W): [] clear semaphore flag

0x04040020 to 0x0407FFFF * Unused

0x04080000 to 0x04080003  SP_PC_REG //SP PC
   (RW): [11:0] program counter

0x04080004 to 0x04080007  SP_IBIST_REG //SP IMEM BIST REG
    (W): [0] BIST check           (R): [0] BIST check
         [1] BIST go                   [1] BIST go
         [2] BIST clear                [2] BIST done
                                       [3-6] BIST fail
0x04080008 to 0x040FFFFF * Unused

*/

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
            dmem: Box::new([0; (SP_DMEM_END - SP_DMEM_START) as usize]),
            /* Allocate the IMEM. */
            imem: Box::new([0;  (SP_IMEM_END - SP_IMEM_START) as usize]),

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
