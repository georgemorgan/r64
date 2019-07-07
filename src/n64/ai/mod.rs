const AI_REG_DRAM_ADDR: u32 = 0x0450_0000;
const AI_REG_LEN: u32 = 0x0450_0004;
const AI_REG_CONTROL: u32 = 0x0450_0008;
const AI_REG_STATUS: u32 = 0x0450_000C;
const AI_REG_DACRATE: u32 = 0x0450_0010;
const AI_REG_BITRATE: u32 = 0x0450_0014;

/*

AI_BASE_REG - 0x04500000

       0x04500000 to 0x04500003  AI_DRAM_ADDR_REG //AI DRAM address
           (W): [23:0] starting RDRAM address (8B-aligned)

       0x04500004 to 0x04500007  AI_LEN_REG //AI length

          (RW): [14:0] transfer length (v1.0) - Bottom 3 bits are ignored
                [17:0] transfer length (v2.0) - Bottom 3 bits are ignored
       0x04500008 to 0x0450000B  AI_CONTROL_REG //AI control
           (W): [0] DMA enable - if LSB == 1, DMA is enabled

       0x0450000C to 0x0450000F  AI_STATUS_REG //AI status
           (R): [31]/[0] ai_full (addr & len buffer full)
                [30] ai_busy
                Note that a 1to0 transition in ai_full will set interrupt
           (W): clear audio interrupt

       0x04500010 to 0x04500013  AI_DACRATE_REG //AI DAC sample period register
           (W): [13:0] dac rate
                    - vid_clock/(dperiod + 1) is the DAC sample rate
                    - (dperiod + 1) >= 66 * (aclockhp + 1) must be true

       0x04500014 to 0x04500017  AI_BITRATE_REG //AI bit rate
           (W): [3:0] bit rate (abus clock half period register - aclockhp)
                    - vid_clock/(2*(aclockhp + 1)) is the DAC clock rate
                    - The abus clock stops if aclockhp is zero

       0x04500018 to 0x045FFFFF  Unused

*/

pub struct AI {
    dram_addr: u32,
    len: u32,
    control: u32,
    status: u32,
    dacrate: u32,
    bitrate: u32
}

impl AI {
    pub fn new() -> AI {
        AI {
            dram_addr: 0,
            len: 0,
            control: 0,
            status: 0,
            dacrate: 0,
            bitrate: 0
        }
    }

    /* Reads from the AI's registers. */
    pub fn rreg(&self, reg: u32) -> u32 {
        match reg {
            AI_REG_DRAM_ADDR => {
                self.dram_addr
            }, AI_REG_LEN => {
                self.len
            }, AI_REG_CONTROL => {
                self.control
            }, AI_REG_STATUS => {
                self.status
            }, AI_REG_DACRATE => {
                self.dacrate
            }, AI_REG_BITRATE => {
                self.bitrate
            }, _ => panic!("Read from unrecognized AI register address: {:#x}", reg)
        }
    }

    /* Writes to the AI's registers. */
    pub fn wreg(&mut self, reg: u32, value: u32) {
        match reg {
            AI_REG_DRAM_ADDR => {
                self.dram_addr = value
            }, AI_REG_LEN => {
                self.len = value
            }, AI_REG_CONTROL => {
                self.control = value
            }, AI_REG_STATUS => {
                self.status = value
            }, AI_REG_DACRATE => {
                self.dacrate = value
            }, AI_REG_BITRATE => {
                self.bitrate = value
            }, _ => panic!("Write to unrecognized AI register address: {:#x}", reg)
        }
    }
}
