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

/*

VI_BASE_REG - 0x04400000

        0x04400000 to 0x04400003  VI_STATUS_REG or VI_CONTROL_REG //VI status/control
           (RW): [1:0] type[0-1] (pixel size)
                     0: blank (no data, no sync)
                     1: reserved
                     2: 5/5/5/3 ("16" bit)
                     3: 8/8/8/8 (32 bit)
                 [2] gamma_dither_enable (normally on, unless "special effect")
                 [3] gamma_enable (normally on, unless MPEG/JPEG)
                 [4] divot_enable (normally on if antialiased,
                     unless decal lines)
                 [5] reserved - always off
                 [6] serrate (always on if interlaced, off if not)
                 [7] reserved - diagnostics only
                 [9:8] anti-alias (aa) mode[1:0]
                     0: aa & resamp (always fetch extra lines)
                     1: aa & resamp (fetch extra lines if needed)
                     2: resamp only (treat as all fully covered)
                     3: neither (replicate pixels, no interpolate)
                 [11] reserved - diagnostics only
                 [15:12] reserved

        0x04400004 to 0x04400007  VI_ORIGIN_REG or VI_DRAM_ADDR_REG //VI origin
           (RW): [23:0] frame buffer origin in bytes

        0x04400008 to 0x0440000B  VI_WIDTH_REG or VI_H_WIDTH_REG //VI width
           (RW): [11:0] frame buffer line width in pixels

        0x0440000C to 0x0440000F  VI_INTR_REG or VI_V_INTR_REG //VI vertical intr
           (RW): [9:0] interrupt when current half-line = V_INTR

        0x04400010 to 0x04400013  VI_CURRENT_REG or VI_V_CURRENT_LINE_REG //VI current vertical line
           (RW): [9:0] current half line, sampled once per line (the lsb of
                       V_CURRENT is constant within a field, and in
                       interlaced modes gives the field number - which is
                       constant for non-interlaced modes)
                       - Writes clears interrupt line

        0x04400014 to 0x04400017  VI_BURST_REG or VI_TIMING_REG //VI video timing
           (RW): [7:0] horizontal sync width in pixels
                 [15:8] color burst width in pixels
                 [19:16] vertical sync width in half lines
                 [29:20] start of color burst in pixels from h-sync

        0x04400018 to 0x0440001B  VI_V_SYNC_REG //VI vertical sync
           (RW): [9:0] number of half-lines per field

        0x0440001C to 0x0440001F  VI_H_SYNC_REG //VI horizontal sync
           (RW): [11:0] total duration of a line in 1/4 pixel
                 [20:16] a 5-bit leap pattern used for PAL only (h_sync_period)

        0x04400020 to 0x04400023  VI_LEAP_REG or VI_H_SYNC_LEAP_REG //VI horizontal sync leap
           (RW): [11:0] identical to h_sync_period
                 [27:16] identical to h_sync_period

        0x04400024 to 0x04400027  VI_H_START_REG or VI_H_VIDEO_REG //VI horizontal video
           (RW): [9:0] end of active video in screen pixels
                 [25:16] start of active video in screen pixels

        0x04400028 to 0x0440002B  VI_V_START_REG or VI_V_VIDEO_REG //VI vertical video
           (RW): [9:0] end of active video in screen half-lines
                 [25:16] start of active video in screen half-lines

        0x0440002C to 0x0440002F  VI_V_BURST_REG //VI vertical burst
           (RW): [9:0] end of color burst enable in half-lines
                 [25:16] start of color burst enable in half-lines

        0x04400030 to 0x04400033  VI_X_SCALE_REG //VI x-scale
           (RW): [11:0] 1/horizontal scale up factor (2.10 format)
                 [27:16] horizontal subpixel offset (2.10 format)

        0x04400034 to 0x04400037  VI_Y_SCALE_REG //VI y-scale
           (RW): [11:0] 1/vertical scale up factor (2.10 format)
                 [27:16] vertical subpixel offset (2.10 format)

        0x04400038 to 0x044FFFFF  Unused

*/

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
