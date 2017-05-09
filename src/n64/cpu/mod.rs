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

struct Inst(pub u32);

impl Inst {
	/* Returns the instruction's opcode. */
	pub fn op(&self) -> u8{
		((self.0 >> 26) & 0b111111) as u8
	}
	/* Returns the instruction's source register. */
	pub fn rs(&self) -> u8{
		((self.0 >> 21) & 0b11111) as u8
	}
	/* Returns the instruction's target register. */
	pub fn rt(&self) -> u8 {
		((self.0 >> 16) & 0b11111) as u8
	}
	/* Returns the instruciton's destination register. */
	pub fn rd(&self) -> u8 {
		((self.0 >> 11) & 0b11111) as u8
	}
	/* Returns the instruction's shift amount. */
	pub fn sa(&self) -> u8 {
		((self.0 >> 6) & 0b11111) as u8
	}
	/* Returns the instruction's immediate value. */
	pub fn imm(&self) -> u32 {
		self.0 & 0xffff
	}
	/* Return's the function's funct field. */
	pub fn funct(&self) -> u8 {
		0
	}
	/* Return's the instruction's target field. */
	pub fn target(&self) -> u32 {
		0
	}
}

/* Valid VR4300 opcodes. Figure 16-1 in NEC VR4300. */
enum Op {
	special,	regimm,		j,			jal,		beq,		bne,		blez,		bgtz,
	addi,		addiu,		slti,		sltiu,		andi,		ori,		xori,		lui,
	cop0,		cop1,		cop2,		/**/		beql,		bnel,		blezl,		bgtzl,
	daddi,		daddiu,		ldl,		ldr,		/**/		/**/		/**/		/**/
	lb,			lh,			lwl,		lw,			lbu,		lhu,		lwr,		lwu,
	sb,			sh,			swl,		sw,			sdl,		sdr,		swr,		cache,
	ll,			lwc1,		lwc2,		/**/		lld,		ldc1,		ldc2,		ld,
	sc,			swc1,		swc2,		/**/		scd,		sdc1,		sdc2,		sd,

	reserved
}

/* A constant 2-d array of the opcode values. */
static OpTable: [[Op; 8]; 8] = [
	[ Op::special,	Op::regimm,		Op::j,			Op::jal,		Op::beq,		Op::bne,		Op::blez,		Op::bgtz	 ],
	[ Op::addi,		Op::addiu,		Op::slti,		Op::sltiu,		Op::andi,		Op::ori,		Op::xori,		Op::lui		 ],
	[ Op::cop0,		Op::cop1,		Op::cop2,		Op::reserved,	Op::beql,		Op::bnel,		Op::blezl,		Op::bgtzl	 ],
	[ Op::daddi,	Op::daddiu,		Op::ldl,		Op::ldr,		Op::reserved,	Op::reserved,	Op::reserved,	Op::reserved ],
	[ Op::lb,		Op::lh,			Op::lwl,		Op::lw,			Op::lbu,		Op::lhu,		Op::lwr,		Op::lwu		 ],
	[ Op::sb,		Op::sh,			Op::swl,		Op::sw,			Op::sdl,		Op::sdr,		Op::swr,		Op::cache	 ],
	[ Op::ll,		Op::lwc1,		Op::lwc2,		Op::reserved,	Op::lld,		Op::ldc1,		Op::ldc2,		Op::ld		 ],
	[ Op::sc,		Op::swc1,		Op::swc2,		Op::reserved,	Op::scd,		Op::sdc1,		Op::sdc2,		Op::sd		 ],
];

/* Special operations. */
enum SpOp {
	sll,		/**/		srl,		sra,		sllv,		/**/		srlv,		srav,
	jr,			jalr,		/**/		/**/		syscall,	brk,		/**/		sync,
	mfhi,		mthi,		mflo,		mtlo,		dsllv,		/**/		dsrlv,		dsrav,
	mult,		multu,		div,		divu,		dmult,		dmultu,		ddiv,		ddivu,
	add,		addu,		sub,		subu,		and,		or,			xor,		nor,
	/**/		/**/		slt,		sltu,		dadd,		daddu,		dsub,		dsubu,
	tge,		tgeu,		tlt,		tltu,		teq,		/**/		tne,		/**/
	dsll,		/**/		dsrl,		dsra,		dsll32,		/**/		dsrl32,		dsra32,

