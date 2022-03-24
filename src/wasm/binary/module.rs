use super::{WASMBinary, WriteContext};
use crate::wasm::instr::Instr;
use crate::wasm::module::*;
use crate::wasm::typ::RefType;
use std::io;

fn write_section<W>(w: &mut W, n: u8, input: Vec<u8>) -> io::Result<()>
where
    W: io::Write,
{
    let size = input.len();
    if size == 0 {
        Ok(())
    } else {
        WASMBinary::write(w, &n)?;
        WASMBinary::write(w, &size)?;
        WASMBinary::write(w, &input as &[u8])
    }
}

impl WriteContext<Module> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Module) -> io::Result<()> {
        WASMBinary::write(w, &[0x00u8, 0x61, 0x73, 0x6D] as &[u8])?; // magic
        WASMBinary::write(w, &[0x01u8, 0x00, 0x00, 0x00] as &[u8])?; // version
        write_section(w, 1, WASMBinary::as_bytes(&input.types)?)?;
        write_section(w, 2, WASMBinary::as_bytes(&input.imports)?)?;
        write_section(w, 3, WASMBinary::as_bytes(&function_section(input))?)?;
        write_section(w, 4, WASMBinary::as_bytes(&input.tables)?)?;
        write_section(w, 5, WASMBinary::as_bytes(&input.mems)?)?;
        write_section(w, 6, WASMBinary::as_bytes(&input.globals)?)?;
        write_section(w, 7, WASMBinary::as_bytes(&input.exports)?)?;
        write_section(w, 8, WASMBinary::as_bytes(&input.start)?)?;
        write_section(w, 9, WASMBinary::as_bytes(&input.elems)?)?;
        write_section(w, 12, WASMBinary::as_bytes(&data_count_section(input))?)?;
        write_section(w, 10, WASMBinary::as_bytes(&code_section(input)?)?)?;
        write_section(w, 11, WASMBinary::as_bytes(&input.datas)?)
    }
}

impl WriteContext<Import> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Import) -> io::Result<()> {
        WASMBinary::write(w, &input.module)?;
        WASMBinary::write(w, &input.name)?;
        WASMBinary::write(w, &input.desc)
    }
}

impl WriteContext<ImportDesc> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &ImportDesc) -> io::Result<()> {
        match input {
            ImportDesc::Func(x) => {
                WASMBinary::write(w, &0x00u8)?;
                WASMBinary::write(w, x)
            }
            ImportDesc::Table(tt) => {
                WASMBinary::write(w, &0x01u8)?;
                WASMBinary::write(w, tt)
            }
            ImportDesc::Mem(mt) => {
                WASMBinary::write(w, &0x02u8)?;
                WASMBinary::write(w, mt)
            }
            ImportDesc::Global(gt) => {
                WASMBinary::write(w, &0x03u8)?;
                WASMBinary::write(w, gt)
            }
        }
    }
}

fn function_section(module: &Module) -> Vec<TypeIdx> {
    let mut funcs = Vec::new();
    for f in &module.funcs {
        funcs.push(f.typ);
    }
    Vec::from(funcs)
}

impl WriteContext<Table> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Table) -> io::Result<()> {
        WASMBinary::write(w, &input.typ)
    }
}

impl WriteContext<Mem> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Mem) -> io::Result<()> {
        WASMBinary::write(w, &input.typ)
    }
}

impl WriteContext<Global> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Global) -> io::Result<()> {
        WASMBinary::write(w, &input.typ)?;
        WASMBinary::write(w, &input.init)
    }
}

impl WriteContext<Export> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Export) -> io::Result<()> {
        WASMBinary::write(w, &input.name)?;
        WASMBinary::write(w, &input.desc)
    }
}

impl WriteContext<ExportDesc> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &ExportDesc) -> io::Result<()> {
        match input {
            ExportDesc::Func(x) => {
                WASMBinary::write(w, &0x00u8)?;
                WASMBinary::write(w, x)
            }
            ExportDesc::Table(tt) => {
                WASMBinary::write(w, &0x01u8)?;
                WASMBinary::write(w, tt)
            }
            ExportDesc::Mem(mt) => {
                WASMBinary::write(w, &0x02u8)?;
                WASMBinary::write(w, mt)
            }
            ExportDesc::Global(gt) => {
                WASMBinary::write(w, &0x03u8)?;
                WASMBinary::write(w, gt)
            }
        }
    }
}

