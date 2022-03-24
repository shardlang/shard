use super::{WASMBinary, WriteContext};
use crate::wasm::typ::*;
use std::io;

impl WriteContext<NumType> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &NumType) -> io::Result<()> {
        let byte: u8 = match input {
            NumType::I32 => 0x7F,
            NumType::I64 => 0x7E,
            NumType::F32 => 0x7D,
            NumType::F64 => 0x7C,
        };
        WASMBinary::write(w, &byte)
    }
}

impl WriteContext<VecType> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &VecType) -> io::Result<()> {
        let byte: u8 = match input {
            VecType::V128 => 0x7B,
        };
        WASMBinary::write(w, &byte)
    }
}

impl WriteContext<RefType> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &RefType) -> io::Result<()> {
        let byte: u8 = match input {
            RefType::FuncRef => 0x70,
            RefType::ExternRef => 0x6F,
        };
        WASMBinary::write(w, &byte)
    }
}

impl WriteContext<ValType> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &ValType) -> io::Result<()> {
        match input {
            ValType::NumType(t) => WASMBinary::write(w, t),
            ValType::VecType(t) => WASMBinary::write(w, t),
            ValType::RefType(t) => WASMBinary::write(w, t),
        }
    }
}

impl WriteContext<ResultType> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &ResultType) -> io::Result<()> {
        WASMBinary::write(w, &input.values)
    }
}

impl WriteContext<FuncType> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &FuncType) -> io::Result<()> {
        WASMBinary::write(w, &0x60u8)?;
        WASMBinary::write(w, &input.from)?;
        WASMBinary::write(w, &input.to)
    }
}

impl WriteContext<Limits> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Limits) -> io::Result<()> {
        match input.max {
            None => {
                WASMBinary::write(w, &0x00u8)?;
                WASMBinary::write(w, &input.min)
            }
            Some(m) => {
                WASMBinary::write(w, &0x01u8)?;
                WASMBinary::write(w, &input.min)?;
                WASMBinary::write(w, &m)
            }
        }
    }
}

impl WriteContext<MemType> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &MemType) -> io::Result<()> {
        WASMBinary::write(w, &input.lim)
    }
}

impl WriteContext<TableType> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &TableType) -> io::Result<()> {
        WASMBinary::write(w, &input.et)?;
        WASMBinary::write(w, &input.lim)
    }
}

impl WriteContext<GlobalType> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &GlobalType) -> io::Result<()> {
        WASMBinary::write(w, &input.typ)?;
        WASMBinary::write(w, &input.mutability)
    }
}

impl WriteContext<Mut> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Mut) -> io::Result<()> {
        let byte: u8 = match input {
            Mut::Const => 0x00,
            Mut::Var => 0x01,
        };
        WASMBinary::write(w, &byte)
    }
}
