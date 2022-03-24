use super::value::*;

// Type Specification

// https://webassembly.github.io/spec/core/syntax/types.html

#[derive(PartialEq)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(PartialEq)]
pub enum VecType {
    V128,
}

#[derive(PartialEq)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

#[derive(PartialEq)]
pub enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

pub struct ResultType {
    pub values: Vec<ValType>,
}

pub struct FuncType {
    pub from: ResultType,
    pub to: ResultType,
}

pub struct Limits {
    pub min: U32,
    pub max: Option<U32>,
}

pub struct MemType {
    pub lim: Limits,
}

pub struct TableType {
    pub lim: Limits,
    pub et: RefType,
}

pub struct GlobalType {
    pub mutability: Mut,
    pub typ: ValType,
}

pub enum Mut {
    Const,
    Var,
}

pub enum ExternType {
    Func(FuncType),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