impl WriteContext<Start> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Start) -> io::Result<()> {
        WASMBinary::write(w, &input.func)
    }
}

impl WriteContext<Elem> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Elem) -> io::Result<()> {
        let typ = &input.typ;
        let init = &input.init;

        let mut all_reffunc = true;
        let mut funcidxs = Vec::new();
        for expr in init {
            if expr.instrs.len() != 1 {
                all_reffunc = false;
                break;
            }
            match expr.instrs[0] {
                Instr::RefFunc(fidx) => funcidxs.push(fidx),
                _ => {
                    all_reffunc = false;
                    break;
                }
            }
        }

        match &input.mode {
            ElemMode::Active { table, offset } => {
                let is_funcref = input.typ == RefType::FuncRef;
                if all_reffunc {
                    if table == &0 && is_funcref {
                        WASMBinary::write(w, &0x00u8)?;
                        WASMBinary::write(w, &offset)?;
                    } else {
                        WASMBinary::write(w, &0x02u8)?;
                        WASMBinary::write(w, &table)?;
                        WASMBinary::write(w, &offset)?;
                        WASMBinary::write(w, &ElemKind::Zero)?;
                    }
                    WASMBinary::write(w, &funcidxs)
                } else {
                    if table == &0 && is_funcref {
                        WASMBinary::write(w, &0x04u8)?;
                        WASMBinary::write(w, &offset)?;
                    } else {
                        WASMBinary::write(w, &0x06u8)?;
                        WASMBinary::write(w, &table)?;
                        WASMBinary::write(w, &offset)?;
                        WASMBinary::write(w, &typ)?;
                    }
                    WASMBinary::write(w, &init)
                }
            }
            ElemMode::Passive => {
                if all_reffunc {
                    WASMBinary::write(w, &0x01u8)?;
                    WASMBinary::write(w, &ElemKind::Zero)?;
                    WASMBinary::write(w, &funcidxs)
                } else {
                    WASMBinary::write(w, &0x05u8)?;
                    WASMBinary::write(w, &typ)?;
                    WASMBinary::write(w, &init)
                }
            }
            ElemMode::Declarative => {
                if all_reffunc {
                    WASMBinary::write(w, &0x03u8)?;
                    WASMBinary::write(w, &ElemKind::Zero)?;
                    WASMBinary::write(w, &funcidxs)
                } else {
                    WASMBinary::write(w, &0x07u8)?;
                    WASMBinary::write(w, &typ)?;
                    WASMBinary::write(w, &init)
                }
            }
        }
    }
}

impl WriteContext<ElemKind> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, _input: &ElemKind) -> io::Result<()> {
        WASMBinary::write(w, &0x00u8)
    }
}

fn code_section(module: &Module) -> io::Result<Vec<Vec<u8>>> {
    let mut codes = Vec::new();
    for f in &module.funcs {
        let mut codevec = Vec::new();

        if f.locals.len() > 0 {
            let mut prev_local = &f.locals[0];
            let mut count: u32 = 0;
            for local in &f.locals {
                if local == prev_local {
                    count += 1;
                } else {
                    WASMBinary::write(&mut codevec, &count)?;
                    WASMBinary::write(&mut codevec, &prev_local)?;
                    prev_local = local;
                    count = 1;
                }
            }
            WASMBinary::write(&mut codevec, &count)?;
            WASMBinary::write(&mut codevec, &prev_local)?;
        }

        WASMBinary::write(&mut codevec, &f.body)?;

        codes.push(codevec);
    }
    Ok(codes)
}

impl WriteContext<Data> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Data) -> io::Result<()> {
        match &input.mode {
            DataMode::Active { memory: 0, offset } => {
                WASMBinary::write(w, &0x00u8)?;
                WASMBinary::write(w, offset)?;
                WASMBinary::write(w, &input.init)
            }
            DataMode::Passive => {
                WASMBinary::write(w, &0x01u8)?;
                WASMBinary::write(w, &input.init)
            }
            DataMode::Active { memory, offset } => {
                WASMBinary::write(w, &0x02u8)?;
                WASMBinary::write(w, memory)?;
                WASMBinary::write(w, offset)?;
                WASMBinary::write(w, &input.init)
            }
        }
    }
}

fn data_count_section(module: &Module) -> u32 {
    module.datas.len() as u32
}
