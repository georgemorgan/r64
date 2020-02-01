const PI_REG_DRAM_ADDR: u32 = 0x0460_0000;
const PI_REG_CART_ADDR: u32 = 0x0460_0004;
const PI_REG_RD_LEN: u32 = 0x0460_0008;
const PI_REG_WR_LEN: u32 = 0x0460_000C;
const PI_REG_STATUS: u32 = 0x0460_0010;
const PI_REG_BSD_DOM1_LAT: u32 = 0x0460_0014;
const PI_REG_BSD_DOM1_PWD: u32 = 0x0460_0018;
const PI_REG_BSD_DOM1_PGS: u32 = 0x0460_001C;
const PI_REG_BSD_DOM1_RLS: u32 = 0x0460_0020;
const PI_REG_BSD_DOM2_LAT: u32 = 0x0460_0024;
const PI_REG_BSD_DOM2_PWD: u32 = 0x0460_0028;
const PI_REG_BSD_DOM2_PGS: u32 = 0x0460_002C;
const PI_REG_BSD_DOM2_RLS: u32 = 0x0460_0030;

/*

PI_BASE_REG - 0x04600000

0x04600000 to 0x04600003  PI_DRAM_ADDR_REG  //PI DRAM address
   (RW): [23:0] starting RDRAM address

0x04600004 to 0x04600007  PI_CART_ADDR_REG //PI pbus (cartridge) address
   (RW): [31:0] starting AD16 address

0x04600008 to 0x0460000B  PI_RD_LEN_REG //PI read length
   (RW): [23:0] read data length

0x0460000C to 0x0460000F  PI_WR_LEN_REG //PI write length
   (RW): [23:0] write data length

0x04600010 to 0x04600013 PI_STATUS_REG //PI status
    (R): [0] DMA busy             (W): [0] reset controller
         [1] IO busy                       (and abort current op)
         [2] error                     [1] clear intr

0x04600014 to 0x04600017  PI_BSD_DOM1_LAT_REG or PI_DOMAIN1_REG //PI dom1 latency
   (RW): [7:0] domain 1 device latency

0x04600018 to 0x0460001B  PI_BSD_DOM1_PWD_REG //PI dom1 pulse width
   (RW): [7:0] domain 1 device R/W strobe pulse width

0x0460001C to 0x0460001F  PI_BSD_DOM1_PGS_REG //PI dom1 page size
   (RW): [3:0] domain 1 device page size

0x04600020 to 0x04600023  PI_BSD_DOM1_RLS_REG //PI dom1 release
   (RW): [1:0] domain 1 device R/W release duration

0x04600024 to 0x04600027  PI_BSD_DOM2_LAT_REG or PI_DOMAIN2_REG //PI dom2 latency
   (RW): [7:0] domain 2 device latency

0x04600028 to 0x0460002B  PI_BSD_DOM2_PWD_REG //PI dom2 pulse width
   (RW): [7:0] domain 2 device R/W strobe pulse width

0x0460002C to 0x0460002F  PI_BSD_DOM2_PGS_REG //PI dom2 page size
   (RW): [3:0] domain 2 device page size

0x04600030 to 0x04600033  PI_BSD_DOM2_RLS_REG //PI dom2 release
   (RW): [1:0] domain 2 device R/W release duration

0x04600034 to 0x046FFFFF  Unused

*/

pub struct PI {
    dram_addr: u32,
    cart_addr: u32,
    rd_len : u32,
    wr_len: u32,
    status: u32,
    bsd_dom1_lat: u32,
    bsd_dom1_pwd: u32,
    bsd_dom1_pgs: u32,
    bsd_dom1_rls: u32,
    bsd_dom2_lat: u32,
    bsd_dom2_pwd: u32,
    bsd_dom2_pgs: u32,
    bsd_dom2_rls: u32,
}

impl PI {
    pub fn new() -> PI {
        PI {
            dram_addr: 0,
            cart_addr: 0,
            rd_len : 0,
            wr_len: 0,
            status: 0,
            bsd_dom1_lat: 0,
            bsd_dom1_pwd: 0,
            bsd_dom1_pgs: 0,
            bsd_dom1_rls: 0,
            bsd_dom2_lat: 0,
            bsd_dom2_pwd: 0,
            bsd_dom2_pgs: 0,
            bsd_dom2_rls: 0,
        }
    }

    /* Reads from the PI's registers. */
    pub fn rreg(&self, reg: u32) -> u32 {
        match reg {
            PI_REG_DRAM_ADDR => {
                self.dram_addr
            }, PI_REG_CART_ADDR => {
                self.cart_addr
            }, PI_REG_RD_LEN => {
                self.rd_len
            }, PI_REG_WR_LEN => {
                self.wr_len
            }, PI_REG_STATUS => {
                self.status
            }, PI_REG_BSD_DOM1_LAT => {
                self.bsd_dom1_lat
            }, PI_REG_BSD_DOM1_PWD => {
                self.bsd_dom1_pwd
            }, PI_REG_BSD_DOM1_PGS => {
                self.bsd_dom1_pgs
            }, PI_REG_BSD_DOM1_RLS => {
                self.bsd_dom1_rls
            }, PI_REG_BSD_DOM2_LAT => {
                self.bsd_dom2_lat
            }, PI_REG_BSD_DOM2_PWD => {
                self.bsd_dom2_pwd
            }, PI_REG_BSD_DOM2_PGS => {
                self.bsd_dom2_pgs
            }, PI_REG_BSD_DOM2_RLS => {
                self.bsd_dom2_rls
            }, _ => panic!("Read from unrecognized PI register address: {:#x}", reg)
        }
    }

    /* Writes to the PI's registers. */
    pub fn wreg(&mut self, reg: u32, value: u32) {
        match reg {
            PI_REG_DRAM_ADDR => {
                self.dram_addr = value
            }, PI_REG_CART_ADDR => {
                self.cart_addr = value
            }, PI_REG_RD_LEN => {
                self.rd_len = value
            }, PI_REG_WR_LEN => {
                self.wr_len = value
            }, PI_REG_STATUS => {
                self.status = value
            }, PI_REG_BSD_DOM1_LAT => {
                self.bsd_dom1_lat = value
            }, PI_REG_BSD_DOM1_PWD => {
                self.bsd_dom1_pwd = value
            }, PI_REG_BSD_DOM1_PGS => {
                self.bsd_dom1_pgs = value
            }, PI_REG_BSD_DOM1_RLS => {
                self.bsd_dom1_rls = value
            }, PI_REG_BSD_DOM2_LAT => {
                self.bsd_dom2_lat = value
            }, PI_REG_BSD_DOM2_PWD => {
                self.bsd_dom2_pwd = value
            }, PI_REG_BSD_DOM2_PGS => {
                self.bsd_dom2_pgs = value
            }, PI_REG_BSD_DOM2_RLS => {
                self.bsd_dom2_rls = value
            }, _ => panic!("Write to unrecognized PI register address: {:#x}", reg)
        }
    }
}
