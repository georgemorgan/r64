const VI_REG_STATUS: u32 = 0x0440_0000;
const VI_REG_ORIGIN: u32 = 0x0440_0004;
const VI_REG_WIDTH: u32 = 0x0440_0008;
const VI_REG_INTR: u32 = 0x0440_000C;
const VI_REG_CURRENT: u32 = 0x0440_0010;
const VI_REG_BURST: u32 = 0x0440_0014;
const VI_REG_V_SYNC: u32 = 0x0440_0018;
const VI_REG_H_SYNC: u32 = 0x0440_001C;
const VI_REG_LEAP: u32 = 0x0440_0020;
const VI_REG_H_START: u32 = 0x0440_0024;
const VI_REG_V_START: u32 = 0x0440_0028;
const VI_REG_V_BURST: u32 = 0x0440_002C;
const VI_REG_X_SCALE: u32 = 0x0440_0030;
const VI_REG_Y_SCALE: u32 = 0x0440_0034;

pub struct VI {
    status: u32,
    origin: u32,
    width: u32,
    intr: u32,
    current: u32,
    burst: u32,
    v_sync: u32,
    h_sync: u32,
    leap: u32,
    h_start: u32,
    v_start: u32,
    v_burst: u32,
    x_scale: u32,
    y_scale: u32
}

impl VI {
    pub fn new() -> VI {
        VI {
            status: 0,
            origin: 0,
            width: 0,
            intr: 0,
            current: 0,
            burst: 0,
            v_sync: 0,
            h_sync: 0,
            leap: 0,
            h_start: 0,
            v_start: 0,
            v_burst: 0,
            x_scale: 0,
            y_scale: 0
        }
    }

    /* Reads from the PI's registers. */
    pub fn rreg(&self, reg: u32) -> u32 {
        match reg {
            VI_REG_STATUS => {
                self.status
            }, VI_REG_ORIGIN => {
                self.origin
            }, VI_REG_WIDTH => {
                self.width
            }, VI_REG_INTR => {
                self.intr
            }, VI_REG_CURRENT => {
                self.current
            }, VI_REG_BURST => {
                self.burst
            }, VI_REG_V_SYNC => {
                self.v_sync
            }, VI_REG_H_SYNC => {
                self.h_sync
            }, VI_REG_LEAP => {
                self.leap
            }, VI_REG_H_START => {
                self.h_start
            }, VI_REG_V_START => {
                self.v_start
            }, VI_REG_V_BURST => {
                self.v_burst
            }, VI_REG_X_SCALE => {
                self.x_scale
            }, VI_REG_Y_SCALE => {
                self.y_scale
            }, _ => panic!("Read from unrecognized PI register address: {:#x}", reg)
        }
    }

    /* Writes to the PI's registers. */
    pub fn wreg(&mut self, reg: u32, value: u32) {
        match reg {
            VI_REG_STATUS => {
                self.status = value
            }, VI_REG_ORIGIN => {
                self.origin = value
            }, VI_REG_WIDTH => {
                self.width = value
            }, VI_REG_INTR => {
                self.intr = value
            }, VI_REG_CURRENT => {
                self.current = value
            }, VI_REG_BURST => {
                self.burst = value
            }, VI_REG_V_SYNC => {
                self.v_sync = value
            }, VI_REG_H_SYNC => {
                self.h_sync = value
            }, VI_REG_LEAP => {
                self.leap = value
            }, VI_REG_H_START => {
                self.h_start = value
            }, VI_REG_V_START => {
                self.v_start = value
            }, VI_REG_V_BURST => {
                self.v_burst = value
            }, VI_REG_X_SCALE => {
                self.x_scale = value
            }, VI_REG_Y_SCALE => {
                self.y_scale = value
            }, _ => panic!("Write to unrecognized PI register address: {:#x}", reg)
        }
    }
}
