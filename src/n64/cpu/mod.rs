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
 use n64::mc::MC;

#[derive(Copy, Clone)]
/* Possible opcode classes. */
enum OpC {
	/* Immediate instruction. */
	I,
	/* Load instruction. (subset of I-type) */
	L,
	/* Store instruction. (subset of I-type) */
	S,
	/* Jump instruction. */
	J,
	/* Register instruction. */
	R,
}

/* Valid VR4300 opcodes. Figure 16-1 in NEC VR4300. */
#[derive(Copy, Clone)]
enum Op {

	/* Unique opcodes */
	Special,	RegImm,		J,			Jal,		Beq,		Bne,		Blez,		Bgtz,
	Addi,		Addiu,		Slti,		Sltiu,		Andi,		Ori,		Xori,		Lui,
	Cop0,		Cop1,		Cop2,		/**/		Beql,		Bnel,		Blezl,		Bgtzl,
	Daddi,		Daddiu,		Ldl,		Ldr,		/**/		/**/		/**/		/**/
	Lb,			Lh,			Lwl,		Lw,			Lbu,		Lhu,		Lwr,		Lwu,
	Sb,			Sh,			Swl,		Sw,			Sdl,		Sdr,		Swr,		Cache,
	Ll,			Lwc1,		Lwc2,		/**/		Lld,		Ldc1,		Ldc2,		Ld,
	Sc,			Swc1,		Swc2,		/**/		Scd,		Sdc1,		Sdc2,		Sd,

	/* Special opcodes */
	Sll,		/**/		Srl,		Sra,		Sllv,		/**/		Srlv,		Srav,
	Jr,			Jalr,		/**/		/**/		Syscall,	Brk,		/**/		Sync,
	Mfhi,		Mthi,		Mflo,		Mtlo,		Dsllv,		/**/		Dsrlv,		Dsrav,
	Mult,		Multu,		Div,		Divu,		Dmult,		Dmultu,		Ddiv,		Ddivu,
	Add,		Addu,		Sub,		Subu,		And,		Or,			Xor,		Nor,
	/**/		/**/		Slt,		Sltu,		Dadd,		Daddu,		Dsub,		Dsubu,
	Tge,		Tgeu,		Tlt,		Tltu,		Teq,		/**/		Tne,		/**/
	Dsll,		/**/		Dsrl,		Dsra,		Dsll32,		/**/		Dsrl32,		Dsra32,

	/* RegImm opcodes. */
	Bltz,		Bgez,		Bltzl,		Bgezl,		/**/		/**/		/**/		/**/
	Tgei,		Tgeiu,		Tlti,		Tltiu,		Teqi,		/**/		Tnei,		/**/
	Bltzal,		Bgezal,		Bltzall,	Bgezall,	/**/		/**/		/**/		/**/
	/**/		/**/		/**/		/**/		/**/		/**/		/**/		/**/

	/* COPz rs opcodes. */
	Mf,			Dmf,		Cf,			/**/		Mt,			Dmt,		Ct,			/**/
	Bc,			/**/		/**/		/**/		/**/		/**/		/**/		/**/
	Co,			/* Co */	/* Co */	/* Co */	/* Co */	/* Co */	/* Co */	/* Co */
	/* Co */	/* Co */	/* Co */	/* Co */	/* Co */	/* Co */	/* Co */	/* Co */

	/* - UNIMPLEMENTED OPCODES - */

	/* COPz rt opcodes. */
	Bcf,		Bct,		Bcfl,		Bctl,		/**/		/**/		/**/		/**/
	/**/		/**/		/**/		/**/		/**/		/**/		/**/		/**/
	/**/		/**/		/**/		/**/		/**/		/**/		/**/		/**/
	/**/		/**/		/**/		/**/		/**/		/**/		/**/		/**/

	/* CP0 opcodes */
	/**/		Tlbr,		Tlbwi,		/**/		/**/		/**/		Tlbwr,		/**/
	Tlbp,		/**/		/**/		/**/		/**/		/**/		/**/		/**/
	/**/		/**/		/**/		/**/		/**/		/**/		/**/		/**/
	Eret,		/**/		/**/		/**/		/**/		/**/		/**/		/**/
	/**/		/**/		/**/		/**/		/**/		/**/		/**/		/**/
	/**/		/**/		/**/		/**/		/**/		/**/		/**/		/**/
	/**/		/**/		/**/		/**/		/**/		/**/		/**/		/**/
	/**/		/**/		/**/		/**/		/**/		/**/		/**/		/**/

