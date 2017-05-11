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

use std::fmt;

use n64::mc::MC;

mod op;
use self::op::*;

mod instruction;
use self::instruction::Inst;

mod cp0;
use self::cp0::CP0;

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
		/* Increment the program counter. */
		self.pc += 4;
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
		/* Increment the program counter. */
		self.pc += 4;
	}
	/* Handler for the jump (J-Type) instructions. */
	fn exec_jump(&mut self, i: Inst) {

	}
	/* Handler for the branch subclass of the J-type instructions. */
	fn exec_branch(&mut self, i: Inst) {
		/* Obtain the value of rs. */
		let rs = self.rgpr(i.rs());
		/* Obtain the value of rt. */
		let rt = self.rgpr(i.rt());
		/* Obtain the offset address. */
		let offset = (i.offset() as i16 as i64) << 2;
		/* Determine whether or not the branch should occur .*/
		let should_branch = i.function()(rt, rs, 0);
		/* Perform the branch. */
		if should_branch > 0{
			self.pc = (self.pc as i64 + offset) as u64;
		} else {
			/* Increment the program counter. */
			self.pc += 4;
		}
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
		/* Increment the program counter. */
		self.pc += 4;
	}

	/* Executes an instruction. */
	pub fn cycle(&mut self, mc: &mut MC) {
		/* Fetch the next instrution from memory. */
		let i = Inst(mc.read(self.pc as usize));
		/* Print the opcode. */
		println!("{:#x}: ({:#x}) {}", self.pc, i.0, i);
		/* Execute the instrution. */
		match i.op() {
			/* Handle the jump instructions here. */
			Op::J => {
				/* Obtain the jump target. */
				let target = i.target();
				/* Load the target into the program counter. */
				self.pc = target;
			},
			Op::Jal => {
				/* Obtain the jump target. */
				let target = i.target();
				/* Load the program counter into the return address. */
				let pc = self.pc;
				self.wgpr(pc, 31);
				/* Load the target into the program counter. */
				self.pc = target;
			},
			Op::Jr => {
				/* Obtain the jump target. */
				let target = self.rgpr(i.rs());
				/* Load the program counter into the return address. */
				self.pc = target;
			},
			Op::Jalr => {
				/* Obtain the jump target. */
				let target = self.rgpr(i.rs());
				/* Load the program counter into the destination register. */
				let pc = self.pc;
				self.wgpr(pc, i.rd());
				/* Load the program counter into the return address. */
				self.pc = target;
			},
			Op::Reserved => panic!("Attempt made to execute a reserved instruction {:#x}.", i.opcode()),
			_ => match i.class() {
				OpC::I =>
					self.exec_imm(i),
				OpC::L | OpC::S =>
					self.exec_ldst(mc, i),
				OpC::J =>
					self.exec_jump(i),
				OpC::B =>
					self.exec_branch(i),
				OpC::R =>
					self.exec_reg(i),
			}
		}
		//println!("{:?}", self);
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
