/* op.p.rf.rs - Exposes all of the possible VR4300i opcodes and their implementations. */

#[derive(Copy, Clone)]
pub enum OpC {
    /* immediate instruction (i-type) */
    I,
    /* load instruction (subset of i-type) */
    L,
    /* store instruction (subset of i-type) */
    S,
    /* jump instruction (j-type) */
    J,
    /* branch instruction (subset of j-type) */
    B,
    /* register instruction */
    R,
    /* coprocessor instruction */
    C
}

/* Valid VR4300 opcodes. Figure 16-1 in NEC VR4300. */
#[derive(Copy, Clone)]
pub enum Op {

    /* Unique opcodes */

    Special,    RegImm,     J,          Jal,        Beq,        Bne,        Blez,       Bgtz,
    Addi,       Addiu,      Slti,       Sltiu,      Andi,       Ori,        Xori,       Lui,
    Cop0,       Cop1,       Cop2,       /**/        Beql,       Bnel,       Blezl,      Bgtzl,
    Daddi,      Daddiu,     Ldl,        Ldr,        /**/        /**/        /**/        /**/
    Lb,         Lh,         Lwl,        Lw,         Lbu,        Lhu,        Lwr,        Lwu,
    Sb,         Sh,         Swl,        Sw,         Sdl,        Sdr,        Swr,        Cache,
    Ll,         Lwc1,       Lwc2,       /**/        Lld,        Ldc1,       Ldc2,       Ld,
    Sc,         Swc1,       Swc2,       /**/        Scd,        Sdc1,       Sdc2,       Sd,

    /* Special opcodes */

    Sll,        /**/        Srl,        Sra,        Sllv,       /**/        Srlv,       Srav,
    Jr,         Jalr,       /**/        /**/        Syscall,    Brk,        /**/        Sync,
    Mfhi,       Mthi,       Mflo,       Mtlo,       Dsllv,      /**/        Dsrlv,      Dsrav,
    Mult,       Multu,      Div,        Divu,       Dmult,      Dmultu,     Ddiv,       Ddivu,
    Add,        Addu,       Sub,        Subu,       And,        Or,         Xor,        Nor,
    /**/        /**/        Slt,        Sltu,       Dadd,       Daddu,      Dsub,       Dsubu,
    Tge,        Tgeu,       Tlt,        Tltu,       Teq,        /**/        Tne,        /**/
    Dsll,       /**/        Dsrl,       Dsra,       Dsll32,     /**/        Dsrl32,     Dsra32,

    /* RegImm opcodes. */

    Bltz,       Bgez,       Bltzl,      Bgezl,      /**/        /**/        /**/        /**/
    Tgei,       Tgeiu,      Tlti,       Tltiu,      Teqi,       /**/        Tnei,       /**/
    Bltzal,     Bgezal,     Bltzall,    Bgezall,    /**/        /**/        /**/        /**/
    /**/        /**/        /**/        /**/        /**/        /**/        /**/        /**/

    /* COPz p.rf.rs opcodes. */

    Mf,            Dmf,        Cf,            /**/        Mt,            Dmt,        Ct,            /**/
    Bc,            /**/        /**/        /**/        /**/        /**/        /**/        /**/
    Co,            /* Co */    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */
    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */

    /* COPz p.rf.rt opcodes. */

    Bcf,        Bct,        Bcfl,        Bctl,        /**/        /**/        /**/        /**/
    /**/        /**/        /**/        /**/        /**/        /**/        /**/        /**/
    /**/        /**/        /**/        /**/        /**/        /**/        /**/        /**/
    /**/        /**/        /**/        /**/        /**/        /**/        /**/        /**/

    /* CP0 opcodes */

    /**/        Tlbr,        Tlbwi,        /**/        /**/        /**/        Tlbwr,        /**/
    Tlbp,        /**/        /**/        /**/        /**/        /**/        /**/        /**/
    /**/        /**/        /**/        /**/        /**/        /**/        /**/        /**/
    Eret,        /**/        /**/        /**/        /**/        /**/        /**/        /**/
    /**/        /**/        /**/        /**/        /**/        /**/        /**/        /**/
    /**/        /**/        /**/        /**/        /**/        /**/        /**/        /**/
    /**/        /**/        /**/        /**/        /**/        /**/        /**/        /**/
    /**/        /**/        /**/        /**/        /**/        /**/        /**/        /**/

