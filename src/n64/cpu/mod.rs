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

#[derive(Copy, Clone)]
/* Possible opcode classes. */
enum OpC {
	I, J, R
}

/* Valid VR4300 opcodes. Figure 16-1 in NEC VR4300. */
#[derive(Copy, Clone)]
enum Op {

	/* Unique Opcodes */

	Special,	RegImm,		J,			Jal,		Beq,		Bne,		Blez,		Bgtz,
	Addi,		Addiu,		Slti,		Sltiu,		Andi,		Ori,		Xori,		Lui,
	Cop0,		Cop1,		Cop2,		/**/		Beql,		Bnel,		Blezl,		Bgtzl,
	Daddi,		Daddiu,		Ldl,		Ldr,		/**/		/**/		/**/		/**/
	Lb,			Lh,			Lwl,		Lw,			Lbu,		Lhu,		Lwr,		Lwu,
	Sb,			Sh,			Swl,		Sw,			Sdl,		Sdr,		Swr,		Cache,
	Ll,			Lwc1,		Lwc2,		/**/		Lld,		Ldc1,		Ldc2,		Ld,
	Sc,			Swc1,		Swc2,		/**/		Scd,		Sdc1,		Sdc2,		Sd,

	/* Special Opcodes */

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

type OpF = &'static Fn(u64, u64) -> u64;
type OpTup = (Op, &'static str, OpC, OpF);
/* A constant 2-d array of the opcode values. */
const OP_TABLE: [[OpTup; 8]; 8] = [
	[(Op::Special,  "special",  OpC::I, &|_, _| 0),
	 (Op::RegImm,   "regimm",   OpC::I, &|_, _| 0),
	 (Op::J,        "j",        OpC::I, &|_, _| 0),
	 (Op::Jal,      "jal",      OpC::I, &|_, _| 0),
	 (Op::Beq,      "beq",      OpC::I, &|_, _| 0),
	 (Op::Bne,      "bne",      OpC::I, &|_, _| 0),
	 (Op::Blez,     "blez",     OpC::I, &|_, _| 0),
	 (Op::Bgtz,     "bgtz",     OpC::I, &|_, _| 0)
	],
	[(Op::Addi,     "addi",     OpC::I, &|_, _| 0),
	 (Op::Addiu,    "addiu",    OpC::I, &|_, _| 0),
	 (Op::Slti,     "slti",     OpC::I, &|_, _| 0),
	 (Op::Sltiu,    "sltiu",    OpC::I, &|_, _| 0),
	 (Op::Andi,     "andi",     OpC::I, &|_, _| 0),
	 (Op::Ori,      "ori",      OpC::I, &|_, _| 0),
	 (Op::Xori,     "xori",     OpC::I, &|_, _| 0),
	 (Op::Lui,      "lui",      OpC::I, &|_, _| 0)],
	[(Op::Cop0,     "cop0",     OpC::I, &|_, _| 0),
	 (Op::Cop1,     "cop1",     OpC::I, &|_, _| 0),
	 (Op::Cop2,     "cop2",     OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Beql,     "beql",     OpC::I, &|_, _| 0),
	 (Op::Bnel,     "bnel",     OpC::I, &|_, _| 0),
	 (Op::Blezl,    "blezl",    OpC::I, &|_, _| 0),
	 (Op::Bgtzl,    "bgtzl",    OpC::I, &|_, _| 0)],
	[(Op::Daddi,    "daddi",    OpC::I, &|_, _| 0),
	 (Op::Daddiu,   "daddiu",   OpC::I, &|_, _| 0),
	 (Op::Ldl,      "ldl",      OpC::I, &|_, _| 0),
	 (Op::Ldr,      "ldr",      OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0)],
	[(Op::Lb,       "lb",       OpC::I, &|_, _| 0),
	 (Op::Lh,       "lh",       OpC::I, &|_, _| 0),
	 (Op::Lwl,      "lwl",      OpC::I, &|_, _| 0),
	 (Op::Lw,       "lw",       OpC::I, &|_, _| 0),
	 (Op::Lbu,      "lbu",      OpC::I, &|_, _| 0),
	 (Op::Lhu,      "lhu",      OpC::I, &|_, _| 0),
	 (Op::Lwr,      "lwr",      OpC::I, &|_, _| 0),
	 (Op::Lwu,      "lwu",      OpC::I, &|_, _| 0)],
	[(Op::Sb,       "sb",       OpC::I, &|_, _| 0),
	 (Op::Sh,       "sh",       OpC::I, &|_, _| 0),
	 (Op::Swl,      "swl",      OpC::I, &|_, _| 0),
	 (Op::Sw,       "sw",       OpC::I, &|_, _| 0),
	 (Op::Sdl,      "sdl",      OpC::I, &|_, _| 0),
	 (Op::Sdr,      "sdr",      OpC::I, &|_, _| 0),
	 (Op::Swr,      "swr",      OpC::I, &|_, _| 0),
	 (Op::Cache,    "cache",    OpC::I, &|_, _| 0)],
	[(Op::Ll,       "ll",       OpC::I, &|_, _| 0),
	 (Op::Lwc1,     "lwc1",     OpC::I, &|_, _| 0),
	 (Op::Lwc2,     "lwc2",     OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Lld,      "lld",      OpC::I, &|_, _| 0),
	 (Op::Ldc1,     "ldc1",     OpC::I, &|_, _| 0),
	 (Op::Ldc2,     "ldc2",     OpC::I, &|_, _| 0),
	 (Op::Ld,       "ld",       OpC::I, &|_, _| 0)],
	[(Op::Sc,       "sc",       OpC::I, &|_, _| 0),
	 (Op::Swc1,     "swc1",     OpC::I, &|_, _| 0),
	 (Op::Swc2,     "swc2",     OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Scd,      "scd",      OpC::I, &|_, _| 0),
	 (Op::Sdc1,     "sdc1",     OpC::I, &|_, _| 0),
	 (Op::Sdc2,     "sdc2",     OpC::I, &|_, _| 0),
	 (Op::Sd,       "sd",       OpC::I, &|_, _| 0)],
];

/* A constant 2-d array of the opcode values. */
const SP_OP_TABLE: [[OpTup; 8]; 8] = [
	[(Op::Sll,      "sll",      OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Srl,      "srl",      OpC::I, &|_, _| 0),
	 (Op::Sra,      "sra",      OpC::I, &|_, _| 0),
	 (Op::Sllv,     "sllv",     OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Srlv,     "srlv",     OpC::I, &|_, _| 0),
	 (Op::Srav,     "srav",     OpC::I, &|_, _| 0)],
	[(Op::Jr,       "jr",       OpC::I, &|_, _| 0),
	 (Op::Jalr,     "jalr",     OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Syscall,  "syscall",  OpC::I, &|_, _| 0),
	 (Op::Brk,      "brk",      OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Sync,     "sync",     OpC::I, &|_, _| 0)],
	[(Op::Mfhi,     "mfhi",     OpC::I, &|_, _| 0),
	 (Op::Mthi,     "mthi",     OpC::I, &|_, _| 0),
	 (Op::Mflo,     "mflo",     OpC::I, &|_, _| 0),
	 (Op::Mtlo,     "mtlo",     OpC::I, &|_, _| 0),
	 (Op::Dsllv,    "dsllv",    OpC::I, &|_, _| 0),
	 (Op::Reserved, "resered",  OpC::I, &|_, _| 0),
	 (Op::Dsrlv,    "dsrlv",    OpC::I, &|_, _| 0),
	 (Op::Dsrav,    "dsrav",    OpC::I, &|_, _| 0)],
	[(Op::Mult,     "mult",     OpC::I, &|_, _| 0),
	 (Op::Multu,    "multu",    OpC::I, &|_, _| 0),
	 (Op::Div,      "div",      OpC::I, &|_, _| 0),
	 (Op::Divu,     "divu",     OpC::I, &|_, _| 0),
	 (Op::Dmult,    "dmult",    OpC::I, &|_, _| 0),
	 (Op::Dmultu,    "dmultu",  OpC::I, &|_, _| 0),
	 (Op::Ddiv,     "ddiv",     OpC::I, &|_, _| 0),
	 (Op::Ddivu	,   "ddivu",    OpC::I, &|_, _| 0)],
	[(Op::Add,      "add",      OpC::I, &|_, _| 0),
	 (Op::Addu,     "addu",     OpC::I, &|_, _| 0),
	 (Op::Sub,      "sub",      OpC::I, &|_, _| 0),
	 (Op::Subu,     "subu",     OpC::I, &|_, _| 0),
	 (Op::And,      "and",      OpC::I, &|_, _| 0),
	 (Op::Or,       "or",       OpC::I, &|_, _| 0),
	 (Op::Xor,      "xor",      OpC::I, &|_, _| 0),
	 (Op::Nor,      "nor",      OpC::I, &|_, _| 0)],
	[(Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Slt,      "slt",      OpC::I, &|_, _| 0),
	 (Op::Sltu,     "sltu",     OpC::I, &|_, _| 0),
	 (Op::Dadd,     "dadd",     OpC::I, &|_, _| 0),
	 (Op::Daddu,    "daddu",    OpC::I, &|_, _| 0),
	 (Op::Dsub,     "dsub",     OpC::I, &|_, _| 0),
	 (Op::Dsubu,    "dsubu",    OpC::I, &|_, _| 0)],
	[(Op::Tge,      "tge",      OpC::I, &|_, _| 0),
	 (Op::Tgeu,     "tgeu",     OpC::I, &|_, _| 0),
	 (Op::Tlt,      "tlt",      OpC::I, &|_, _| 0),
	 (Op::Tltu,     "tltu",     OpC::I, &|_, _| 0),
	 (Op::Teq,      "teq",      OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Tne,      "tne",      OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0)],
	[(Op::Dsll,     "dsll",     OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Dsrl,     "dsrl",     OpC::I, &|_, _| 0),
	 (Op::Dsra,     "dsra",     OpC::I, &|_, _| 0),
	 (Op::Dsll32,   "dsll32",   OpC::I, &|_, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _| 0),
	 (Op::Dsrl32,   "dsrl32",   OpC::I, &|_, _| 0),
	 (Op::Dsra32,   "dsra32",   OpC::I, &|_, _| 0)],
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
	/* Returns the opcode's tuple. */
	fn op_tup(&self) -> OpTup {
		let t = OP_TABLE[((self.opcode() >> 3) & 0b111) as usize][(self.opcode() & 0b111) as usize];
		match t.0 {
			Op::Special =>
				SP_OP_TABLE[((self.funct() >> 3) & 0b111) as usize][(self.funct() & 0b111) as usize],
			_ =>
				t,
		}
	}
	/* Returns the enumerated operation type. */
	pub fn op(&self) -> Op {
		self.op_tup().0
	}
	/* Returns a string of the opcode for debugging. */
	pub fn op_str(&self) -> &str {
		self.op_tup().1
	}
	/* Returns the class of the opcode. */
	pub fn class(&self) -> OpC {
		self.op_tup().2
	}
	/* Returns the opcode's function. */
	pub fn function(&self) -> OpF {
		self.op_tup().3
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
	fn exec_imm(&mut self, i: Inst) {
		/* Obtain the value stored in the rs register. */
		let rs = self.rgpr(i.rs());
		/* Obtain the immediate value. */
		let imm = i.imm();
		/* Perform the operation. */
		let rt = i.function()(rs, imm);
		/* Write the result back into the target register. */
		self.wgpr(rt, i.rt());

	}
	/* Handler for the jump (J-Type) instructions. */
	fn exec_jump(&mut self, i: Inst) {

	}
	/* Handler for the register (R-Type) instructions. */
	fn exec_reg(&mut self, i: Inst) {
		/* Obtain the value stored in the rs and rt registers. */
		let rs = self.rgpr(i.rs());
		let rt = self.rgpr(i.rt());
		/* Perform the operation. */
		let rd = i.function()(rs, rt);
		/* Write the result back into the destination register. */
		self.wgpr(rd, i.rd());
	}

	/* Executes an instruction. */
	pub fn exec(&mut self, i: Inst) {
		/* Print the opcode. */
		println!("{:#x}: {:?}", self.pc, i.op_str());

		/* Execute the instrution. */
		match i.op() {
			Op::Reserved => panic!("Attempt made to execute a reserved instruction {:#x}.", i.opcode()),
			_ => match i.class() {
				OpC::I =>
					self.exec_imm(i),
				OpC::J =>
					self.exec_jump(i),
				OpC::R =>
					self.exec_reg(i),
				_ => panic!("Invalid opcode class.")
			}
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