	reserved
}

/* A constant 2-d array of the special function values. */
static SpOpTable: [[SpOp; 8]; 8] = [
	[ SpOp::sll,		SpOp::reserved,		SpOp::srl,			SpOp::sra,			SpOp::sllv,			SpOp::reserved,			SpOp::srlv,			SpOp::srav	 	],
	[ SpOp::jr,			SpOp::jalr,			SpOp::reserved,		SpOp::reserved,		SpOp::syscall,		SpOp::brk,				SpOp::reserved,		SpOp::sync	 	],
	[ SpOp::mfhi,		SpOp::mthi,			SpOp::mflo,			SpOp::mtlo,			SpOp::dsllv,		SpOp::reserved,			SpOp::dsrlv,		SpOp::dsrav	 	],
	[ SpOp::mult,		SpOp::multu,		SpOp::	div,		SpOp::divu,			SpOp::dmult,		SpOp::dmultu,			SpOp::ddiv,			SpOp::ddivu	 	],
	[ SpOp::add,		SpOp::addu,			SpOp::sub,			SpOp::subu,			SpOp::and,			SpOp::or,				SpOp::xor,			SpOp::nor	 	],
	[ SpOp::reserved,	SpOp::reserved,		SpOp::slt,			SpOp::sltu,			SpOp::dadd,			SpOp::daddu,			SpOp::dsub,			SpOp::dsubu	 	],
	[ SpOp::tge,		SpOp::tgeu,			SpOp::tlt,			SpOp::tltu,			SpOp::teq,			SpOp::reserved,			SpOp::tne,			SpOp::reserved	],
	[ SpOp::dsll,		SpOp::reserved,		SpOp::dsrl,			SpOp::dsra,			SpOp::dsll32,		SpOp::reserved,			SpOp::dsrl32,		SpOp::dsra32 	],
];

/* Register-Immediate operations */
enum RiOp {
	bltz,		bgez,		bltzl,		bgezl,		/**/		/**/		/**/		/**/
	tgei,		tgeiu,		tlti,		tltiu,		teqi,		/**/		tnei,		/**/
	bltzal,		bgezal,		bltzall,	bgezall,	/**/		/**/		/**/		/**/
	/**/		/**/		/**/		/**/		/**/		/**/		/**/		/**/

	reserved
}

/* A constant 2-d array of the register-immediate rt values. */
static RiOpTable: [[RiOp; 8]; 4] = [
	[ RiOp::bltz,		RiOp::bgez,			RiOp::bltzl,		RiOp::bgezl,		RiOp::reserved,		RiOp::reserved,		RiOp::reserved,		RiOp::reserved, ],
	[ RiOp::tgei,		RiOp::tgeiu,		RiOp::tlti,			RiOp::tltiu,		RiOp::teqi,			RiOp::reserved,		RiOp::tnei,			RiOp::reserved, ],
	[ RiOp::bltzal,		RiOp::bgezal,		RiOp::bltzall,		RiOp::bgezall,		RiOp::reserved,		RiOp::reserved,		RiOp::reserved,		RiOp::reserved, ],
	[ RiOp::reserved,	RiOp::reserved,		RiOp::reserved,		RiOp::reserved,		RiOp::reserved,		RiOp::reserved,		RiOp::reserved,		RiOp::reserved, ],
];


/* Executes a single instruction on the given N64. */
pub fn cycle(n64: &mut N64) {
	/* Fetch the next instrution from memory. */
	let inst = Inst(mc::read(n64, n64.cpu.pc as usize));
	println!("Cycle. Read raw instruction {:#x}.", inst.0);
	/* Execute the instrution. */
	match inst.op() {

		_ => return
	}
	/* Increment the program counter. */
	n64.cpu.pc += 4;
}
