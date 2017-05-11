/* instruction.rs - Exposes a parsing object for a VR4300i instruction. */

use n64::cpu::op::*;
use n64::cpu::GPR_NAMES;

pub struct Inst(pub u32);

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
		(self.0 & 0x3ffffff) as u64
	}
}

use std::fmt;

impl fmt::Display for Inst {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.class() {
			OpC::I =>
				write!(f, "{} {}, {}, {:#x}", self.op_str(), GPR_NAMES[self.rt()], GPR_NAMES[self.rs()], self.imm()),
			OpC::L | OpC::S =>
				write!(f, "{} {}, {}({})", self.op_str(), GPR_NAMES[self.rt()], self.offset(), GPR_NAMES[self.rs()]),
			OpC::J =>
				write!(f, "{} {:#x}\n", self.op_str(), self.target()),
			OpC::B =>
				write!(f, "{} {}, {}, {}\n", self.op_str(), GPR_NAMES[self.rt()], GPR_NAMES[self.rs()], self.offset() as i16 as i64),
			OpC::R =>
				write!(f, "{} {}, {}, {}", self.op_str(), GPR_NAMES[self.rd()], GPR_NAMES[self.rt()], GPR_NAMES[self.rs()]),

		}
	}
}
