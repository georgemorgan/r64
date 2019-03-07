/* cp0.rs - The CP0 (Co-Processor 0) module. */

/*

COP0

  00h = Index       08h = BadVAddr    10h = Config      18h = *RESERVED*
  01h = Random      09h = Count       11h = LLAddr      19h = *RESERVED*
  02h = EntryLo0    0Ah = EntryHi     12h = WatchLo     1Ah = PErr
  03h = EntryLo1    0Bh = Compare     13h = WatchHi     1Bh = CacheErr
  04h = Context     0Ch = Status      14h = XContext    1Ch = TagLo
  05h = PageMask    0Dh = Cause       15h = *RESERVED*  1Dh = TagHi
  06h = Wired       0Eh = EPC         16h = *RESERVED*  1Eh = ErrorEPC
  07h = *RESERVED*  0Fh = PRevID      17h = *RESERVED*  1Fh = *RESERVED*

*/

use n64::cpu::GPR_SIZE;

const CP0_CONFIG: usize = 0x10;

pub struct CP0 {
    /* the cop0 general purpose registers */
    regs: [u64; GPR_SIZE],
}

impl CP0 {
    
    pub fn new() -> CP0 {
        CP0 {
            /* zero-initialize the cop0 registers */
            regs: [0; GPR_SIZE]
        }
    }

    /* reads from a cop0 register */
    pub fn rd(&self, reg: usize) -> u64 {
        self.regs[reg]
    }

    /* writes to a cop0 register */
    pub fn wr(&mut self, val: u64, reg: usize) {
        self.regs[reg] = val;
    }

}