    Reserved
}

use super::*;

pub type OpF = &'static Fn(&mut Pl);

pub type OpTup = (Op, &'static str, OpC, OpF);

const RESERVED: OpTup = (Op::Reserved, "reserved", OpC::R, &|p| {
    unimplemented!()
});

/* A constant 2-d array of the opcode p.dc.dcues. */

pub const OP_TABLE: [[&OpTup; 8]; 8] = [

    /* ROW: 0 */

    [&(Op::Special, "special", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::RegImm, "regimm", OpC::I, &|p| {
        unimplemented!()
    }),

    &(Op::J, "j", OpC::J, &|p| {
        p.ex.ol = p.ic.op.target();
    }),

    &(Op::Jal, "jal", OpC::J, &|p| {
        p.ex.ol = p.ic.op.target();
        p.ex.wlr = true;
    }),

    // Branches to the branch address if register p.rf.rs equals to p.rf.rt.
    &(Op::Beq, "beq", OpC::B, &|p| {
        p.ex.br = if p.rf.rt == p.rf.rs { true } else { false };
    }),

    // Branches to the branch address if register p.rf.rs is not equal to p.rf.rt.
    &(Op::Bne, "bne", OpC::B, &|p| {
        p.ex.br = if p.rf.rt != p.rf.rs { true } else { false };
    }),

    // Branches to the branch address if register p.rf.rs is less than 0.
    &(Op::Blez, "blez", OpC::B, &|p| {
        p.ex.br = if (p.rf.rs as i64) < 0 { true } else { false };
    }),

    // Branches to the branch address if register p.rf.rs is greater than 0.
    &(Op::Bgtz, "bgtz", OpC::B, &|p| {
        p.ex.br = if (p.rf.rs as i64) > 0 { true } else { false };
    })],

    /* ROW: 1 */

    // Sign-extends the 16-bit immediate and adds it to register p.rf.rs. Stores the 32-bit result to register p.rf.rt (sign-extends the result in the 64-bit mode).
    // Generates an exception if a 2's complement integer overflow occup.rf.rs.
    [&(Op::Addi, "addi", OpC::I, &|p| {
        p.ex.ol = (p.rf.rs as i64 + p.ic.op.imm() as i16 as i64) as u64
    }),

    // Sign-extends the 16-bit immediate and adds it to register p.rf.rs. Stores the 32-bit result to register p.rf.rt (sign-extends the result in the 64-bit mode).
    // Does not generate an exception even if an integer overflow occup.rf.rs.
    &(Op::Addiu, "addiu", OpC::I, &|p| {
        p.ex.ol = (p.rf.rs as i64 + p.ic.op.imm() as i16 as i64) as u64
    }),

    // Sign-extends the 16-bit immediate and compares it with register p.rf.rs as a signed integer. If p.rf.rs is less than the immediate, stores 1 to register p.rf.rt; otherwise, stores 0 to register p.rf.rt.
    &(Op::Slti, "slti", OpC::I, &|p| {
        p.ex.ol = if (p.rf.rs as i64) < p.ic.op.imm() as i16 as i64 { 1 } else { 0 }
    }),

    // Sign-extends the 16-bit immediate and compares it with register p.rf.rs as an unsigned integer. If p.rf.rs is less than the immediate, stores 1 to register p.rf.rt; otherwise, stores 0 to register p.rf.rt.
    &(Op::Sltiu, "sltiu", OpC::I, &|p| {
        p.ex.ol = if (p.rf.rs as u64) < p.ic.op.imm() as i16 as i64 as u64 { 1 } else { 0 }
    }),

    // Zero-extends the 16-bit immediate, ANDs it with register p.rf.rs, and stores the result to register p.rf.rt.
    &(Op::Andi, "andi", OpC::I, &|p| {
        p.ex.ol = (p.rf.rs as u64) & (p.ic.op.imm() as u64)
    }),

    // Zero-extends the 16-bit immediate, ORs it with register p.rf.rs, and stores the result to register p.rf.rt.
    &(Op::Ori, "ori", OpC::I, &|p| {
        p.ex.ol = (p.rf.rs as u64) | (p.ic.op.imm() as u64)
    }),

    // Zero-extends the 16-bit immediate, exclusive-ORs it with register p.rf.rs, and stores the result to register p.rf.rt.
    &(Op::Xori, "xori", OpC::I, &|p| {
        p.ex.ol = (p.rf.rs as u64) ^ (p.ic.op.imm() as u64)
    }),

    // Shifts the 16-bit immediate 16 bits to the left, and cleap.rf.rs the low-order 16 bits of the word to 0.
    // Stores the result to register p.rf.rt (by sign-extending the result in the 64-bit mode).
    &(Op::Lui, "lui", OpC::I, &|p| {
        p.ex.ol = (((p.ic.op.imm() as u32) << 16) & !0xFFFF) as i32 as i64 as u64
    })],

    /* ROW: 2 */

    [&(Op::Cop0, "cop0", OpC::C, &|p| {
        unimplemented!()
    }),

    &(Op::Cop1, "cop1", OpC::C, &|p| {
        unimplemented!()
    }),

    &(Op::Cop2, "cop2", OpC::C, &|p| {
        unimplemented!()
    }),

    &RESERVED,

    // Branches to the branch address if registers p.rf.rs and p.rf.rt are equal. If the branch condition is not satisfied, the instruction in the branch delay slot is discarded.
    &(Op::Beql, "beql", OpC::B, &|p| {
        p.ex.br = if p.rf.rt == p.rf.rs { true } else { false };
    }),

    // Branches to the branch address if registers p.rf.rs and p.rf.rt are not equal. If the branch condition is not satisfied, the instruction in the branch delay slot is discarded.
    &(Op::Bnel, "bnel", OpC::B, &|p| {
        p.ex.br = if p.rf.rt != p.rf.rs { true } else { false };
    }),

    // Branches to the branch address if register p.rf.rs is less than 0. If the branch condition is not satisfied, the instruction in the branch delay slot is discarded.
    &(Op::Blezl, "blezl", OpC::B, &|p| {
        p.ex.br = if (p.rf.rs as i64) < 0 { true } else { false };
    }),

    &(Op::Bgtzl, "bgtzl", OpC::B, &|p| {
        p.ex.br = if (p.rf.rs as i64) > 0 { true } else { false };
    })],

    /* ROW: 3 */

    [&(Op::Daddi, "daddi", OpC::I, &|p| {
        unimplemented!()
    }),

    &(Op::Daddiu, "daddiu", OpC::I, &|p| {
        unimplemented!()
    }),

    &(Op::Ldl, "ldl", OpC::L, &|p| {
        unimplemented!()
    }),

    &(Op::Ldr, "ldr", OpC::L, &|p| {
        unimplemented!()
    }),

    &RESERVED,
    &RESERVED,
    &RESERVED,
    &RESERVED],

    /* ROW: 4 */

    // Generates an address by adding a sign-extended offset to the contents of register base.
    // Sign-extends the contents of a byte specified by the address and loads the result to register rt.
    [&(Op::Lb, "lb", OpC::L, &|p| {
        p.ex.ol  = (p.dc.dc & 0xff) as i8 as i64 as u64
    }),

    // Generates an address by adding a sign-extended offset to the contents of register base.
    // Sign-extends the contents of a halfword specified by the address and loads the result to register rt.
    &(Op::Lh, "lh", OpC::L, &|p| {
        p.ex.ol  = (p.dc.dc & 0xffff) as i16 as i64 as u64
     }),

    // Generates an address by adding a sign-extended offset to the contents of register base.
    // Shifts a word specified by the address to the left, so that a byte specified by the address is at the leftmost position of the word. Sign-extends (in the 64bit mode), merges the result of the shift and the contents of register rt, and loads the result to register rt.
    &(Op::Lwl, "lwl", OpC::L, &|p| {
        unimplemented!()
    }),

    // Generates an address by adding a sign-extended offset to the contents of register base.
    // Sign-extends the contents of a word specified by the address (in the 64-bit mode) and loads the result to register rt.
    &(Op::Lw, "lw", OpC::L, &|p| {
        p.ex.ol  = p.dc.dc as i32 as i64 as u64
    }),

    // Generates an address by adding a sign-extended offset to the contents of register base.
    // Zero-extends the contents of a byte specified by the address and loads the result to register rt.
    &(Op::Lbu, "lbu", OpC::L, &|p| {
        p.ex.ol  = (p.dc.dc & 0xff) as u8 as u64
    }),

    //Generates an address by adding a sign-extended offset to the contents of register base.
    // Zero-extends the contents of a halfword specified by the address and loads the result to register rt.
    &(Op::Lhu, "lhu", OpC::L, &|p| {
        p.ex.ol  = (p.dc.dc & 0xffff) as u16 as u64
    }),

    // Generates an address by adding a sign-extended offset to the contents of register base.
    // Shifts a word specified by the address to the right, so that a byte specified by the address is at the rightmost position of the word.
    // Sign-extends (in the 64bit mode), merges the result of the shift and the contents of register rt, and loads the result to register rt.
    &(Op::Lwr, "lwr", OpC::L, &|p| {
        unimplemented!()
    }),

    // Generates an address by adding a sign-extended offset to the contents of register base.
    // Zero-extends the contents of the word specified by the address, and loads the result to register rt.
    &(Op::Lwu, "lwu", OpC::L, &|p| {
        p.ex.ol  = p.dc.dc as u32 as u64
    })],

    /* ROW: 5 */

    // Generates an address by adding a sign-extended offset to the contents of register base.
    // Stores the contents of the low-order byte of register rt to the memory specified by the address.
    [&(Op::Sb, "sb", OpC::S, &|p| {
        p.ex.ol = (p.rf.rt & 0xff) as u64
    }),

    // Generates an address by adding a sign-extended offset to the contents of register base.
    // Stores the contents of the low-order halfword of register rt to the memory specified by the address.
    &(Op::Sh, "sh", OpC::S, &|p| {
        p.ex.ol = (p.rf.rt & 0xffff) as u64
    }),

    // Generates an address by adding a sign-extended offset to the contents of register base.
    // Shifts the contents of register rt to the right so that the leftmost byte of the word is at the position of the byte specified by the address.
    // Stores the result of the shift to the lower portion of the word in memory.
    &(Op::Swl, "swl", OpC::S, &|p| {
        unimplemented!()
    }),

    // Generates an address by adding a sign-extended offset to the contents of register base.
    // Stores the contents of the low-order word of register rt to the memory specified by the address.
    &(Op::Sw, "sw", OpC::S, &|p| {
        p.ex.ol = p.rf.rt as u32 as u64
    }),

    &(Op::Sdl, "sdl", OpC::S, &|p| {
        unimplemented!()
    }),

    &(Op::Sdr, "sdr", OpC::S, &|p| {
        unimplemented!()
    }),

    &(Op::Swr, "swr", OpC::S, &|p| {
        unimplemented!()
    }),

    &(Op::Cache, "cache", OpC::I, &|p| {
        unimplemented!()
    })],

    /* ROW: 6 */

    [&(Op::Ll, "ll", OpC::L, &|p| {
        unimplemented!()
    }),

    &(Op::Lwc1, "lwc1", OpC::L, &|p| {
        unimplemented!()
    }),

    &(Op::Lwc2, "lwc2", OpC::L, &|p| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Lld, "lld", OpC::L, &|p| {
        unimplemented!()
    }),

    &(Op::Ldc1, "ldc1", OpC::L, &|p| {
        unimplemented!()
    }),

    &(Op::Ldc2, "ldc2", OpC::L, &|p| {
        unimplemented!()
    }),

    &(Op::Ld, "ld", OpC::L, &|p| {
        unimplemented!()
    })],

    /* ROW: 7 */

    [&(Op::Sc, "sc", OpC::S, &|p| {
        unimplemented!()
    }),

    &(Op::Swc1, "swc1", OpC::S, &|p| {
        unimplemented!()
    }),

    &(Op::Swc2, "swc2", OpC::S, &|p| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Scd, "scd", OpC::S, &|p| {
        unimplemented!()
    }),

    &(Op::Sdc1, "sdc1", OpC::S, &|p| {
        unimplemented!()
    }),

    &(Op::Sdc2, "sdc2", OpC::S, &|p| {
        unimplemented!()
    }),

    &(Op::Sd, "sd", OpC::S, &|p| {
        unimplemented!()
    })],
];

/* A constant 2-d array of the opcode p.dc.dcues. */
pub const SP_OP_TABLE: [[&OpTup; 8]; 8] = [

    /* ROW: 0 */

    // Shifts the contents of register rt sa bits to the left, and inserts 0 to the loworder bits.
    // Sign-extends (in the 64-bit mode) the 32-bit result and stores it to register rd.
    [&(Op::Sll, "sll", OpC::R, &|p| {
        p.ex.ol = ((p.rf.rt as u32) << p.ic.op.sa()) as i32 as i64 as u64
    }),

    &RESERVED,

    // Shifts the contents of register rt sa bits to the right, and inserts 0 to the highorder bits.
    // Sign-extends (in the 64-bit mode) the 32-bit result and stores it to register rd.
    &(Op::Srl, "srl", OpC::R, &|p| {
        p.ex.ol = ((p.rf.rt as u32) >> p.ic.op.sa()) as i32 as i64 as u64
    }),

    &(Op::Sra, "sra", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Sllv, "sllv", OpC::R, &|p| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Srlv, "srlv", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Srav, "srav", OpC::R, &|p| {
        unimplemented!()
    })],

    /* ROW: 1 */

    // Jumps to the address of register rs, delayed by one instruction.
    [&(Op::Jr, "jr", OpC::J, &|p| {
        p.ex.ol = p.rf.rs;
    }),

    // Jumps to the address of register rs, delayed by one instruction.
    // Stores the address of the instruction following the delay slot to register rd.
    &(Op::Jalr, "jalr", OpC::J, &|p| {
        p.ex.ol = p.rf.rs;
        p.ex.wlr = true;
    }),

    &RESERVED,
    &RESERVED,

    &(Op::Syscall, "syscall", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Brk, "brk", OpC::R, &|p| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Sync, "sync", OpC::R, &|p| {
        unimplemented!()
    })],

    /* ROW: 2 */

    [&(Op::Mfhi, "mfhi", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Mthi, "mthi", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Mflo, "mflo", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Mtlo, "mtlo", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Dsllv, "dsllv", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Reserved, "resered", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Dsrlv, "dsrlv", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Dsrav, "dsrav", OpC::R, &|p| {
        unimplemented!()
    })],

    /* ROW: 3 */

    [&(Op::Mult, "mult", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Multu, "multu", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Div, "div", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Divu, "divu", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Dmult, "dmult", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Dmultu, "dmultu", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Ddiv, "ddiv", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Ddivu    , "ddivu", OpC::R, &|p| {
        unimplemented!()
    })],

    /* ROW: 4 */

    // Adds the contents of register rs and rt, and stores (sign-extends in the 64-bit mode) the 32-bit result to register rd.
    // Generates an exception if an integer overflow occurs.
    [&(Op::Add, "add", OpC::R, &|p| {
        p.ex.ol = ((p.rf.rs as u32) + (p.rf.rt as u32)) as u64
    }),

    // Adds the contents of register rs and rt, and stores (sign-extends in the 64-bit mode) the 32-bit result to register rd.
    // Does not generate an exception even if an integer overflow occurs.
    &(Op::Addu, "addu", OpC::R, &|p| {
        p.ex.ol = ((p.rf.rs as u32) + (p.rf.rt as u32)) as u64
    }),

    // Subtracts the contents of register rs from register rt, and stores (sign-extends in the 64-bit mode) the result to register rd.
    // Generates an exception if an integer overflow occurs.
    &(Op::Sub, "sub", OpC::R, &|p| {
        p.ex.ol = ((p.rf.rt as u32) - (p.rf.rs as u32)) as u64
    }),

    &(Op::Subu, "subu", OpC::R, &|p| {
        p.ex.ol = ((p.rf.rt as u32) - (p.rf.rs as u32)) as u64
    }),

    // ANDs the contents of registers rs and rt in bit units, and stores the result to register rd.
    &(Op::And, "and", OpC::R, &|p| {
        p.ex.ol = p.rf.rs & p.rf.rt
    }),

    // ORs the contents of registers rs and rt in bit units, and stores the result to register rd.
    &(Op::Or, "or", OpC::R, &|p| {
        p.ex.ol = p.rf.rs | p.rf.rt
    }),

    // Exclusive-ORs the contents of registers rs and rt in bit units, and stores the result to register rd.
    &(Op::Xor, "xor", OpC::R, &|p| {
        p.ex.ol = p.rf.rs ^ p.rf.rt
    }),

    // NORs the contents of registers rs and rt in bit units, and stores the result to register rd.
    &(Op::Nor, "nor", OpC::R, &|p| {
        p.ex.ol = !(p.rf.rs | p.rf.rt)
    })],

    /* ROW: 5 */

    [&RESERVED,

    &RESERVED,

    &(Op::Slt, "slt", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Sltu, "sltu", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Dadd, "dadd", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Daddu, "daddu", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Dsub, "dsub", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Dsubu, "dsubu", OpC::R, &|p| {
        unimplemented!()
    })],

    /* ROW: 6 */

    [&(Op::Tge, "tge", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Tgeu, "tgeu", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Tlt, "tlt", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Tltu, "tltu", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Teq, "teq", OpC::R, &|p| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Tne, "tne", OpC::R, &|p| {
        unimplemented!()
    }),

    &RESERVED],

    /* ROW: 7 */

    [&(Op::Dsll, "dsll", OpC::R, &|p| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Dsrl, "dsrl", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Dsra, "dsra", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Dsll32, "dsll32", OpC::R, &|p| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Dsrl32, "dsrl32", OpC::R, &|p| {
        unimplemented!()
    }),

    &(Op::Dsra32, "dsra32", OpC::R, &|p| {
        unimplemented!()
    })],
];

/* A constant 2-d array of the opcode p.dc.dcues. , _*/
pub const RI_OP_TABLE: [[&OpTup; 8]; 4] = [

    /* ROW: 0 */

    [&(Op::Bltz, "bltz", OpC::B, &|p| {
        p.ex.br = if (p.rf.rs as i64) < 0 { true } else { false };
    }),

    &(Op::Bgez, "bgez", OpC::B, &|p| {
        p.ex.br = if (p.rf.rs as i64) >= 0 { true } else { false };
    }),

    // Branches to the branch address if register p.rf.rs is less than 0. If the branch condition is not satisfied, the instruction in the branch delay slot is discarded.
    &(Op::Bltzl, "bltzl", OpC::B, &|p| {
        p.ex.br = if (p.rf.rs as i64) < 0 { true } else { false };
    }),

    // Branches to the branch address if register p.rf.rs is greater than 0. If the branch condition is not satisfied, the instruction in the branch delay slot is discarded.
    &(Op::Bgezl, "bgezl", OpC::B, &|p| {
        p.ex.br = if (p.rf.rs as i64) > 0 { true } else { false };
    }),

    &RESERVED,
    &RESERVED,
    &RESERVED,
    &RESERVED],

    /* ROW: 1 */

    [&(Op::Tgei, "tgei", OpC::I, &|p| {
        unimplemented!()
    }),

    &(Op::Tgeiu, "tgeiu", OpC::I, &|p| {
        unimplemented!()
    }),

    &(Op::Tlti, "tlti", OpC::I, &|p| {
        unimplemented!()
    }),

    &(Op::Tltiu, "tltiu", OpC::I, &|p| {
        unimplemented!()
    }),

    &(Op::Teqi, "teqi", OpC::I, &|p| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Tnei, "tnei", OpC::I, &|p| {
        unimplemented!()
    }),

    &RESERVED],

    /* ROW: 2 */

    [&(Op::Bltzal, "bltzal", OpC::B, &|p| {
        p.ex.br = if (p.rf.rs as i64) < 0 { true } else { false };
        p.ex.wlr = true;
    }),

    &(Op::Bgezal, "bgezal", OpC::B, &|p| {
        p.ex.br = if (p.rf.rs as i64) >= 0 { true } else { false };
        p.ex.wlr = true;
    }),

    &(Op::Bltzall, "bltzall", OpC::B, &|p| {
        p.ex.br = if (p.rf.rs as i64) < 0 { true } else { false };
        p.ex.wlr = true;
        unimplemented!()
    }),

    &(Op::Bgezall, "bgezall", OpC::B, &|p| {
        p.ex.wlr = true;
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
    &RESERVED],
];

/* A constant 2-d array of the opcode p.dc.dcues. */
pub const COP_OP_RS_TABLE: [[&OpTup; 8]; 4] = [

    /* ROW: 0 */

    // Loads the contents of the word of the general purpose register rd of CP0 to the general purpose register rt of the CPU.
    [&(Op::Mf, "mf", OpC::C, &|p| {
        p.ex.ol = p.rf.rs
    }),

    // Loads the contents of the doubleword of the general purpose register rd of CP0 to the general purpose register rt of the CPU.
    &(Op::Dmf, "dmf", OpC::C, &|p| {
        unimplemented!()
    }),

    &(Op::Cf, "cf", OpC::C, &|p| {
        unimplemented!()
    }),

    &RESERVED,

    // Loads the contents of the word of the general purpose register rt of the CPU to the general purpose register rd of CP0.
    &(Op::Mt, "mt", OpC::C, &|p| {
        p.ex.ol = p.rf.rt
    }),

    // Loads the contents of the doubleword of the general purpose register rt of the CPU to the general purpose register rd of CP0.
    &(Op::Dmt, "dmt", OpC::C, &|p| {
        unimplemented!()
    }),

    &(Op::Ct, "ct", OpC::C, &|p| {
        unimplemented!()
    }),

    &RESERVED],

    /* ROW: 4 */

    [&(Op::Bc, "bc", OpC::C, &|p| {
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

    [&(Op::Co, "co", OpC::C, &|p| {
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

/* A constant 2-d array of the opcode p.dc.dcues. */
pub const COP_OP_RT_TABLE: [[&OpTup; 8]; 4] = [

    /* ROW: 0 */

    [&(Op::Bcf, "bcf", OpC::C, &|p| {
        unimplemented!()
    }),

    &(Op::Bct, "bct", OpC::C, &|p| {
        unimplemented!()
    }),

    &(Op::Bcfl, "bcfl", OpC::C, &|p| {
        unimplemented!()
    }),

    &(Op::Bctl, "bctl", OpC::C, &|p| {
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

/* A constant 2-d array of the opcode p.dc.dcues. */
pub const COP_OP_FN_TABLE: [[&OpTup; 8]; 4] = [

    /* ROW: 0 */

    [&RESERVED,

    &(Op::Tlbr, "tlbr", OpC::C, &|p| {
        unimplemented!()
    }),

    &(Op::Tlbwi, "tlbwi", OpC::C, &|p| {
        unimplemented!()
    }),

    &RESERVED,
    &RESERVED,
    &RESERVED,

    &(Op::Tlbwr, "tlbwr", OpC::C, &|p| {
        unimplemented!()
    }),

    &RESERVED],

    /* ROW: 1 */

    [&(Op::Tlbp, "tlbp", OpC::C, &|p| {
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

    [&(Op::Eret, "eret", OpC::C, &|p| {
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
