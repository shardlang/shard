use super::module::*;
use super::typ::*;
use super::value::*;

// Instruction Specification

// https://webassembly.github.io/spec/core/syntax/instructions.html

pub enum Instr {
    // Const Instructions -----------------------
    I32Const(U32),

    I64Const(U64),

    F32Const(F32),

    F64Const(F64),

    // Test Instructions ------------------------
    I32Eqz,

    I64Eqz,

    // Comparison Instructions ------------------
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    // Unary Instructions -----------------------
    I32Clz,
    I32Ctz,
    I32Popcnt,

    I64Clz,
    I64Ctz,
    I64Popcnt,

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,

    // Binary Instructions ----------------------
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,

    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,

    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,

    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,

    // Conversion Instructions ------------------
    I32WrapI64,

    I64ExtendI32S,
    I64ExtendI32U,

    F32DemoteF64,

    F64PromoteF32,

    I32TruncF32S,
    I32TruncF32U,
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32ReinterpretF32,

    I32TruncF64S,
    I32TruncF64U,
    I32TruncSatF64S,
    I32TruncSatF64U,

    I64TruncF32S,
    I64TruncF32U,
    I64TruncSatF32S,
    I64TruncSatF32U,

    I64TruncF64S,
    I64TruncF64U,
    I64TruncSatF64S,
    I64TruncSatF64U,
    I64ReinterpretF64,

    F32ConvertI32S,
    F32ConvertI32U,
    F32ReinterpretI32,

    F32ConvertI64S,
    F32ConvertI64U,

    F64ConvertI32S,
    F64ConvertI32U,

    F64ConvertI64S,
    F64ConvertI64U,
    F64ReinterpretI64,

    I32Extend8S,
    I32Extend16S,

    I64Extend8S,
    I64Extend16S,
    I64Extend32S,

    // Reference Instructions ---------------------
    RefNull(RefType),
    RefIsNull,
    RefFunc(FuncIdx),

    // Parametric Instructions --------------------
    Drop,
    Select(Option<Vec<ValType>>),

    // Variable Instructions ----------------------
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),

    // Table Instructions -------------------------
    TableGet(TableIdx),
    TableSet(TableIdx),
    TableSize(TableIdx),
    TableGrow(TableIdx),
    TableFill(TableIdx),
    TableCopy(TableIdx, TableIdx),
    TableInit(TableIdx, ElemIdx),
    ElemDrop(ElemIdx),

    // Memory Instructions ------------------------
    I32Load(MemArg),
    I64Load(MemArg),
    F32Load(MemArg),
    F64Load(MemArg),

    I32Store(MemArg),
    I64Store(MemArg),
    F32Store(MemArg),
    F64Store(MemArg),

    I32Load8U(MemArg),
    I32Load8S(MemArg),
    I64Load8U(MemArg),
    I64Load8S(MemArg),

    I32Load16U(MemArg),
    I32Load16S(MemArg),
    I64Load16U(MemArg),
    I64Load16S(MemArg),

    I64Load32U(MemArg),
    I64Load32S(MemArg),

    I32Store8(MemArg),
    I64Store8(MemArg),

    I32Store16(MemArg),
    I64Store16(MemArg),

    I64Store32(MemArg),

    MemorySize,
    MemoryGrow,
    MemoryFill,
    MemoryCopy,
    MemoryInit(DataIdx),
    DataDrop(DataIdx),

    // Control Instructions -----------------------
    Nop,
    Unreachable,
    Block(BlockType, Vec<Instr>),
    Loop(BlockType, Vec<Instr>),
    If(BlockType, Vec<Instr>, Vec<Instr>),
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable(Vec<LabelIdx>, LabelIdx),
    Return,
    Call(FuncIdx),
    CallIndirect(TableIdx, TypeIdx),
}

pub struct MemArg {
    pub offset: U32,
    pub align: U32,
}

pub enum BlockType {
    TypeIdx(TypeIdx),
    ValType(ValType),
    None,
}

pub struct Expr {
    pub instrs: Vec<Instr>,
}

impl Expr {
    pub fn new() -> Expr {
        Expr { instrs: Vec::new() }
    }

    pub fn append(&mut self, other: &mut Expr) {
        self.instrs.append(&mut other.instrs)
    }
}

impl From<Vec<Instr>> for Expr {
    fn from(input: Vec<Instr>) -> Expr {
        Expr { instrs: input }
    }
}
impl From<Instr> for Expr {
    fn from(input: Instr) -> Expr {
        Expr {
            instrs: vec![input],
        }
    }
}
