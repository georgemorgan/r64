/* op.rs - Exposes all of the possible VR4300i opcodes and their implementations. */

#[derive(Copy, Clone)]
/* Possible opcode classes. */
pub enum OpC {
	/* Immediate instruction. (I-type) */
	I,
	/* Load instruction. (subset of I-type) */
	L,
	/* Store instruction. (subset of I-type) */
	S,
	/* Jump instruction. (J-type) */
	J,
	/* Branch instruction. (subset of J-type) */
	B,
	/* Register instruction. */
	R,
}

/* Valid VR4300 opcodes. Figure 16-1 in NEC VR4300. */
#[derive(Copy, Clone)]
pub enum Op {

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

pub type OpF = &'static Fn(u64, u64, u16) -> u64;
pub type OpTup = (Op, &'static str, OpC, OpF);
/* A constant 2-d array of the opcode values. */
pub const OP_TABLE: [[OpTup; 8]; 8] = [
	[(Op::Special,  "special",  OpC::R, &|_, _, _| 0),
	 (Op::RegImm,   "regimm",   OpC::I, &|_, _, _| 0),
	 (Op::J,        "j",        OpC::J, &|_, _, _| 0),
	 (Op::Jal,      "jal",      OpC::J, &|_, _, _| 0),
	 (Op::Beq,      "beq",      OpC::B, &|rt, rs, _| if rt == rs { 1 } else { 0 }),
	 (Op::Bne,      "bne",      OpC::B, &|rt, rs, _| 0),
	 (Op::Blez,     "blez",     OpC::B, &|rt, rs, _| 0),
	 (Op::Bgtz,     "bgtz",     OpC::B, &|rt, rs, _| 0)],

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
	 (Op::Beql,     "beql",     OpC::B, &|rt, rs, _| if rt == rs { 1 } else { 0 }),
	 (Op::Bnel,     "bnel",     OpC::B, &|rt, rs, _| if rt != rs { 1 } else { 0 }),
	 (Op::Blezl,    "blezl",    OpC::B, &|rt, rs, _| if (rs as i64) < 0 { 1 } else { 0 }),
	 (Op::Bgtzl,    "bgtzl",    OpC::B, &|rt, rs, _| if (rs as i64) > 0 { 1 } else { 0 })],

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

	[(Op::Sb,       "sb",       OpC::S, &|rt, _, _| (rt & 0xff) as u64),
	 (Op::Sh,       "sh",       OpC::S, &|rt, _, _| (rt & 0xffff) as u64),
	 (Op::Swl,      "swl",      OpC::S, &|rt, _, _| 0),
	 (Op::Sw,       "sw",       OpC::S, &|rt, _, _| rt as u32 as u64),
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
pub const SP_OP_TABLE: [[OpTup; 8]; 8] = [
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
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Syscall,  "syscall",  OpC::R, &|_, _, _| 0),
	 (Op::Brk,      "brk",      OpC::R, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
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
pub const RI_OP_TABLE: [[OpTup; 8]; 4] = [
	[(Op::Bltz,     "bltz",     OpC::B, &|rt, rs, _| 0),
	 (Op::Bgez,     "bgez",     OpC::B, &|rt, rs, _| 0),
	 (Op::Bltzl,    "bltzl",    OpC::B, &|rt, rs, _| 0),
	 (Op::Bgezl,    "bgezl",    OpC::B, &|rt, rs, _| 0),
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

	[(Op::Bltzal,   "bltzal",   OpC::B, &|rt, rs, _| 0),
	 (Op::Bgezal,   "bgezal",   OpC::B, &|rt, rs, _| 0),
	 (Op::Bltzall,  "bltzall",  OpC::B, &|rt, rs, _| 0),
	 (Op::Bgezall,  "bgezall",  OpC::B, &|rt, rs, _| 0),
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
pub const COP_OP_RS_TABLE: [[OpTup; 8]; 4] = [
	[(Op::Mf,       "mf",       OpC::R, &|_, _, _| 0),
	 (Op::Dmf,      "dmf",      OpC::R, &|_, _, _| 0),
	 (Op::Cf,       "cf",       OpC::R, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Mt,       "mt",       OpC::R, &|_, _, _| 0),
	 (Op::Dmt,      "dmt",      OpC::R, &|_, _, _| 0),
	 (Op::Ct,       "ct",       OpC::R, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],

	[(Op::Bc,       "bc",       OpC::B, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],

	[(Op::Co,   	"co",       OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
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
pub const COP_OP_RT_TABLE: [[OpTup; 8]; 4] = [
	[(Op::Bcf,      "bcf",      OpC::R, &|_, _, _| 0),
	 (Op::Bct,      "bct",      OpC::R, &|_, _, _| 0),
	 (Op::Bcfl,     "bcfl",     OpC::R, &|_, _, _| 0),
	 (Op::Bctl,     "bctl", 	OpC::I, &|_, _, _| 0),
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

	[(Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
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
pub const COP_OP_FN_TABLE: [[OpTup; 8]; 4] = [
	[(Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Tlbr,     "tlbr",     OpC::R, &|_, _, _| 0),
	 (Op::Tlbwi,    "tlbwi",    OpC::R, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Tlbwr,    "tlbwr",    OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],

	[(Op::Tlbp,     "tlbp",     OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
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

	[(Op::Eret,     "eret",     OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0),
	 (Op::Reserved, "reserved", OpC::I, &|_, _, _| 0)],
];
