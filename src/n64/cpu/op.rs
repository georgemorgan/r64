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
const RESERVED: OpTup = (Op::Reserved, "reserved", OpC::R, &|_, _, _| {
	unimplemented!()
});

/* A constant 2-d array of the opcode values. */
pub const OP_TABLE: [[&OpTup; 8]; 8] = [

    /* ROW: 0 */

	[&(Op::Special, "special", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::RegImm, "regimm", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::J, "j", OpC::J, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Jal, "jal", OpC::J, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Beq, "beq", OpC::B, &|rt, rs, _| {
		if rt == rs { 1 } else { 0 }
	}),

	&(Op::Bne, "bne", OpC::B, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Blez, "blez", OpC::B, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Bgtz, "bgtz", OpC::B, &|rt, rs, _| {
		unimplemented!()
	})],

	/* ROW: 1 */

	[&(Op::Addi, "addi", OpC::I, &|_, rs, imm| {
		rs + (imm as i16) as u64
	}),

	&(Op::Addiu, "addiu", OpC::I, &|_, rs, imm| {
		rs + (imm as i16) as u64
	}),

	&(Op::Slti, "slti", OpC::I, &|_, rs, imm| {
		if (rs as i64) < (imm as i16) as i64 { 1 } else { 0 }
	}),

	&(Op::Sltiu, "sltiu", OpC::I, &|_, rs, imm| {
		if rs < (imm as i16) as u64 { 1 } else { 0 }
	}),

	&(Op::Andi, "andi", OpC::I, &|_, rs, imm| {
		rs & imm as u64
	}),

	&(Op::Ori, "ori", OpC::I, &|_, rs, imm| {
		rs | imm as u64
	}),

	&(Op::Xori, "xori", OpC::I, &|_, rs, imm| {
		rs ^ imm as u64
	}),

	&(Op::Lui, "lui", OpC::I, &|_, _, imm| {
		(imm as i16 as i64 as u64) << 16
	})],

	/* ROW: 2 */

	[&(Op::Cop0, "cop0", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Cop1, "cop1", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Cop2, "cop2", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED,

	&(Op::Beql, "beql", OpC::B, &|rt, rs, _| {
		if rt == rs { 1 } else { 0 }
	}),

	&(Op::Bnel, "bnel", OpC::B, &|rt, rs, _| {
		if rt != rs { 1 } else { 0 }
	}),

	&(Op::Blezl, "blezl", OpC::B, &|rt, rs, _| {
		if (rs as i64) < 0 { 1 } else { 0 }
	}),

	&(Op::Bgtzl, "bgtzl", OpC::B, &|rt, rs, _| {
		if (rs as i64) > 0 { 1 } else { 0 }
	})],

	/* ROW: 3 */

	[&(Op::Daddi, "daddi", OpC::I, &|_, rs, imm| {
		unimplemented!()
	}),

	&(Op::Daddiu, "daddiu", OpC::I, &|_, rs, imm| {
		unimplemented!()
	}),

	&(Op::Ldl, "ldl", OpC::L, &|val, _, _| {
		unimplemented!()
	}),

	&(Op::Ldr, "ldr", OpC::L, &|val, _, _| {
		unimplemented!()
	}),

	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],

	/* ROW: 4 */

	[&(Op::Lb, "lb", OpC::L, &|val, _, _| {
		(val & 0xff) as i8 as i64 as u64
	}),

	&(Op::Lh, "lh", OpC::L, &|val, _, _| {
		(val & 0xffff) as i16 as i64 as u64
 	}),

	&(Op::Lwl, "lwl", OpC::L, &|val, _, _| {
		unimplemented!()
	}),

	&(Op::Lw, "lw", OpC::L, &|val, _, _| {
		val as i32 as i64 as u64
	}),

	&(Op::Lbu, "lbu", OpC::L, &|val, _, _| {
		(val & 0xff) as u8 as u64
	}),

	&(Op::Lhu, "lhu", OpC::L, &|val, _, _| {
		(val & 0xffff) as u16 as u64
	}),

	&(Op::Lwr, "lwr", OpC::L, &|val, _, _| {
		unimplemented!()
	}),

	&(Op::Lwu, "lwu", OpC::L, &|val, _, _| val as u32 as u64)],

	/* ROW: 5 */

	[&(Op::Sb, "sb", OpC::S, &|rt, _, _| {
		(rt & 0xff) as u64
	}),

	&(Op::Sh, "sh", OpC::S, &|rt, _, _| {
		(rt & 0xffff) as u64
	}),

	&(Op::Swl, "swl", OpC::S, &|rt, _, _| {
		unimplemented!()
	}),

	&(Op::Sw, "sw", OpC::S, &|rt, _, _| {
		rt as u32 as u64
	}),

	&(Op::Sdl, "sdl", OpC::S, &|rt, _, _| {
		unimplemented!()
	}),

	&(Op::Sdr, "sdr", OpC::S, &|rt, _, _| {
		unimplemented!()
	}),

	&(Op::Swr, "swr", OpC::S, &|rt, _, _| {
		unimplemented!()
	}),

	&(Op::Cache, "cache", OpC::I, &|_, _, _| {
		unimplemented!()
	})],

	/* ROW: 6 */

	[&(Op::Ll, "ll", OpC::L, &|val, _, _| {
		unimplemented!()
	}),

	&(Op::Lwc1, "lwc1", OpC::L, &|val, _, _| {
		unimplemented!()
	}),

	&(Op::Lwc2, "lwc2", OpC::L, &|val, _, _| {
		unimplemented!()
	}),

	&RESERVED,

	&(Op::Lld, "lld", OpC::L, &|val, _, _| {
		unimplemented!()
	}),

	&(Op::Ldc1, "ldc1", OpC::L, &|val, _, _| {
		unimplemented!()
	}),

	&(Op::Ldc2, "ldc2", OpC::L, &|val, _, _| {
		unimplemented!()
	}),

	&(Op::Ld, "ld", OpC::L, &|val, _, _| {
		unimplemented!()
	})],

	/* ROW: 7 */

	[&(Op::Sc, "sc", OpC::S, &|rt, _, _| {
		unimplemented!()
	}),

	&(Op::Swc1, "swc1", OpC::S, &|rt, _, _| {
		unimplemented!()
	}),

	&(Op::Swc2, "swc2", OpC::S, &|rt, _, _| {
		unimplemented!()
	}),

	&RESERVED,

	&(Op::Scd, "scd", OpC::S, &|rt, _, _| {
		unimplemented!()
	}),

	&(Op::Sdc1, "sdc1", OpC::S, &|rt, _, _| {
		unimplemented!()
	}),

	&(Op::Sdc2, "sdc2", OpC::S, &|rt, _, _| {
		unimplemented!()
	}),

	&(Op::Sd, "sd", OpC::S, &|rt, _, _| 0)],
];

/* A constant 2-d array of the opcode values. */
pub const SP_OP_TABLE: [[&OpTup; 8]; 8] = [

    /* ROW: 0 */

	[&(Op::Sll, "sll", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	}),

	&RESERVED,

	&(Op::Srl, "srl", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	}),

	&(Op::Sra, "sra", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	}),

	&(Op::Sllv, "sllv", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	}),

	&RESERVED,

	&(Op::Srlv, "srlv", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	}),

	&(Op::Srav, "srav", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	})],

	/* ROW: 1 */

	[&(Op::Jr, "jr", OpC::J, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Jalr, "jalr", OpC::J, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED,
	&RESERVED,

	&(Op::Syscall, "syscall", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Brk, "brk", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED,

	&(Op::Sync, "sync", OpC::R, &|_, _, _| {
		unimplemented!()
	})],

	/* ROW: 2 */

	[&(Op::Mfhi, "mfhi", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Mthi, "mthi", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Mflo, "mflo", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Mtlo, "mtlo", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Dsllv, "dsllv", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Reserved, "resered", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Dsrlv, "dsrlv", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Dsrav, "dsrav", OpC::R, &|_, _, _| {
		unimplemented!()
	})],

	/* ROW: 3 */

	[&(Op::Mult, "mult", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Multu, "multu", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Div, "div", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Divu, "divu", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Dmult, "dmult", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Dmultu, "dmultu", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Ddiv, "ddiv", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Ddivu	, "ddivu", OpC::R, &|rt, rs, _| {
		unimplemented!()
	})],

	/* ROW: 4 */

	[&(Op::Add, "add", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Addu, "addu", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Sub, "sub", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Subu, "subu", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::And, "and", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Or, "or", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Xor, "xor", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Nor, "nor", OpC::R, &|rt, rs, _| {
		unimplemented!()
	})],

	/* ROW: 5 */

	[&RESERVED,

	&RESERVED,

	&(Op::Slt, "slt", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Sltu, "sltu", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Dadd, "dadd", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Daddu, "daddu", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Dsub, "dsub", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Dsubu, "dsubu", OpC::R, &|rt, rs, _| {
		unimplemented!()
	})],

	/* ROW: 6 */

	[&(Op::Tge, "tge", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Tgeu, "tgeu", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Tlt, "tlt", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Tltu, "tltu", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Teq, "teq", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Reserved, "reserved", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Tne, "tne", OpC::R, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Reserved, "reserved", OpC::R, &|rt, rs, _| {
		unimplemented!()
	})],

	/* ROW: 7 */

	[&(Op::Dsll, "dsll", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	}),

	&RESERVED,

	&(Op::Dsrl, "dsrl", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	}),

	&(Op::Dsra, "dsra", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	}),

	&(Op::Dsll32, "dsll32", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	}),

	&RESERVED,

	&(Op::Dsrl32, "dsrl32", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	}),

	&(Op::Dsra32, "dsra32", OpC::R, &|rt, rs, sa| {
		unimplemented!()
	})],
];

