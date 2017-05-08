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

 use n64::N64;
 use n64::mc;

/* Size of the general purpose register file. */
const GPR_SIZE: usize = 32;

pub struct CPU {

	/* The CPU's register file. */
	gpr: [u32; GPR_SIZE],

	/* The program counter. */
	pc: u32
}

impl CPU {
	pub fn new(pc: u32) -> CPU {
		CPU {
			/* Zero-initialize the general purpose registers. */
			gpr: [0; GPR_SIZE],
			/* Initialize the program counter. */
			pc: pc
		}
	}
}

/* Executes a single instruction on the given N64. */
pub fn cycle(n64: &mut N64) {
	/* Fetch the next instrution from memory. */
	let inst = mc::read(n64, n64.cpu.pc as usize);
	println!("Cycle. Read raw instruction {:#x}.", inst);
	/* Execute the instrution. */

	/* Increment the program counter. */
	n64.cpu.pc += 4;
}
