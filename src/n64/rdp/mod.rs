const DPC_REG_START: u32 = 0x0410_0000;
const DPC_REG_END: u32 = 0x0410_0004;
const DPC_REG_CURRENT: u32 = 0x0410_0008;
const DPC_REG_STATUS: u32 = 0x0410_000C;
const DPC_REG_CLOCK: u32 = 0x0410_0010;
const DPC_REG_BUFBUSY: u32 = 0x0410_0014;
const DPC_REG_PIPEBUSY: u32 = 0x0410_0018;
const DPC_REG_TMEM: u32 = 0x0410_001C;

const DPS_REG_TBIST: u32 = 0x0420_0000;
const DPS_REG_TEST_MODE: u32 = 0x0420_0004;
const DPS_REG_BUFTEST_ADDR: u32 = 0x0420_0008;
const DPS_REG_BUFTEST_DATA: u32 = 0x0420_000C;

pub struct RDP {
    start: u32,
    end: u32,
    current: u32,
    status: u32,
    clock: u32,
    bufbusy: u32,
    pipebusy: u32,
    tmem: u32,

    tbist: u32,
    test_mode: u32,
    buftest_addr: u32,
    buftest_data: u32
}

impl RDP {
    pub fn new() -> RDP {
        RDP {
            start: 0,
            end: 0,
            current: 0,
            status: 0,
            clock: 0,
            bufbusy: 0,
            pipebusy: 0,
            tmem: 0,

            tbist: 0,
            test_mode: 0,
            buftest_addr: 0,
            buftest_data: 0
        }
    }

    /* Reads from the PIF's registers. */
    pub fn dpc_rreg(&self, reg: u32) -> u32 {
        match reg {
            DPC_REG_START => {
                self.start
            }, DPC_REG_END => {
                self.end
            }, DPC_REG_CURRENT => {
                self.current
            }, DPC_REG_STATUS => {
                self.status
            }, DPC_REG_CLOCK => {
                self.clock
            }, DPC_REG_BUFBUSY => {
                self.bufbusy
            }, DPC_REG_PIPEBUSY => {
                self.pipebusy
            }, DPC_REG_TMEM => {
                self.tmem
            }, _ => panic!("Read from unrecognized PIF register address: {:#x}", reg)
        }
    }

    /* Reads from the PIF's registers. */
    pub fn dps_rreg(&self, reg: u32) -> u32 {
        match reg {
            DPS_REG_TBIST => {
                self.tbist
            }, DPS_REG_TEST_MODE => {
                self.test_mode
            }, DPS_REG_BUFTEST_ADDR => {
                self.buftest_addr
            }, DPS_REG_BUFTEST_DATA => {
                self.buftest_data
            }, _ => panic!("Read from unrecognized PIF register address: {:#x}", reg)
        }
    }

    /* Writes to the PIF's registers. */
    pub fn dpc_wreg(&mut self, reg: u32, value: u32) {
        match reg {
            DPC_REG_START => {
                self.start = value
            }, DPC_REG_END => {
                self.end = value
            }, DPC_REG_CURRENT => {
                self.current = value
            }, DPC_REG_STATUS => {
                self.status = value
            }, DPC_REG_CLOCK => {
                self.clock = value
            }, DPC_REG_BUFBUSY => {
                self.bufbusy = value
            }, DPC_REG_PIPEBUSY => {
                self.pipebusy = value
            }, DPC_REG_TMEM => {
                self.tmem = value
            }, _ => panic!("Write to unrecognized PIF register address: {:#x}", reg)
        }
    }

    /* Writes to the PIF's registers. */
    pub fn dps_wreg(&mut self, reg: u32, value: u32) {
        match reg {
            DPS_REG_TBIST => {
                self.tbist = value
            }, DPS_REG_TEST_MODE => {
                self.test_mode = value
            }, DPS_REG_BUFTEST_ADDR => {
                self.buftest_addr = value
            }, DPS_REG_BUFTEST_DATA => {
                self.buftest_data = value
            }, _ => panic!("Write to unrecognized PIF register address: {:#x}", reg)
        }
    }
}
