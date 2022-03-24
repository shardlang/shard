use super::instr::*;
use super::typ::*;
use super::value::*;

// Module Specification

// https://webassembly.github.io/spec/core/syntax/modules.html

pub struct Module {
    pub types: Vec<FuncType>,
    pub funcs: Vec<Func>,
    pub tables: Vec<Table>,
    pub mems: Vec<Mem>,
    pub globals: Vec<Global>,
    pub elems: Vec<Elem>,
    pub datas: Vec<Data>,
    pub start: Option<Start>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

pub type TypeIdx = U32;
pub type FuncIdx = U32;
pub type TableIdx = U32;
pub type MemIdx = U32;
pub type GlobalIdx = U32;
pub type ElemIdx = U32;
pub type DataIdx = U32;
pub type LocalIdx = U32;
pub type LabelIdx = U32;

pub struct Func {
    pub typ: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: Expr,
}

pub struct Table {
    pub typ: TableType,
}

pub struct Mem {
    pub typ: MemType,
}

pub struct Global {
    pub typ: GlobalType,
    pub init: Expr,
}

pub struct Elem {
    pub typ: RefType,
    pub init: Vec<Expr>,
    pub mode: ElemMode,
}

pub enum ElemMode {
    Passive,
    Active { table: TableIdx, offset: Expr },
    Declarative,
}

pub enum ElemKind {
    Zero,
}

pub struct Data {
    pub init: Vec<Byte>,
    pub mode: DataMode,
}

pub enum DataMode {
    Passive,
    Active { memory: MemIdx, offset: Expr },
}

pub struct Start {
    pub func: FuncIdx,
}

pub struct Export {
    pub name: Name,
    pub desc: ExportDesc,
}

pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

pub struct Import {
    pub module: Name,
    pub name: Name,
    pub desc: ImportDesc,
}

pub enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