/* A constant 2-d array of the opcode values. , _*/
pub const RI_OP_TABLE: [[&OpTup; 8]; 4] = [

    /* ROW: 0 */

	[&(Op::Bltz, "bltz", OpC::B, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Bgez, "bgez", OpC::B, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Bltzl, "bltzl", OpC::B, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Bgezl, "bgezl", OpC::B, &|rt, rs, _| {
		unimplemented!()
	}),

	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],

	/* ROW: 1 */

	[&(Op::Tgei, "tgei", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Tgeiu, "tgeiu", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Tlti, "tlti", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Tltiu, "tltiu", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Teqi, "teqi", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED,

	&(Op::Tnei, "tnei", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED],

	/* ROW: 2 */

	[&(Op::Bltzal, "bltzal", OpC::B, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Bgezal, "bgezal", OpC::B, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Bltzall, "bltzall", OpC::B, &|rt, rs, _| {
		unimplemented!()
	}),

	&(Op::Bgezall, "bgezall", OpC::B, &|rt, rs, _| {
		unimplemented!()
	}),

	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],

	/* ROW: 3 */

	[&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,

	&(Op::Reserved, "reserved", OpC::I, &|_, _, _| {
		unimplemented!()
	})],
];

/* A constant 2-d array of the opcode values. */
pub const COP_OP_RS_TABLE: [[&OpTup; 8]; 4] = [

    /* ROW: 0 */

	[&(Op::Mf, "mf", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Dmf, "dmf", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Cf, "cf", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED,

	&(Op::Mt, "mt", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Dmt, "dmt", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Ct, "ct", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED],

	/* ROW: 4 */

	[&(Op::Bc, "bc", OpC::B, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],

	/* ROW: 5 */

	[&(Op::Co, "co", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],

	/* ROW: 6 */

	[&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],
];

/* A constant 2-d array of the opcode values. */
pub const COP_OP_RT_TABLE: [[&OpTup; 8]; 4] = [

    /* ROW: 0 */

	[&(Op::Bcf, "bcf", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Bct, "bct", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Bcfl, "bcfl", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Bctl, "bctl", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],

	/* ROW: 7 */

	[&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],

	/* ROW: 8 */

	[&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],

	/* ROW: 9 */

	[&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],
];

/* A constant 2-d array of the opcode values. */
pub const COP_OP_FN_TABLE: [[&OpTup; 8]; 4] = [

    /* ROW: 0 */

	[&RESERVED,

	&(Op::Tlbr, "tlbr", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&(Op::Tlbwi, "tlbwi", OpC::R, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED,
	&RESERVED,
	&RESERVED,

	&(Op::Tlbwr, "tlbwr", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED],

	/* ROW: 1 */

	[&(Op::Tlbp, "tlbp", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],

	/* ROW: 2 */

	[&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],

	/* ROW: 3 */

	[&(Op::Eret, "eret", OpC::I, &|_, _, _| {
		unimplemented!()
	}),

	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED,
	&RESERVED],
];
