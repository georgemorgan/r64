/* Exposes datatypes and functionaly of the VR4300 CPU. */

/*

CPU0

  00h = r0/reg0     08h = t0/reg8     10h = s0/reg16    18h = t8/reg24
  01h = at/reg1     09h = t1/reg9     11h = s1/reg17    19h = t9/reg25
  02h = v0/reg2     0Ah = t2/reg10    12h = s2/reg18    1Ah = k0/reg26
  03h = v1/reg3     0Bh = t3/reg11    13h = s3/reg19    1Bh = k1/reg27
  04h = a0/reg4     0Ch = t4/reg12    14h = s4/reg20    1Ch = gp/reg28
  05h = a1/reg5     0Dh = t5/reg13    15h = s5/reg21    1Dh = sp/reg29
  06h = a2/reg6     0Eh = t6/reg14    16h = s6/reg22    1Eh = s8/reg30
  07h = a3/reg7     0Fh = t7/reg15    17h = s7/reg23    1Fh = ra/reg31

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

pub struct CPU {
	/* The CPU's register file. */
    r0: u32,
    at: u32,
    v0: u32,
    v1: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    t0: u32,
    t1: u32,
    t2: u32,
    t3: u32,
    t4: u32,
    t5: u32,
    t6: u32,
    t7: u32,
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    s4: u32,
    s5: u32,
    s6: u32,
    s7: u32,
    t8: u32,
    t9: u32,
    k0: u32,
    k1: u32,
    gp: u32,
    sp: u32,
    s8: u32,
    ra: u32,

    /* The program counter. */
    pc: u32
}

impl CPU {
	pub fn new() -> CPU {
		CPU {
			/* Zero-initialize the registers. */
		    r0: 0,
		    at: 0,
		    v0: 0,
		    v1: 0,
		    a0: 0,
		    a1: 0,
		    a2: 0,
		    a3: 0,
		    t0: 0,
		    t1: 0,
		    t2: 0,
		    t3: 0,
		    t4: 0,
		    t5: 0,
		    t6: 0,
		    t7: 0,
		    s0: 0,
		    s1: 0,
		    s2: 0,
		    s3: 0,
		    s4: 0,
		    s5: 0,
		    s6: 0,
		    s7: 0,
		    t8: 0,
		    t9: 0,
		    k0: 0,
		    k1: 0,
		    gp: 0,
		    sp: 0,
		    s8: 0,
		    ra: 0,
			/* Initialize the program counter. */
		    pc: 0
		}
	}
	/* Executes a single instruction. */
	pub fn cycle(&mut self) {
		/* Increment the program counter. */
		self.pc += 4;
		/* Fetch the next instrution from memory. */

	}
}
