/* cpu.rs - Exposes all of the implementation needed to organize and execute VR4300i opcodes. */

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

*/

use std::fmt;

use n64::N64;

mod op;
use self::op::*;

mod instruction;
use self::instruction::Inst;

mod cp0;
use self::cp0::*;

/* Size of the general purpose register file. */
const GPR_SIZE: usize = 32;

pub struct CPU {
	/* The CPU's coprocessor. */
	cp0: CP0,
	/* The CPU's general purpose register file. */
	gpr: [u64; GPR_SIZE],
	/* The CPU's floating point register file. */
	fpr: [f64; GPR_SIZE],
	/* The hi and lo register pair. */
	hi: u64,
	lo: u64,
	/* The load/link bit. */
	ll: u8,
	/* The program counter. */
	pc: u64
}

impl CPU {
	pub fn new(pc: u64) -> CPU {
		CPU {
			/* Initialize coprocessor 0. */
			cp0: CP0::new(),
			/* Zero-initialize the registers. */
			gpr: [0; GPR_SIZE],
			fpr: [0.0; GPR_SIZE],
			/* The hi and lo register pair. */
			hi: 0,
			lo: 0,
			/* The load/link bit. */
			ll: 0,
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
		/* Don't write to register 0. */
		match reg {
			0 => (),
			_ => {
				self.gpr[reg] = val;
			},
		}
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

	/* Handler for the jump (J-type) instructions. */
	fn exec_jump(&mut self, i: Inst) {
		match i.op() {
			/* Handle the jump (J-type) instructions here. */
			Op::J => {
				/* Obtain the jump target. */
				let target = i.target();
				/* Load the target into the program counter. */
				self.pc = target;
			}, Op::Jal => {
				/* Obtain the jump target. */
				let target = i.target();
				/* Load the program counter into the return address. */
				let pc = self.pc;
				self.wgpr(pc, 31);
				/* Load the target into the program counter. */
				self.pc = target;
			}, Op::Jr => {
				/* Obtain the jump target. */
				let target = self.rgpr(i.rs());
				/* Load the program counter into the return address. */
				self.pc = target;
			}, Op::Jalr => {
				/* Obtain the jump target. */
				let target = self.rgpr(i.rs());
				/* Load the program counter into the destination register. */
				let pc = self.pc;
				self.wgpr(pc, i.rd());
				/* Load the program counter into the return address. */
				self.pc = target;
			}, _ => ()
		}
	}

	/* Handler for the branch subclass of the J-type instructions. */
	fn exec_branch(&mut self, i: Inst) -> bool {
		/* Obtain the value stored in the rs and rt registers. */
		let rs = self.rgpr(i.rs());
		let rt = self.rgpr(i.rt());
		/* Obtain the offset address. */
		let offset = (i.offset() as i16 as i32) << 2;
		/* Determine whether or not the branch should occur .*/
		let should_branch = i.function()(rt, rs, 0);
		/* Perform the branch. */
		if should_branch > 0 {
			self.pc = (self.pc as i32 + offset) as u64;
			/* Don't increment the PC. */
			return false;
		}
		/* Increment the PC if the branch fell through. */
		true
	}

	/* Handler for the register (R-Type) instructions. */
	fn exec_reg(&mut self, i: Inst) {
		/* Obtain the value stored in the rs and rt registers. */
		let rs = self.rgpr(i.rs());
		let rt = self.rgpr(i.rt());
		/* Obtain the value that is to be placed in rd. */
		let rd = i.function()(rt, rs, i.sa());
		/* Write the result back into the destination register. */
		self.wgpr(rd, i.rd());
	}

	/* Handler for the coprocessor0 instructions. */
	fn exec_cop0(&mut self, i: Inst) {
		match i.op() {
			Op::Mf => {
				/* TODO: Check destination register parameters. */

				/* Obtain the rt register from CP0. */
				let rt = self.cp0.rreg(i.rt());
				/* Write the value into rd. */
				self.wgpr(rt, i.rd());
			}, Op::Dmf => {
				/* Obtain the rt register from CP0. */
				let rt = self.cp0.rreg(i.rt());
				/* Write the value into rd. */
				self.wgpr(rt, i.rd());
			}, Op::Cf => {
				unimplemented!()
			}, Op::Mt => {
				/* TODO: Check destination register parameters. */

				/* Obtain the rt register from CPU0. */
				let rt = self.rgpr(i.rt());
				/* Write the value into rd of CP0. */
				self.cp0.wreg(rt, i.rd());
			}, Op::Dmt => {
				/* Obtain the rt register from CPU0. */
				let rt = self.rgpr(i.rt());
				/* Write the value into rd of CP0. */
				self.cp0.wreg(rt, i.rd());
			}, Op::Ct => {
				unimplemented!()
			}, Op::Bcf => {
				unimplemented!()
			}, Op::Bct => {
				unimplemented!()
			}, Op::Bcfl => {
				unimplemented!()
			}, Op::Bctl => {
				unimplemented!()
			}, Op::Tlbr => {
				unimplemented!()
			}, Op::Tlbwi => {
				unimplemented!()
			}, Op::Tlbwr => {
				unimplemented!()
			}, Op::Tlbp => {
				unimplemented!()
			}, Op::Eret => {
				unimplemented!()
			}, _ => ()
		}
	}
}

/* Handler for the load/store subclass of the I-type instructions. */
fn exec_ldst(n64: &mut N64, i: Inst) {
	/* Obtain the base and offset addresses. */
	let base = n64.cpu.rgpr(i.rs()) as i32;
	let offset = i.offset() as i16 as i32;
	/* Are we loading or storing? */
	match i.class() {
		OpC::L => {
			/* Read the memory address. */
			let val = n64.read((base + offset) as u32) as u64;
			/* Let the instruction's function determine the value we write. */
			let rt = i.function()(val, 0, 0);
			/* Write the result back into the target register. */
			n64.cpu.wgpr(rt, i.rt());
		},
		OpC::S => {
			/* Obtain the value that is to be stored. */
			let rt = n64.cpu.rgpr(i.rt());
			/* Let the function mutate this value as required1 . */
			let val = i.function()(rt, 0, 0) as u32;
			/* Write the result into memory. */
			n64.write(val, (base + offset) as u32);
		},
		_ => ()
	}
}

pub fn cycle(n64: &mut N64) {
	/* Fetch the next instrution from memory. */
	let i = Inst(n64.read(n64.cpu.pc as u32));
	/* Print the opcode. */
	println!("{:#x}: ({:#x}) {}", n64.cpu.pc, i.0, i);
	/* Whether or not the PC should be incremented. */
	let mut inc_pc = true;
	/* Determine if the instrution needs to be performed on a co-processor. */
	match i.kind() {
		/* If the instruction is a co-processor instruction, perform it on CPz. */
		Op::Cop0 =>
			n64.cpu.exec_cop0(i),
		Op::Cop1 =>
			unimplemented!(),
		Op::Cop2 =>
			panic!("Attempt to perfrom a coprocessor instruction on an invalid coprocessor."),
		Op::Reserved =>
			panic!("Attempt made to execute a reserved instruction {:#x}.", i.opcode()),
		_ => match i.class() {
			OpC::I =>
				n64.cpu.exec_imm(i),
			OpC::L | OpC::S =>
				exec_ldst(n64, i),
			OpC::J => {
				n64.cpu.exec_jump(i);
				inc_pc = false;
			},
			OpC::B => {
				inc_pc = n64.cpu.exec_branch(i);
			},
			OpC::R =>
				n64.cpu.exec_reg(i)
		}
	}
	/* Increment the program counter if necessary. */
	if inc_pc {
		/* Increment the program counter. */
		n64.cpu.pc += 4;
	}
}

const GPR_NAMES: [&'static str; GPR_SIZE] = [
	"r0", "at", "v0", "v1", "a0", "a1", "a2", "a3",
	"t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
	"s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
	"t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra",
];

impl fmt::Debug for CPU {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		const REGS_PER_LINE: usize = 2;
		for r in 0..GPR_SIZE {
			if (r % REGS_PER_LINE) == 0 {
				try!(writeln!(f, ""))
			}
			try!(write!(f, "{:02} ({}): {:#018X} ", r, GPR_NAMES[r], self.rgpr(r)))
		}

		// for r in 0..GPR_SIZE {
		// 	if (r % REGS_PER_LINE) == 0 {
		// 		try!(writeln!(f, ""))
		// 	}
		// 	try!(write!(f, "{:02} ({}): {:#018X} ", r, r, self.cp0.rreg(r)))
		// }

		// try!(write!(f, "\n\nCPU Floating Point Registers:"));
		// for r in 0..GPR_SIZE {
		// 	if (r % REGS_PER_LINE) == 0 {
		// 		try!(writeln!(f, ""))
		// 	}
		// 	try!(write!(f, "fpr{:02}: {:21} ", r, self.rfpr(r)))
		// }

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