	Reserved
}

type OpF = &'static Fn(u64, u64, u16) -> u64;
type OpTup = (Op, &'static str, OpC, OpF);
/* A constant 2-d array of the opcode values. */
const OP_TABLE: [[OpTup; 8]; 8] = [
	[(Op::Special,  "special",  OpC::I, &|_, _, _| 0),
	 (Op::RegImm,   "regimm",   OpC::I, &|_, _, _| 0),
	 (Op::J,        "j",        OpC::J, &|_, _, _| 0),
	 (Op::Jal,      "jal",      OpC::J, &|_, _, _| 0),
	 (Op::Beq,      "beq",      OpC::J, &|_, _, _| 0),
	 (Op::Bne,      "bne",      OpC::J, &|_, _, _| 0),
	 (Op::Blez,     "blez",     OpC::J, &|_, _, _| 0),
	 (Op::Bgtz,     "bgtz",     OpC::J, &|_, _, _| 0)],

	[(Op::Addi,     "addi",     OpC::I, &|_, rs, imm| rs + (imm as i16) as u64),
	 (Op::Addiu,    "addiu",    OpC::I, &|_, rs, imm| rs + (imm as i16) as u64),
	 (Op::Slti,     "slti",     OpC::I, &|_, rs, imm| if (rs as i64) < (imm as i16) as i64 { 1 } else { 0 }),
	 (Op::Sltiu,    "sltiu",    OpC::I, &|_, rs, imm| if rs < (imm as i16) as u64 { 1 } else { 0 }),
	 (Op::Andi,     "andi",     OpC::I, &|_, rs, imm| rs & imm as u64),
	 (Op::Ori,      "ori",      OpC::I, &|_, rs, imm| rs | imm as u64),
	 (Op::Xori,     "xori",     OpC::I, &|_, rs, imm| rs ^ imm as u64),
	 (Op::Lui,      "lui",      OpC::I, &|_, rs, imm| (imm as u64) << 16)],

	[(Op::Cop0,     "cop0",     OpC::I, &|_, _, _| 0),
	 (Op::Cop1,     "cop1",     OpC::I, &|_, _, _| 0),
	 (Op::Cop2,     "cop2",     OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Beql,     "beql",     OpC::J, &|_, _, _| 0),
	 (Op::Bnel,     "bnel",     OpC::J, &|_, _, _| 0),
	 (Op::Blezl,    "blezl",    OpC::J, &|_, _, _| 0),
	 (Op::Bgtzl,    "bgtzl",    OpC::J, &|_, _, _| 0)],

	[(Op::Daddi,    "daddi",    OpC::I, &|_, rs, imm| 0),
	 (Op::Daddiu,   "daddiu",   OpC::I, &|_, rs, imm| 0),
	 (Op::Ldl,      "ldl",      OpC::L, &|val, _, _| 0),
	 (Op::Ldr,      "ldr",      OpC::L, &|val, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],

	[(Op::Lb,       "lb",       OpC::L, &|val, _, _| (val & 0xff) as i8 as i64 as u64),
	 (Op::Lh,       "lh",       OpC::L, &|val, _, _| (val & 0xffff) as i16 as i64 as u64),
	 (Op::Lwl,      "lwl",      OpC::L, &|val, _, _| 0),
	 (Op::Lw,       "lw",       OpC::L, &|val, _, _| val as i32 as i64 as u64),
	 (Op::Lbu,      "lbu",      OpC::L, &|val, _, _| (val & 0xff) as u8 as u64),
	 (Op::Lhu,      "lhu",      OpC::L, &|val, _, _| (val & 0xffff) as u16 as u64),
	 (Op::Lwr,      "lwr",      OpC::L, &|val, _, _| 0),
	 (Op::Lwu,      "lwu",      OpC::L, &|val, _, _| val as u32 as u64)],

	[(Op::Sb,       "sb",       OpC::S, &|rt, _, _| 0),
	 (Op::Sh,       "sh",       OpC::S, &|rt, _, _| 0),
	 (Op::Swl,      "swl",      OpC::S, &|rt, _, _| 0),
	 (Op::Sw,       "sw",       OpC::S, &|rt, _, _| 0),
	 (Op::Sdl,      "sdl",      OpC::S, &|rt, _, _| 0),
	 (Op::Sdr,      "sdr",      OpC::S, &|rt, _, _| 0),
	 (Op::Swr,      "swr",      OpC::S, &|rt, _, _| 0),
	 (Op::Cache,    "cache",    OpC::I, &|_, _, _| 0)],

	[(Op::Ll,       "ll",       OpC::L, &|val, _, _| 0),
	 (Op::Lwc1,     "lwc1",     OpC::L, &|val, _, _| 0),
	 (Op::Lwc2,     "lwc2",     OpC::L, &|val, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|val, _, _| 0),
	 (Op::Lld,      "lld",      OpC::L, &|val, _, _| 0),
	 (Op::Ldc1,     "ldc1",     OpC::L, &|val, _, _| 0),
	 (Op::Ldc2,     "ldc2",     OpC::L, &|val, _, _| 0),
	 (Op::Ld,       "ld",       OpC::L, &|val, _, _| 0)],

	[(Op::Sc,       "sc",       OpC::S, &|rt, _, _| 0),
	 (Op::Swc1,     "swc1",     OpC::S, &|rt, _, _| 0),
	 (Op::Swc2,     "swc2",     OpC::S, &|rt, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|rt, _, _| 0),
	 (Op::Scd,      "scd",      OpC::S, &|rt, _, _| 0),
	 (Op::Sdc1,     "sdc1",     OpC::S, &|rt, _, _| 0),
	 (Op::Sdc2,     "sdc2",     OpC::S, &|rt, _, _| 0),
	 (Op::Sd,       "sd",       OpC::S, &|rt, _, _| 0)],
];

/* A constant 2-d array of the opcode values. */
const SP_OP_TABLE: [[OpTup; 8]; 8] = [
	[(Op::Sll,      "sll",      OpC::R, &|rt, rs, sa| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Srl,      "srl",      OpC::R, &|rt, rs, sa| 0),
	 (Op::Sra,      "sra",      OpC::R, &|rt, rs, sa| 0),
	 (Op::Sllv,     "sllv",     OpC::R, &|rt, rs, sa| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Srlv,     "srlv",     OpC::R, &|rt, rs, sa| 0),
	 (Op::Srav,     "srav",     OpC::R, &|rt, rs, sa| 0)],

	[(Op::Jr,       "jr",       OpC::J, &|_, _, _| 0),
	 (Op::Jalr,     "jalr",     OpC::J, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::R, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::R, &|_, _, _| 0),
	 (Op::Syscall,  "syscall",  OpC::R, &|_, _, _| 0),
	 (Op::Brk,      "brk",      OpC::R, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::R, &|_, _, _| 0),
	 (Op::Sync,     "sync",     OpC::R, &|_, _, _| 0)],

	[(Op::Mfhi,     "mfhi",     OpC::R, &|_, _, _| 0),
	 (Op::Mthi,     "mthi",     OpC::R, &|_, _, _| 0),
	 (Op::Mflo,     "mflo",     OpC::R, &|_, _, _| 0),
	 (Op::Mtlo,     "mtlo",     OpC::R, &|_, _, _| 0),
	 (Op::Dsllv,    "dsllv",    OpC::R, &|_, _, _| 0),
	 (Op::Reserved, "resered",  OpC::R, &|_, _, _| 0),
	 (Op::Dsrlv,    "dsrlv",    OpC::R, &|_, _, _| 0),
	 (Op::Dsrav,    "dsrav",    OpC::R, &|_, _, _| 0)],

	[(Op::Mult,     "mult",     OpC::R, &|rt, rs, _| 0),
	 (Op::Multu,    "multu",    OpC::R, &|rt, rs, _| 0),
	 (Op::Div,      "div",      OpC::R, &|rt, rs, _| 0),
	 (Op::Divu,     "divu",     OpC::R, &|rt, rs, _| 0),
	 (Op::Dmult,    "dmult",    OpC::R, &|rt, rs, _| 0),
	 (Op::Dmultu,   "dmultu",   OpC::R, &|rt, rs, _| 0),
	 (Op::Ddiv,     "ddiv",     OpC::R, &|rt, rs, _| 0),
	 (Op::Ddivu	,   "ddivu",    OpC::R, &|rt, rs, _| 0)],

	[(Op::Add,      "add",      OpC::R, &|rt, rs, _| 0),
	 (Op::Addu,     "addu",     OpC::R, &|rt, rs, _| 0),
	 (Op::Sub,      "sub",      OpC::R, &|rt, rs, _| 0),
	 (Op::Subu,     "subu",     OpC::R, &|rt, rs, _| 0),
	 (Op::And,      "and",      OpC::R, &|rt, rs, _| 0),
	 (Op::Or,       "or",       OpC::R, &|rt, rs, _| 0),
	 (Op::Xor,      "xor",      OpC::R, &|rt, rs, _| 0),
	 (Op::Nor,      "nor",      OpC::R, &|rt, rs, _| 0)],

	[(Op::Reserved, "reserved", OpC::R, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::R, &|_, _, _| 0),
	 (Op::Slt,      "slt",      OpC::R, &|rt, rs, _| 0),
	 (Op::Sltu,     "sltu",     OpC::R, &|rt, rs, _| 0),
	 (Op::Dadd,     "dadd",     OpC::R, &|rt, rs, _| 0),
	 (Op::Daddu,    "daddu",    OpC::R, &|rt, rs, _| 0),
	 (Op::Dsub,     "dsub",     OpC::R, &|rt, rs, _| 0),
	 (Op::Dsubu,    "dsubu",    OpC::R, &|rt, rs, _| 0)],

	[(Op::Tge,      "tge",      OpC::R, &|rt, rs, _| 0),
	 (Op::Tgeu,     "tgeu",     OpC::R, &|rt, rs, _| 0),
	 (Op::Tlt,      "tlt",      OpC::R, &|rt, rs, _| 0),
	 (Op::Tltu,     "tltu",     OpC::R, &|rt, rs, _| 0),
	 (Op::Teq,      "teq",      OpC::R, &|rt, rs, _| 0),
	 (Op::Reserved, "reserved", OpC::R, &|rt, rs, _| 0),
	 (Op::Tne,      "tne",      OpC::R, &|rt, rs, _| 0),
	 (Op::Reserved, "reserved", OpC::R, &|rt, rs, _| 0)],

	[(Op::Dsll,     "dsll",     OpC::R, &|rt, rs, sa| 0),
	 (Op::Reserved, "reserved", OpC::R, &|_, _, _| 0),
	 (Op::Dsrl,     "dsrl",     OpC::R, &|rt, rs, sa| 0),
	 (Op::Dsra,     "dsra",     OpC::R, &|rt, rs, sa| 0),
	 (Op::Dsll32,   "dsll32",   OpC::R, &|rt, rs, sa| 0),
	 (Op::Reserved, "reserved", OpC::R, &|_, _, _| 0),
	 (Op::Dsrl32,   "dsrl32",   OpC::R, &|rt, rs, sa| 0),
	 (Op::Dsra32,   "dsra32",   OpC::R, &|rt, rs, sa| 0)],
];

/* A constant 2-d array of the opcode values. , _*/
const RI_OP_TABLE: [[OpTup; 8]; 4] = [
	[(Op::Bltz,     "bltz",     OpC::J, &|_, _, _| 0),
	 (Op::Bgez,     "bgez",     OpC::J, &|_, _, _| 0),
	 (Op::Bltzl,    "bltzl",    OpC::J, &|_, _, _| 0),
	 (Op::Bgezl,    "bgezl",    OpC::J, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],

	[(Op::Tgei,     "tgei",     OpC::I, &|_, _, _| 0),
	 (Op::Tgeiu,    "tgeiu",    OpC::I, &|_, _, _| 0),
	 (Op::Tlti,     "tlti",     OpC::I, &|_, _, _| 0),
	 (Op::Tltiu,    "tltiu",    OpC::I, &|_, _, _| 0),
	 (Op::Teqi,     "teqi",     OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Tnei,     "tnei",     OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],

	[(Op::Bltzal,   "bltzal",   OpC::J, &|_, _, _| 0),
	 (Op::Bgezal,   "bgezal",   OpC::J, &|_, _, _| 0),
	 (Op::Bltzall,  "bltzall",  OpC::J, &|_, _, _| 0),
	 (Op::Bgezall,  "bgezall",  OpC::J, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],

	[(Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],
];

/* A constant 2-d array of the opcode values. */
const COP_OP_TABLE: [[OpTup; 8]; 4] = [
	[(Op::Mf,       "mf",       OpC::J, &|_, _, _| 0),
	 (Op::Dmf,      "dmf",      OpC::J, &|_, _, _| 0),
	 (Op::Cf,       "cf",       OpC::J, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::J, &|_, _, _| 0),
	 (Op::Mt,       "mt",       OpC::I, &|_, _, _| 0),
	 (Op::Dmt,      "dmt",      OpC::I, &|_, _, _| 0),
	 (Op::Ct,       "ct",       OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],

	[(Op::Bc,       "bc",       OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],

	[(Op::Co,   	"co",       OpC::J, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::J, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::J, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::J, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],

	[(Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],
];

struct Inst(pub u32);

impl Inst {
	/* Returns the instruction's opcode. */
	pub fn opcode(&self) -> u8{
		((self.0 >> 26) & 0b111111) as u8
	}
	/* The top-level opcode tuple. */
	fn op_tup_top(&self) -> OpTup {
		OP_TABLE[((self.opcode() >> 3) & 0b111) as usize][(self.opcode() & 0b111) as usize]
	}
	/* Returns the opcode's tuple. */
	fn op_tup(&self) -> OpTup {
		let t = self.op_tup_top();
		match t.0 {
			Op::Special =>
				SP_OP_TABLE[((self.funct() >> 3) & 0b111) as usize][(self.funct() & 0b111) as usize],
			Op::RegImm =>
				RI_OP_TABLE[((self.rt() >> 3) & 0b11) as usize][(self.rt() & 0b111) as usize],
			Op::Cop0 =>
				COP_OP_TABLE[((self.rs() >> 3) & 0b11) as usize][(self.rs() & 0b111) as usize],
			Op::Cop1 => panic!("Attempt to resolve Cop1 instruction."),
			Op::Cop2 => panic!("Attempt to resolve Cop2 instruction."),
			_ =>
				t,
		}
	}
	/* Returns the enumerated operation type. */
	pub fn op(&self) -> Op {
		self.op_tup().0
	}
	/* Returns a string of the opcode for debugging. */
	pub fn op_str(&self) -> String {
		let s = self.op_tup().1;
		match self.op_tup_top().0 {
			Op::Cop0 =>
				format!("{}c0", s),
			Op::Cop1 =>
				format!("{}c1", s),
			Op::Cop2 =>
				format!("{}c2", s),
			_ => s.to_owned()
		}
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
	pub fn sa(&self) -> u16 {
		((self.0 >> 6) & 0b11111) as u16
	}
	/* Returns the instruction's immediate value. */
	pub fn imm(&self) -> u16 {
		(self.0 & 0xffff) as u16
	}
	/* Return's the function's funct field. */
	pub fn funct(&self) -> u8 {
		(self.0 & 0b111111) as u8
	}
	/* The offset in a LD/ST instruction. */
	pub fn offset(&self) -> u16 {
		(self.0 & 0xffff) as u16
	}
	/* Returns the function's target. */
	pub fn target(&self) -> u64 {
		0
	}
}
const GPR_NAMES: [&'static str; GPR_SIZE] = [
	"r0", "at", "v0", "v1", "a0", "a1", "a2", "a3",
	"t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
	"s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
	"t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra",
];

use std::fmt;

impl fmt::Display for Inst {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.class() {
			OpC::I =>
				write!(f, "{} {}, {}, {:#x}", self.op_str(), GPR_NAMES[self.rt()], GPR_NAMES[self.rs()], self.imm()),
			OpC::L | OpC::S =>
				write!(f, "{} {}, {}({})", self.op_str(), GPR_NAMES[self.rt()], self.offset(), GPR_NAMES[self.rs()]),
			OpC::J =>
				write!(f, "{} {:#x}", self.op_str(), self.target()),
			OpC::R =>
				write!(f, "{} {}, {}, {}", self.op_str(), GPR_NAMES[self.rd()], GPR_NAMES[self.rt()], GPR_NAMES[self.rs()]),

		}
	}
}

/* Size of the general purpose register file. */
const GPR_SIZE: usize = 32;

pub struct CPU {
	/* The CPU's general purpose register file. */
	gpr: [u64; GPR_SIZE],
	/* The CPU's floating point register file. */
	fpr: [f64; GPR_SIZE],
	/* The program counter. */
	pc: u64
}

impl CPU {
	pub fn new(pc: u64) -> CPU {
		CPU {
			/* Zero-initialize the registers. */
			gpr: [0; GPR_SIZE],
			fpr: [0.0; GPR_SIZE],
			/* Initialize the program counter. */
			pc: pc
		}
	}
	/* Reads a value from the specified GPR. */
	fn rgpr(&self, reg: usize) -> u64 {
		self.gpr[reg]
	}
	/* Reads a value from the specified FPR. */
	fn rfpr(&self, reg: usize) -> f64 {
		self.fpr[reg]
	}
	/* Writes a value to the specified GPR. */
	fn wgpr(&mut self, val: u64, reg: usize) {
		self.gpr[reg] = val;
	}
	/* Writes a value to the specified FPR. */
	fn wfpr(&mut self, val: f64, reg: usize) {
		self.fpr[reg] = val;
	}

	/* Handlers for the 3 instruction formats. - Chapter 3.1 in NEC VR4300 manual. */

	/* Handler for the immediate (I-Type) instructions. */
	fn exec_imm(&mut self, i: Inst) {
		/* Obtain the value stored in the rs register. */
		let rs = self.rgpr(i.rs());
		/* Obtain the immediate value. */
		let imm = i.imm();
		/* Perform the operation. */
		let rt = i.function()(0, rs, imm);
		/* Write the result back into the target register. */
		self.wgpr(rt, i.rt());
	}
	/* Handler for the load/store subclass of the I-type instructions. */
	fn exec_ldst(&mut self, mc: &mut MC, i: Inst) {
		/* Obtain the base address. */
		let base = self.rgpr(i.rs());
		/* Obtain the offset. */
		let offset = i.offset();
		/* Are we loading or storing? */
		match i.class() {
			OpC::L => {
				let val = mc.read((base + offset as u64) as usize) as u64;
				/* Let the instruction's function determine the value we write. */
				let rt = i.function()(val, 0, 0);
				/* Write the result back into the target register. */
				self.wgpr(rt, i.rt());
			},
			OpC::S => {
				/* Obtain the value to be stored. */
				let rt = self.rgpr(i.rt());
				/* Let the function mutate this value as needed. */
				let val = i.function()(rt, 0, 0) as u32;
				/* Write the result into memory. */
				mc.write(val, (base + offset as u64) as usize);
			},
			_ => ()
		}
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
		let rd = i.function()(rt, rs, i.sa());
		/* Write the result back into the destination register. */
		self.wgpr(rd, i.rd());
	}

	/* Executes an instruction. */
	pub fn cycle(&mut self, mc: &mut MC) {
		/* Fetch the next instrution from memory. */
		let i = Inst(mc.read(self.pc as usize));
		/* Print the opcode. */
		println!("{:#x}: {}", self.pc, i);
		/* Execute the instrution. */
		match i.op() {
			Op::Reserved => panic!("Attempt made to execute a reserved instruction {:#x}.", i.opcode()),
			_ => match i.class() {
				OpC::I =>
					self.exec_imm(i),
				OpC::L | OpC::S =>
					self.exec_ldst(mc, i),
				OpC::J =>
					self.exec_jump(i),
				OpC::R =>
					self.exec_reg(i),
			}
		}
		println!("{:?}", self);
		/* Increment the program counter. */
		self.pc += 4;
	}
}

impl fmt::Debug for CPU {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		const REGS_PER_LINE: usize = 2;
		try!(write!(f, "\nCPU General Purpose Registers:"));
		for r in 0..GPR_SIZE {
			if (r % REGS_PER_LINE) == 0 {
				try!(writeln!(f, ""))
			}
			try!(write!(f, "{:02} ({}): {:#018X} ", r, GPR_NAMES[r], self.rgpr(r)))
		}

		try!(write!(f, "\n\nCPU Floating Point Registers:"));
		for r in 0..GPR_SIZE {
			if (r % REGS_PER_LINE) == 0 {
				try!(writeln!(f, ""))
			}
			try!(write!(f, "fpr{:02}: {:21} ", r, self.rfpr(r)))
		}

		Ok(())

		// try!(writeln!(f, "\n\nCPU Special Registers:"));
		// try!(writeln!(f,
		// 	"\
		// 	reg_pc: {:#018X}\n\
		// 	reg_hi: {:#018X}\n\
		// 	reg_lo: {:#018X}\n\
		// 	reg_llbit: {}\n\
		// 	reg_fcr0:  {:#010X}\n\
		// 	reg_fcr31: {:#010X}\n\
		// 	",
		// 	self.reg_pc,
		// 	self.reg_hi,
		// 	self.reg_lo,
		// 	self.reg_llbit,
		// 	self.reg_fcr0,
		// 	self.reg_fcr31
		// ));

		//writeln!(f, "{:#?}", self.cp0)
	}
}
