use n64::PIF_RAM_START;
use n64::PIF_RAM_END;

const SI_REG_DRAM_ADDR: u32 = 0x0480_0000;
const SI_REG_PIF_ADDR_RD64B: u32 = 0x0480_0004;
const SI_REG_PIF_ADDR_WR64B: u32 = 0x0480_0010;
const SI_REG_STATUS: u32 = 0x0480_0018;

pub struct PIF {
    /* PIF memories. */

    /* The PIF's internal RAM. */
    pub pram: Box<[u8]>,
    /* The PIF's internal ROM from which the console boots. */
    pub prom: Box<[u8]>,

    dram_addr: u32,
    pif_addr_rd64b: u32,
    pif_addr_wr64b: u32,
    status: u32
}

impl PIF {
    pub fn new(pr: Box<[u8]>) -> PIF {
        PIF {
            /* Allocate the PRAM. */
            pram: vec![0; (PIF_RAM_END - PIF_RAM_START + 1) as usize].into_boxed_slice(),
            /* Transfer ownership of the PROM. */
            prom: pr,

            dram_addr: 0,
            pif_addr_rd64b: 0,
            pif_addr_wr64b: 0,
            status: 0
        }
    }

    /* Reads from the PIF's registers. */
    pub fn rreg(&self, reg: u32) -> u32 {
        match reg {
            SI_REG_DRAM_ADDR => {
                self.dram_addr
            }, SI_REG_PIF_ADDR_RD64B => {
                self.pif_addr_rd64b
            }, SI_REG_PIF_ADDR_WR64B => {
                self.pif_addr_wr64b
            }, SI_REG_STATUS => {
                self.status
            }, _ => panic!("Read from unrecognized PIF register address: {:#x}", reg)
        }
    }

    /* Writes to the PIF's registers. */
    pub fn wreg(&mut self, reg: u32, value: u32) {
        match reg {
            SI_REG_DRAM_ADDR => {
                self.dram_addr = value
            }, SI_REG_PIF_ADDR_RD64B => {
                self.pif_addr_rd64b = value
            }, SI_REG_PIF_ADDR_WR64B => {
                self.pif_addr_wr64b = value
            }, SI_REG_STATUS => {
                self.status = value
            }, _ => panic!("Write to unrecognized PIF register address: {:#x}", reg)
        }
    }
}
