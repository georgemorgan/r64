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

/* Valid VR4300 opcodes. Figure 16-1 in NEC VR4300. */
#[derive(Copy, Clone)]
enum Op {
	Special,	RegImm,		J,			Jal,		Beq,		Bne,		Blez,		Bgtz,
	Addi,		Addiu,		Slti,		Sltiu,		Andi,		Ori,		Xori,		Lui,
	Cop0,		Cop1,		Cop2,		/**/		Beql,		Bnel,		Blezl,		Bgtzl,
	Daddi,		Daddiu,		Ldl,		Ldr,		/**/		/**/		/**/		/**/
	Lb,			Lh,			Lwl,		Lw,			Lbu,		Lhu,		Lwr,		Lwu,
	Sb,			Sh,			Swl,		Sw,			Sdl,		Sdr,		Swr,		Cache,
	Ll,			Lwc1,		Lwc2,		/**/		Lld,		Ldc1,		Ldc2,		Ld,
	Sc,			Swc1,		Swc2,		/**/		Scd,		Sdc1,		Sdc2,		Sd,

	Reserved
}

/* A static 2-d array of the opcode names. */
const OP_NAMES: [[&'static str; 8]; 8] = [
	[ "special",	"regimm",		"j",		"jal",			"beq",			"bne",			"blez",			"bgtz"		],
	[ "addi",		"addiu",		"slti",		"sltiu",		"andi",			"ori",			"xori",			"lui"		],
	[ "cop0",		"cop1",			"cop2",		"reserved",		"beql",			"bnel",			"blezl",		"bgtzl"		],
	[ "daddi",		"daddiu",		"ldl",		"ldr",			"reserved",		"reserved",		"reserved",		"reserved"	],
	[ "lb",			"lh",			"lwl",		"lw",			"lbu",			"lhu",			"lwr",			"lwu"		],
	[ "sb",			"sh",			"swl",		"sw",			"sdl",			"sdr",			"swr",			"cache"		],
	[ "ll",			"lwc1",			"lwc2",		"reserved",		"lld",			"ldc1",			"ldc2",			"ld"		],
	[ "sc",			"swc1",			"swc2",		"reserved",		"scd",			"sdc1",			"sdc2",			"sd"		],
];

/* A constant 2-d array of the opcode values. */
static OP_TABLE: [[Op; 8]; 8] = [
	[ Op::Special,	Op::RegImm,		Op::J,			Op::Jal,		Op::Beq,		Op::Bne,		Op::Blez,		Op::Bgtz	 ],
	[ Op::Addi,		Op::Addiu,		Op::Slti,		Op::Sltiu,		Op::Andi,		Op::Ori,		Op::Xori,		Op::Lui		 ],
	[ Op::Cop0,		Op::Cop1,		Op::Cop2,		Op::Reserved,	Op::Beql,		Op::Bnel,		Op::Blezl,		Op::Bgtzl	 ],
	[ Op::Daddi,	Op::Daddiu,		Op::Ldl,		Op::Ldr,		Op::Reserved,	Op::Reserved,	Op::Reserved,	Op::Reserved ],
	[ Op::Lb,		Op::Lh,			Op::Lwl,		Op::Lw,			Op::Lbu,		Op::Lhu,		Op::Lwr,		Op::Lwu		 ],
	[ Op::Sb,		Op::Sh,			Op::Swl,		Op::Sw,			Op::Sdl,		Op::Sdr,		Op::Swr,		Op::Cache	 ],
	[ Op::Ll,		Op::Lwc1,		Op::Lwc2,		Op::Reserved,	Op::Lld,		Op::Ldc1,		Op::Ldc2,		Op::Ld		 ],
	[ Op::Sc,		Op::Swc1,		Op::Swc2,		Op::Reserved,	Op::Scd,		Op::Sdc1,		Op::Sdc2,		Op::Sd		 ],
];

/* Special operations. */
#[derive(Copy, Clone)]
enum SpOp {
	Sll,		/**/		Srl,		Sra,		Sllv,		/**/		Srlv,		Srav,
	Jr,			Jalr,		/**/		/**/		Syscall,	Brk,		/**/		Sync,
	Mfhi,		Mthi,		Mflo,		Mtlo,		Dsllv,		/**/		Dsrlv,		Dsrav,
	Mult,		Multu,		Div,		Divu,		Dmult,		Dmultu,		Ddiv,		Ddivu,
	Add,		Addu,		Sub,		Subu,		And,		Or,			Xor,		Nor,
	/**/		/**/		Slt,		Sltu,		Dadd,		Daddu,		Dsub,		Dsubu,
	Tge,		Tgeu,		Tlt,		Tltu,		Teq,		/**/		Tne,		/**/
	Dsll,		/**/		Dsrl,		Dsra,		Dsll32,		/**/		Dsrl32,		Dsra32,

	Reserved
}

/* A static 2-d array of the opcode names. */
const SP_OP_NAMES: [[&'static str; 8]; 8] = [
	[ "sll",		"reserved",		"srl",			"sra",			"sllv",			"reserved",		"srlv",			"srav"		],
	[ "jr",			"jalr",			"reserved",		"reserved",		"syscall",		"brk",			"reserved",		"sync"		],
	[ "mfhi",		"mthi",			"mflo",			"mtlo",			"dsllv",		"reserved",		"dsrlv",		"dsrav"		],
	[ "mult",		"multu",		"div",			"divu",			"dmult",		"dmultu",		"ddiv",			"ddivu"		],
	[ "add",		"addu",			"sub",			"subu",			"and",			"or",			"xor",			"nor"		],
	[ "reserved",	"reserved",		"slt",			"sltu",			"dadd",			"daddu",		"dsub",			"dsubu"		],
	[ "tge",		"tgeu",			"tlt",			"tltu",			"teq",			"reserved",		"tne",			"reserved"	],
	[ "dsll",		"reserved",		"dsrl",			"dsra",			"dsll32",		"reserved",		"dsrl32",		"dsra32"	],
];

/* A constant 2-d array of the special function values. */
static SP_OP_TABLE: [[SpOp; 8]; 8] = [
	[ SpOp::Sll,		SpOp::Reserved,		SpOp::Srl,			SpOp::Sra,			SpOp::Sllv,			SpOp::Reserved,			SpOp::Srlv,			SpOp::Srav	 	],
	[ SpOp::Jr,			SpOp::Jalr,			SpOp::Reserved,		SpOp::Reserved,		SpOp::Syscall,		SpOp::Brk,				SpOp::Reserved,		SpOp::Sync	 	],
	[ SpOp::Mfhi,		SpOp::Mthi,			SpOp::Mflo,			SpOp::Mtlo,			SpOp::Dsllv,		SpOp::Reserved,			SpOp::Dsrlv,		SpOp::Dsrav	 	],
	[ SpOp::Mult,		SpOp::Multu,		SpOp::Div,			SpOp::Divu,			SpOp::Dmult,		SpOp::Dmultu,			SpOp::Ddiv,			SpOp::Ddivu	 	],
	[ SpOp::Add,		SpOp::Addu,			SpOp::Sub,			SpOp::Subu,			SpOp::And,			SpOp::Or,				SpOp::Xor,			SpOp::Nor	 	],
	[ SpOp::Reserved,	SpOp::Reserved,		SpOp::Slt,			SpOp::Sltu,			SpOp::Dadd,			SpOp::Daddu,			SpOp::Dsub,			SpOp::Dsubu	 	],
	[ SpOp::Tge,		SpOp::Tgeu,			SpOp::Tlt,			SpOp::Tltu,			SpOp::Teq,			SpOp::Reserved,			SpOp::Tne,			SpOp::Reserved	],
	[ SpOp::Dsll,		SpOp::Reserved,		SpOp::Dsrl,			SpOp::Dsra,			SpOp::Dsll32,		SpOp::Reserved,			SpOp::Dsrl32,		SpOp::Dsra32 	],
];

/* Register-Immediate operations */
#[derive(Copy, Clone)]
enum RiOp {
	Bltz,		Bgez,		Bltzl,		Bgezl,		/**/		/**/		/**/		/**/
	Tgei,		Tgeiu,		Tlti,		Tltiu,		Teqi,		/**/		Tnei,		/**/
	Bltzal,		Bgezal,		Bltzall,	Bgezall,	/**/		/**/		/**/		/**/
	/**/		/**/		/**/		/**/		/**/		/**/		/**/		/**/

	Reserved
}

/* A constant 2-d array of the register-immediate rt values. */
static RiOP_TABLE: [[RiOp; 8]; 4] = [
	[ RiOp::Bltz,		RiOp::Bgez,			RiOp::Bltzl,		RiOp::Bgezl,		RiOp::Reserved,		RiOp::Reserved,		RiOp::Reserved,		RiOp::Reserved, ],
	[ RiOp::Tgei,		RiOp::Tgeiu,		RiOp::Tlti,			RiOp::Tltiu,		RiOp::Teqi,			RiOp::Reserved,		RiOp::Tnei,			RiOp::Reserved, ],
	[ RiOp::Bltzal,		RiOp::Bgezal,		RiOp::Bltzall,		RiOp::Bgezall,		RiOp::Reserved,		RiOp::Reserved,		RiOp::Reserved,		RiOp::Reserved, ],
	[ RiOp::Reserved,	RiOp::Reserved,		RiOp::Reserved,		RiOp::Reserved,		RiOp::Reserved,		RiOp::Reserved,		RiOp::Reserved,		RiOp::Reserved, ],
];

struct Inst(pub u32);

impl Inst {
	/* Returns the instruction's opcode. */
	pub fn opcode(&self) -> u8{
		((self.0 >> 26) & 0b111111) as u8
	}
	/* Returns the enumerated operation type. */
	pub fn op(&self) -> Op {
		OP_TABLE[((self.opcode() >> 3) & 0b111) as usize][(self.opcode() & 0b111) as usize]
	}
	/* Returns a string of the opcode for debugging. */
	pub fn op_str(&self) -> &str {
		match self.op() {
			Op::Special =>
				self.sp_op_str(),
			_ =>
				OP_NAMES[((self.opcode() >> 3) & 0b111) as usize][(self.opcode() & 0b111) as usize]
		}
	}
	/* Returns the instruction's source register. */
	pub fn rs(&self) -> usize {
		((self.0 >> 21) & 0b11111) as usize
	}
	/* Returns the instruction's target register. */
	pub fn rt(&self) -> usize {
		((self.0 >> 16) & 0b11111) as usize
	}
	/* Returns the instruciton's destination register. */
	pub fn rd(&self) -> usize {
		((self.0 >> 11) & 0b11111) as usize
	}
	/* Returns the instruction's shift amount. */
	pub fn sa(&self) -> u8 {
		((self.0 >> 6) & 0b11111) as u8
	}
	/* Returns the instruction's immediate value. */
	pub fn imm(&self) -> u64 {
		(self.0 & 0xffff) as u64
	}
	/* Return's the function's funct field. */
	pub fn funct(&self) -> u8 {
		(self.0 & 0b11111) as u8
	}
	/* Returns the instruction's special operation. */
	pub fn sp_op(&self) -> SpOp {
		SP_OP_TABLE[((self.funct() >> 3) & 0b111) as usize][(self.funct() & 0b111) as usize]
	}
	/* Returns a string of the special function for debugging. */
	pub fn sp_op_str(&self) -> &str {
		SP_OP_NAMES[((self.funct() >> 3) & 0b111) as usize][(self.funct() & 0b111) as usize]
	}
	/* Return's the instruction's target field. */
	pub fn target(&self) -> u32 {
		0
	}
}

/* Size of the general purpose register file. */
const GPR_SIZE: usize = 32;

pub struct CPU {
	/* The CPU's register file. */
	gpr: [u64; GPR_SIZE],
	/* The program counter. */
	pc: u64
}

const GPR_NAMES: [&'static str; GPR_SIZE] = [
	"r0", "at", "v0", "v1", "a0", "a1", "a2", "a3",
	"t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
	"s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
	"t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra",
];

impl CPU {
	pub fn new(pc: u64) -> CPU {
		CPU {
			/* Zero-initialize the general purpose registers. */
			gpr: [0; GPR_SIZE],
			/* Initialize the program counter. */
			pc: pc
		}
	}
	/* Reads a value from the specified GPR. */
	fn rgpr(&self, reg: usize) -> u64 {
		self.gpr[reg]
	}
	/* Writes a value to the specified GPR. */
	fn wgpr(&mut self, val: u64, reg: usize) {
		self.gpr[reg] = val;
	}

	/* Handlers for the 3 instruction formats. - Chapter 3.1 in NEC VR4300 manual. */

	/* Handler for the immediate (I-Type) instructions. */
	fn exec_imm<F>(&mut self, i: Inst, f: F) where F: FnOnce(u64, u64) -> u64 {
		/* Obtain the value stored in the rs register. */
		let rs = self.rgpr(i.rs());
		/* Obtain the immediate value. */
		let imm = i.imm();
		/* Perform the operation. */
		let rt = f(rs, imm);
		/* Write the result back into the target register. */
		self.wgpr(rt, i.rt());

	}
	/* Handler for the jump (J-Type) instructions. */
	fn exec_jump<F>(&mut self, i: Inst, f: F) where F: FnOnce(u64, u64) -> u64 {

	}
	/* Handler for the register (R-Type) instructions. */
	fn exec_reg<F>(&mut self, i: Inst, f: F) where F: FnOnce(u64, u64) -> u64 {
		/* Obtain the value stored in the rs and rt registers. */
		let rs = self.rgpr(i.rs());
		let rt = self.rgpr(i.rt());
		/* Perform the operation. */
		let rd = f(rs, rt);
		/* Write the result back into the destination register. */
		self.wgpr(rd, i.rd());
	}

	/* Executes an instruction. */
	pub fn exec(&mut self, i: Inst) {
		/* Print the opcode. */
		println!("{:#x}: {:?}", self.pc, i.op_str());
		/* Execute the instrution. */
		match i.op() {
			Op::Special =>
				/* Match against the instruction's special operation. */
				match i.sp_op() {
					_ => panic!("Attempt made to execute an unknown or unimplemented special function. {:#x}.", i.funct()),
				},
			Op::RegImm =>
				(),
			Op::J =>
				(),
			Op::Jal =>
				(),
			Op::Beq =>
				(),
			Op::Bne =>
				(),
			Op::Blez =>
				(),
			Op::Bgtz =>
				(),
			Op::Addi =>
				(),
			Op::Addiu =>
				(),
			Op::Slti =>
				(),
			Op::Sltiu =>
				(),
			Op::Andi =>
				(),
			Op::Ori =>
				(),
			Op::Xori =>
				(),
			Op::Lui =>
				(),
			Op::Cop0 =>
				(),
			Op::Cop1 =>
				(),
			Op::Cop2 =>
				(),
			Op::Beql =>
				(),
			Op::Bnel =>
				(),
			Op::Blezl =>
				(),
			Op::Bgtzl =>
				(),
			Op::Daddi =>
				(),
			Op::Daddiu =>
				(),
			Op::Ldl =>
				(),
			Op::Ldr =>
				(),
			Op::Lb =>
				(),
			Op::Lh =>
				(),
			Op::Lwl =>
				(),
			Op::Lw =>
				(),
			Op::Lbu =>
				(),
			Op::Lhu =>
				(),
			Op::Lwr =>
				(),
			Op::Lwu =>
				(),
			Op::Sb =>
				(),
			Op::Sh =>
				(),
			Op::Swl =>
				(),
			Op::Sw =>
				(),
			Op::Sdl =>
				(),
			Op::Sdr =>
				(),
			Op::Swr =>
				(),
			Op::Cache =>
				(),
			Op::Ll =>
				(),
			Op::Lwc1 =>
				(),
			Op::Lwc2 =>
				(),
			Op::Lld =>
				(),
			Op::Ldc1 =>
				(),
			Op::Ldc2 =>
				(),
			Op::Ld =>
				(),
			Op::Sc =>
				(),
			Op::Swc1 =>
				(),
			Op::Swc2 =>
				(),
			Op::Scd =>
				(),
			Op::Sdc1 =>
				(),
			Op::Sdc2 =>
				(),
			Op::Sd =>
				(),
			Op::Reserved => panic!("Attempt made to execute a reserved instruction {:#x}.", i.opcode()),
			_ => panic!("Attempt made to execute an unknown or unimplemented opcode. {:#x}.", i.opcode()),
		}
		/* Increment the program counter. */
		self.pc += 4;
	}
}

/* Executes a single instruction on the given N64. */
pub fn cycle(n64: &mut N64) {
	/* Fetch the next instrution from memory. */
	let inst = Inst(mc::read(n64, n64.cpu.pc as usize));
	/* Execute the instruction. */
	n64.cpu.exec(inst);
}
