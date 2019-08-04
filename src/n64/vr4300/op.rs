/* op.i.rs(cpu) - Exposes all of the possible VR4300i opcodes and their implementations. */

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

    /* COPz i.rs(cpu) opcodes. */

    Mf,            Dmf,        Cf,            /**/        Mt,            Dmt,        Ct,            /**/
    Bc,            /**/        /**/        /**/        /**/        /**/        /**/        /**/
    Co,            /* Co */    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */
    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */    /* Co */

    /* COPz i.rt(cpu) opcodes. */

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

use n64::vr4300::*;

pub type OpF = &'static Fn(&Inst, &mut VR4300) -> u64;

pub type OpTup = (Op, &'static str, OpC, OpF);

const RESERVED: OpTup = (Op::Reserved, "reserved", OpC::R, &|i, cpu| {
    unimplemented!()
});

/* A constant 2-d array of the opcode values. */

pub const OP_TABLE: [[&OpTup; 8]; 8] = [

    /* ROW: 0 */

    [&(Op::Special, "special", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::RegImm, "regimm", OpC::I, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::J, "j", OpC::J, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Jal, "jal", OpC::J, &|i, cpu| {
        unimplemented!()
    }),

    // Branches to the branch address if register i.rs(cpu) equals to i.rt(cpu).
    &(Op::Beq, "beq", OpC::B, &|i, cpu| {
        if i.rt(cpu) == i.rs(cpu) { 1 } else { 0 }
    }),

    // Branches to the branch address if register i.rs(cpu) is not equal to i.rt(cpu).
    &(Op::Bne, "bne", OpC::B, &|i, cpu| {
        if i.rt(cpu) != i.rs(cpu) { 1 } else { 0 }
    }),

    // Branches to the branch address if register i.rs(cpu) is less than 0.
    &(Op::Blez, "blez", OpC::B, &|i, cpu| {
        if (i.rs(cpu) as i64) < 0 { 1 } else { 0 }
    }),

    // Branches to the branch address if register i.rs(cpu) is greater than 0.
    &(Op::Bgtz, "bgtz", OpC::B, &|i, cpu| {
        if (i.rs(cpu) as i64) > 0 { 1 } else { 0 }
    })],

    /* ROW: 1 */

    // Sign-extends the 16-bit immediate and adds it to register i.rs(cpu). Stores the 32-bit result to register i.rt(cpu) (sign-extends the result in the 64-bit mode).
    // Generates an exception if a 2's complement integer overflow occui.rs(cpu).
    [&(Op::Addi, "addi", OpC::I, &|i, cpu| {
        (i.rs(cpu) as i64 + i.imm() as i16 as i64) as u64
    }),

    // Sign-extends the 16-bit immediate and adds it to register i.rs(cpu). Stores the 32-bit result to register i.rt(cpu) (sign-extends the result in the 64-bit mode).
    // Does not generate an exception even if an integer overflow occui.rs(cpu).
    &(Op::Addiu, "addiu", OpC::I, &|i, cpu| {
        (i.rs(cpu) as i64 + i.imm() as i16 as i64) as u64
    }),

    // Sign-extends the 16-bit immediate and compares it with register i.rs(cpu) as a signed integer. If i.rs(cpu) is less than the immediate, stores 1 to register i.rt(cpu); otherwise, stores 0 to register i.rt(cpu).
    &(Op::Slti, "slti", OpC::I, &|i, cpu| {
        if (i.rs(cpu) as i64) < i.imm() as i16 as i64 { 1 } else { 0 }
    }),

    // Sign-extends the 16-bit immediate and compares it with register i.rs(cpu) as an unsigned integer. If i.rs(cpu) is less than the immediate, stores 1 to register i.rt(cpu); otherwise, stores 0 to register i.rt(cpu).
    &(Op::Sltiu, "sltiu", OpC::I, &|i, cpu| {
        if (i.rs(cpu) as u64) < i.imm() as i16 as i64 as u64 { 1 } else { 0 }
    }),

    // Zero-extends the 16-bit immediate, ANDs it with register i.rs(cpu), and stores the result to register i.rt(cpu).
    &(Op::Andi, "andi", OpC::I, &|i, cpu| {
        (i.rs(cpu) as u64) & (i.imm() as u64)
    }),

    // Zero-extends the 16-bit immediate, ORs it with register i.rs(cpu), and stores the result to register i.rt(cpu).
    &(Op::Ori, "ori", OpC::I, &|i, cpu| {
        (i.rs(cpu) as u64) | (i.imm() as u64)
    }),

    // Zero-extends the 16-bit immediate, exclusive-ORs it with register i.rs(cpu), and stores the result to register i.rt(cpu).
    &(Op::Xori, "xori", OpC::I, &|i, cpu| {
        (i.rs(cpu) as u64) ^ (i.imm() as u64)
    }),

    // Shifts the 16-bit immediate 16 bits to the left, and cleai.rs(cpu) the low-order 16 bits of the word to 0.
    // Stores the result to register i.rt(cpu) (by sign-extending the result in the 64-bit mode).
    &(Op::Lui, "lui", OpC::I, &|i, cpu| {
        (((i.imm() as u32) << 16) & !0xFFFF) as i32 as i64 as u64
    })],

    /* ROW: 2 */

    [&(Op::Cop0, "cop0", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Cop1, "cop1", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Cop2, "cop2", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,

    // Branches to the branch address if registers i.rs(cpu) and i.rt(cpu) are equal. If the branch condition is not satisfied, the instruction in the branch delay slot is discarded.
    &(Op::Beql, "beql", OpC::B, &|i, cpu| {
        if i.rt(cpu) == i.rs(cpu) { 1 } else { 0 }
    }),

    // Branches to the branch address if registers i.rs(cpu) and i.rt(cpu) are not equal. If the branch condition is not satisfied, the instruction in the branch delay slot is discarded.
    &(Op::Bnel, "bnel", OpC::B, &|i, cpu| {
        if i.rt(cpu) != i.rs(cpu) { 1 } else { 0 }
    }),

    // Branches to the branch address if register i.rs(cpu) is less than 0. If the branch condition is not satisfied, the instruction in the branch delay slot is discarded.
    &(Op::Blezl, "blezl", OpC::B, &|i, cpu| {
        if (i.rs(cpu) as i64) < 0 { 1 } else { 0 }
    }),

    &(Op::Bgtzl, "bgtzl", OpC::B, &|i, cpu| {
        if (i.rs(cpu) as i64) > 0 { 1 } else { 0 }
    })],

    /* ROW: 3 */

    [&(Op::Daddi, "daddi", OpC::I, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Daddiu, "daddiu", OpC::I, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Ldl, "ldl", OpC::L, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Ldr, "ldr", OpC::L, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,
    &RESERVED,
    &RESERVED,
    &RESERVED],

    /* ROW: 4 */

    [&(Op::Lb, "lb", OpC::L, &|i, cpu| {
        let val = 0;
        (val & 0xff) as i8 as i64 as u64
    }),

    &(Op::Lh, "lh", OpC::L, &|i, cpu| {
        let val = 0;
        (val & 0xffff) as i16 as i64 as u64
     }),

    &(Op::Lwl, "lwl", OpC::L, &|i, cpu| {
        let val = 0;
        unimplemented!()
    }),

    &(Op::Lw, "lw", OpC::L, &|i, cpu| {
        let val = 0;
        val as i32 as i64 as u64
    }),

    &(Op::Lbu, "lbu", OpC::L, &|i, cpu| {
        let val = 0;
        (val & 0xff) as u8 as u64
    }),

    &(Op::Lhu, "lhu", OpC::L, &|i, cpu| {
        let val = 0;
        (val & 0xffff) as u16 as u64
    }),

    &(Op::Lwr, "lwr", OpC::L, &|i, cpu| {
        let val = 0;
        unimplemented!()
    }),

    &(Op::Lwu, "lwu", OpC::L, &|i, cpu| {
        let val = 0;
        val as u32 as u64
    })],

    /* ROW: 5 */

    [&(Op::Sb, "sb", OpC::S, &|i, cpu| {
        (i.rt(cpu) & 0xff) as u64
    }),

    &(Op::Sh, "sh", OpC::S, &|i, cpu| {
        (i.rt(cpu) & 0xffff) as u64
    }),

    &(Op::Swl, "swl", OpC::S, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Sw, "sw", OpC::S, &|i, cpu| {
        i.rt(cpu) as u32 as u64
    }),

    &(Op::Sdl, "sdl", OpC::S, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Sdr, "sdr", OpC::S, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Swr, "swr", OpC::S, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Cache, "cache", OpC::I, &|i, cpu| {
        unimplemented!()
    })],

    /* ROW: 6 */

    [&(Op::Ll, "ll", OpC::L, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Lwc1, "lwc1", OpC::L, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Lwc2, "lwc2", OpC::L, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Lld, "lld", OpC::L, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Ldc1, "ldc1", OpC::L, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Ldc2, "ldc2", OpC::L, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Ld, "ld", OpC::L, &|i, cpu| {
        unimplemented!()
    })],

    /* ROW: 7 */

    [&(Op::Sc, "sc", OpC::S, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Swc1, "swc1", OpC::S, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Swc2, "swc2", OpC::S, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Scd, "scd", OpC::S, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Sdc1, "sdc1", OpC::S, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Sdc2, "sdc2", OpC::S, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Sd, "sd", OpC::S, &|i, cpu| 0)],
];

/* A constant 2-d array of the opcode values. */
pub const SP_OP_TABLE: [[&OpTup; 8]; 8] = [

    /* ROW: 0 */

    [&(Op::Sll, "sll", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Srl, "srl", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Sra, "sra", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Sllv, "sllv", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Srlv, "srlv", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Srav, "srav", OpC::R, &|i, cpu| {
        unimplemented!()
    })],

    /* ROW: 1 */

    [&(Op::Jr, "jr", OpC::J, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Jalr, "jalr", OpC::J, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,
    &RESERVED,

    &(Op::Syscall, "syscall", OpC::R, &|i, cpu| {
        let result = if i.rt(cpu) == 16 { "Pass" }  else { "Fail" };
        println!("Test Result - ISA:{:X}  Set:{:X}  Test:{:X}  Result:{:?}", i.rs(cpu), i.rd(cpu), i.sa(), result);
        0
    }),

    &(Op::Brk, "brk", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Sync, "sync", OpC::R, &|i, cpu| {
        unimplemented!()
    })],

    /* ROW: 2 */

    [&(Op::Mfhi, "mfhi", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Mthi, "mthi", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Mflo, "mflo", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Mtlo, "mtlo", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dsllv, "dsllv", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Reserved, "resered", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dsrlv, "dsrlv", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dsrav, "dsrav", OpC::R, &|i, cpu| {
        unimplemented!()
    })],

    /* ROW: 3 */

    [&(Op::Mult, "mult", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Multu, "multu", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Div, "div", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Divu, "divu", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dmult, "dmult", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dmultu, "dmultu", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Ddiv, "ddiv", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Ddivu    , "ddivu", OpC::R, &|i, cpu| {
        unimplemented!()
    })],

    /* ROW: 4 */

    // Adds the contents of register rs and rt, and stores (sign-extends in the 64-bit mode) the 32-bit result to register rd.
    // Generates an exception if an integer overflow occurs.
    [&(Op::Add, "add", OpC::R, &|i, cpu| {
        i.rs(cpu) + i.rt(cpu)
    }),

    &(Op::Addu, "addu", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Sub, "sub", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Subu, "subu", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::And, "and", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Or, "or", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Xor, "xor", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Nor, "nor", OpC::R, &|i, cpu| {
        unimplemented!()
    })],

    /* ROW: 5 */

    [&RESERVED,

    &RESERVED,

    &(Op::Slt, "slt", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Sltu, "sltu", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dadd, "dadd", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Daddu, "daddu", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dsub, "dsub", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dsubu, "dsubu", OpC::R, &|i, cpu| {
        unimplemented!()
    })],

    /* ROW: 6 */

    [&(Op::Tge, "tge", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Tgeu, "tgeu", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Tlt, "tlt", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Tltu, "tltu", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Teq, "teq", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Tne, "tne", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED],

    /* ROW: 7 */

    [&(Op::Dsll, "dsll", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Dsrl, "dsrl", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dsra, "dsra", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dsll32, "dsll32", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Dsrl32, "dsrl32", OpC::R, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dsra32, "dsra32", OpC::R, &|i, cpu| {
        unimplemented!()
    })],
];

/* A constant 2-d array of the opcode values. , _*/
pub const RI_OP_TABLE: [[&OpTup; 8]; 4] = [

    /* ROW: 0 */

    [&(Op::Bltz, "bltz", OpC::B, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Bgez, "bgez", OpC::B, &|i, cpu| {
        unimplemented!()
    }),

    // Branches to the branch address if register i.rs(cpu) is less than 0. If the branch condition is not satisfied, the instruction in the branch delay slot is discarded.
    &(Op::Bltzl, "bltzl", OpC::B, &|i, cpu| {
        if (i.rs(cpu) as i64) < 0 { 1 } else { 0 }
    }),

    // Branches to the branch address if register i.rs(cpu) is greater than 0. If the branch condition is not satisfied, the instruction in the branch delay slot is discarded.
    &(Op::Bgezl, "bgezl", OpC::B, &|i, cpu| {
        if (i.rs(cpu) as i64) > 0 { 1 } else { 0 }
    }),

    &RESERVED,
    &RESERVED,
    &RESERVED,
    &RESERVED],

    /* ROW: 1 */

    [&(Op::Tgei, "tgei", OpC::I, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Tgeiu, "tgeiu", OpC::I, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Tlti, "tlti", OpC::I, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Tltiu, "tltiu", OpC::I, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Teqi, "teqi", OpC::I, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Tnei, "tnei", OpC::I, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED],

    /* ROW: 2 */

    [&(Op::Bltzal, "bltzal", OpC::B, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Bgezal, "bgezal", OpC::B, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Bltzall, "bltzall", OpC::B, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Bgezall, "bgezall", OpC::B, &|i, cpu| {
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

/* A constant 2-d array of the opcode values. */
pub const COP_OP_RS_TABLE: [[&OpTup; 8]; 4] = [

    /* ROW: 0 */

    [&(Op::Mf, "mf", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dmf, "dmf", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Cf, "cf", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,

    &(Op::Mt, "mt", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Dmt, "dmt", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Ct, "ct", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED],

    /* ROW: 4 */

    [&(Op::Bc, "bc", OpC::C, &|i, cpu| {
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

    [&(Op::Co, "co", OpC::C, &|i, cpu| {
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

    [&(Op::Bcf, "bcf", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Bct, "bct", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Bcfl, "bcfl", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Bctl, "bctl", OpC::C, &|i, cpu| {
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

    &(Op::Tlbr, "tlbr", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &(Op::Tlbwi, "tlbwi", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED,
    &RESERVED,
    &RESERVED,

    &(Op::Tlbwr, "tlbwr", OpC::C, &|i, cpu| {
        unimplemented!()
    }),

    &RESERVED],

    /* ROW: 1 */

    [&(Op::Tlbp, "tlbp", OpC::C, &|i, cpu| {
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

    [&(Op::Eret, "eret", OpC::C, &|i, cpu| {
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
