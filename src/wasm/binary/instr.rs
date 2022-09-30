use super::{WASMBinary, WriteContext};
use crate::wasm::instr::*;
use crate::wasm::value::*;
use std::io;

macro_rules! write_bin {
    ($w:ident << $firstthing:tt) => {
        WASMBinary::write($w, &$firstthing)
    };

    ($w:ident << $firstthing:tt $(<< $thing:tt)+) => {{
        write_bin!($w << $firstthing)?;
        write_bin!($w $(<< $thing)+)
    }};
}

impl WriteContext<Instr> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, instr: &Instr) -> io::Result<()> {
        match instr {
            // Control Instructions ------------------------------
            Instr::Unreachable => write_bin!(w << 0x00u8),
            Instr::Nop => write_bin!(w << 0x01u8),
            Instr::Block(bt, instrs) => {
                write_bin!(w << 0x02u8 << bt)?;
                for instr in instrs {
                    write_bin!(w << instr)?;
                }
                write_bin!(w << 0x0Bu8)
            }
            Instr::Loop(bt, instrs) => {
                write_bin!(w << 0x03u8 << bt)?;
                for instr in instrs {
                    write_bin!(w << instr)?;
                }
                write_bin!(w << 0x0Bu8)
            }
            Instr::If(bt, if_instrs, else_instrs) => {
                write_bin!(w << 0x04u8 << bt)?;
                for instr in if_instrs {
                    write_bin!(w << instr)?;
                }
                if else_instrs.len() > 0 {
                    write_bin!(w << 0x0Bu8)?;
                    for instr in else_instrs {
                        write_bin!(w << instr)?;
                    }
                }
                write_bin!(w << 0x0Bu8)
            }
            Instr::Br(l) => write_bin!(w << 0x0Cu8 << l),
            Instr::BrIf(l) => write_bin!(w << 0x0Du8 << l),
            Instr::BrTable(v, l) => write_bin!(w << 0x0Eu8 << v << l),
            Instr::Return => write_bin!(w << 0x0Fu8),
            Instr::Call(x) => write_bin!(w << 0x10u8 << x),
            Instr::CallIndirect(y, x) => write_bin!(w << 0x11u8 << y << x),

            // Reference Instrunctions ---------------------------
            Instr::RefNull(t) => write_bin!(w << 0xD0u8 << t),
            Instr::RefIsNull => write_bin!(w << 0xD1u8),
            Instr::RefFunc(x) => write_bin!(w << 0xD2u8 << x),

            // Parametric Instructions ---------------------------
            Instr::Drop => write_bin!(w << 0x1Au8),
            Instr::Select(ts) => match ts {
                None => write_bin!(w << 0x1Bu8),
                Some(ts) => write_bin!(w << 0x1Cu8 << ts),
            },

            // Variable Instructions -----------------------------
            Instr::LocalGet(x) => write_bin!(w << 0x20u8 << x),
            Instr::LocalSet(x) => write_bin!(w << 0x21u8 << x),
            Instr::LocalTee(x) => write_bin!(w << 0x22u8 << x),
            Instr::GlobalGet(x) => write_bin!(w << 0x23u8 << x),
            Instr::GlobalSet(x) => write_bin!(w << 0x24u8 << x),

            // Table Instructions --------------------------------
            Instr::TableGet(x) => write_bin!(w << 0x25u8 << x),
            Instr::TableSet(x) => write_bin!(w << 0x26u8 << x),
            Instr::TableInit(x, y) => write_bin!(w << 0xFCu8 << 12u32 << y << x),
            Instr::ElemDrop(x) => write_bin!(w << 0xFCu8 << 12u32 << x),
            Instr::TableCopy(x, y) => write_bin!(w << 0xFCu8 << 14u32 << y << x),
            Instr::TableGrow(x) => write_bin!(w << 0xFCu8 << 15u32 << x),
            Instr::TableSize(x) => write_bin!(w << 0xFCu8 << 16u32 << x),
            Instr::TableFill(x) => write_bin!(w << 0xFCu8 << 17u32 << x),

            // Memory Instructions -------------------------------
            Instr::I32Load(m) => write_bin!(w << 0x28u8 << m),
            Instr::I64Load(m) => write_bin!(w << 0x29u8 << m),
            Instr::F32Load(m) => write_bin!(w << 0x2Au8 << m),
            Instr::F64Load(m) => write_bin!(w << 0x2Bu8 << m),
            Instr::I32Load8S(m) => write_bin!(w << 0x2Cu8 << m),
            Instr::I32Load8U(m) => write_bin!(w << 0x2Du8 << m),
            Instr::I32Load16S(m) => write_bin!(w << 0x2Eu8 << m),
            Instr::I32Load16U(m) => write_bin!(w << 0x2Fu8 << m),
            Instr::I64Load8S(m) => write_bin!(w << 0x30u8 << m),
            Instr::I64Load8U(m) => write_bin!(w << 0x31u8 << m),
            Instr::I64Load16S(m) => write_bin!(w << 0x32u8 << m),
            Instr::I64Load16U(m) => write_bin!(w << 0x33u8 << m),
            Instr::I64Load32S(m) => write_bin!(w << 0x34u8 << m),
            Instr::I64Load32U(m) => write_bin!(w << 0x35u8 << m),
            Instr::I32Store(m) => write_bin!(w << 0x36u8 << m),
            Instr::I64Store(m) => write_bin!(w << 0x37u8 << m),
            Instr::F32Store(m) => write_bin!(w << 0x38u8 << m),
            Instr::F64Store(m) => write_bin!(w << 0x39u8 << m),
            Instr::I32Store8(m) => write_bin!(w << 0x3Au8 << m),
            Instr::I32Store16(m) => write_bin!(w << 0x3Bu8 << m),
            Instr::I64Store8(m) => write_bin!(w << 0x3Cu8 << m),
            Instr::I64Store16(m) => write_bin!(w << 0x3Du8 << m),
            Instr::I64Store32(m) => write_bin!(w << 0x3Eu8 << m),
            Instr::MemorySize => write_bin!(w << 0x3Fu8 << 0x00u8),
            Instr::MemoryGrow => write_bin!(w << 0x40u8 << 0x00u8),
            Instr::MemoryInit(x) => write_bin!(w << 0xFCu8 << 8u32 << x << 0x00u8),
            Instr::DataDrop(x) => write_bin!(w << 0xFCu8 << 9u32 << x),
            Instr::MemoryCopy => write_bin!(w << 0xFCu8 << 10u32 << 0x00u8 << 0x00u8),
            Instr::MemoryFill => write_bin!(w << 0xFCu8 << 11u32 << 0x00u8),

            // Numeric Instructions ------------------------------
            Instr::I32Const(n) => write_bin!(w << 0x41u8 << n),
            Instr::I64Const(n) => write_bin!(w << 0x42u8 << n),
            Instr::F32Const(z) => write_bin!(w << 0x43u8 << z),
            Instr::F64Const(z) => write_bin!(w << 0x44u8 << z),

            Instr::I32Eqz => write_bin!(w << 0x45u8),
            Instr::I32Eq => write_bin!(w << 0x46u8),
            Instr::I32Ne => write_bin!(w << 0x47u8),
            Instr::I32LtS => write_bin!(w << 0x48u8),
            Instr::I32LtU => write_bin!(w << 0x49u8),
            Instr::I32GtS => write_bin!(w << 0x4Au8),
            Instr::I32GtU => write_bin!(w << 0x4Bu8),
            Instr::I32LeS => write_bin!(w << 0x4Cu8),
            Instr::I32LeU => write_bin!(w << 0x4Du8),
            Instr::I32GeS => write_bin!(w << 0x4Eu8),
            Instr::I32GeU => write_bin!(w << 0x4Fu8),

            Instr::I64Eqz => write_bin!(w << 0x50u8),
            Instr::I64Eq => write_bin!(w << 0x51u8),
            Instr::I64Ne => write_bin!(w << 0x52u8),
            Instr::I64LtS => write_bin!(w << 0x53u8),
            Instr::I64LtU => write_bin!(w << 0x54u8),
            Instr::I64GtS => write_bin!(w << 0x55u8),
            Instr::I64GtU => write_bin!(w << 0x56u8),
            Instr::I64LeS => write_bin!(w << 0x57u8),
            Instr::I64LeU => write_bin!(w << 0x58u8),
            Instr::I64GeS => write_bin!(w << 0x59u8),
            Instr::I64GeU => write_bin!(w << 0x5Au8),

            Instr::F32Eq => write_bin!(w << 0x5Bu8),
            Instr::F32Ne => write_bin!(w << 0x5Cu8),
            Instr::F32Lt => write_bin!(w << 0x5Du8),
            Instr::F32Gt => write_bin!(w << 0x5Eu8),
            Instr::F32Le => write_bin!(w << 0x5Fu8),
            Instr::F32Ge => write_bin!(w << 0x60u8),

            Instr::F64Eq => write_bin!(w << 0x61u8),
            Instr::F64Ne => write_bin!(w << 0x62u8),
            Instr::F64Lt => write_bin!(w << 0x63u8),
            Instr::F64Gt => write_bin!(w << 0x64u8),
            Instr::F64Le => write_bin!(w << 0x65u8),
            Instr::F64Ge => write_bin!(w << 0x66u8),

            Instr::I32Clz => write_bin!(w << 0x67u8),
            Instr::I32Ctz => write_bin!(w << 0x68u8),
            Instr::I32Popcnt => write_bin!(w << 0x69u8),
            Instr::I32Add => write_bin!(w << 0x6Au8),
            Instr::I32Sub => write_bin!(w << 0x6Bu8),
            Instr::I32Mul => write_bin!(w << 0x6Cu8),
            Instr::I32DivS => write_bin!(w << 0x6Du8),
            Instr::I32DivU => write_bin!(w << 0x6Eu8),
            Instr::I32RemS => write_bin!(w << 0x6Fu8),
            Instr::I32RemU => write_bin!(w << 0x70u8),
            Instr::I32And => write_bin!(w << 0x71u8),
            Instr::I32Or => write_bin!(w << 0x72u8),
            Instr::I32Xor => write_bin!(w << 0x73u8),
            Instr::I32Shl => write_bin!(w << 0x74u8),
            Instr::I32ShrS => write_bin!(w << 0x75u8),
            Instr::I32ShrU => write_bin!(w << 0x76u8),
            Instr::I32Rotl => write_bin!(w << 0x77u8),
            Instr::I32Rotr => write_bin!(w << 0x78u8),

            Instr::I64Clz => write_bin!(w << 0x79u8),
            Instr::I64Ctz => write_bin!(w << 0x7Au8),
            Instr::I64Popcnt => write_bin!(w << 0x7Bu8),
            Instr::I64Add => write_bin!(w << 0x7Cu8),
            Instr::I64Sub => write_bin!(w << 0x7Du8),
            Instr::I64Mul => write_bin!(w << 0x7Eu8),
            Instr::I64DivS => write_bin!(w << 0x7Fu8),
            Instr::I64DivU => write_bin!(w << 0x80u8),
            Instr::I64RemS => write_bin!(w << 0x81u8),
            Instr::I64RemU => write_bin!(w << 0x82u8),
            Instr::I64And => write_bin!(w << 0x83u8),
            Instr::I64Or => write_bin!(w << 0x84u8),
            Instr::I64Xor => write_bin!(w << 0x85u8),
            Instr::I64Shl => write_bin!(w << 0x86u8),
            Instr::I64ShrS => write_bin!(w << 0x87u8),
            Instr::I64ShrU => write_bin!(w << 0x88u8),
            Instr::I64Rotl => write_bin!(w << 0x89u8),
            Instr::I64Rotr => write_bin!(w << 0x8Au8),

            Instr::F32Abs => write_bin!(w << 0x8Bu8),
            Instr::F32Neg => write_bin!(w << 0x8Cu8),
            Instr::F32Ceil => write_bin!(w << 0x8Du8),
            Instr::F32Floor => write_bin!(w << 0x8Eu8),
            Instr::F32Trunc => write_bin!(w << 0x8Fu8),
            Instr::F32Nearest => write_bin!(w << 0x9Au8),
            Instr::F32Sqrt => write_bin!(w << 0x91u8),
            Instr::F32Add => write_bin!(w << 0x92u8),
            Instr::F32Sub => write_bin!(w << 0x93u8),
            Instr::F32Mul => write_bin!(w << 0x94u8),
            Instr::F32Div => write_bin!(w << 0x95u8),
            Instr::F32Min => write_bin!(w << 0x96u8),
            Instr::F32Max => write_bin!(w << 0x97u8),
            Instr::F32Copysign => write_bin!(w << 0x98u8),

            Instr::F64Abs => write_bin!(w << 0x99u8),
            Instr::F64Neg => write_bin!(w << 0x9Au8),
            Instr::F64Ceil => write_bin!(w << 0x9Bu8),
            Instr::F64Floor => write_bin!(w << 0x9Cu8),
            Instr::F64Trunc => write_bin!(w << 0x9Du8),
            Instr::F64Nearest => write_bin!(w << 0x9Eu8),
            Instr::F64Sqrt => write_bin!(w << 0x9Fu8),
            Instr::F64Add => write_bin!(w << 0xA0u8),
            Instr::F64Sub => write_bin!(w << 0xA1u8),
            Instr::F64Mul => write_bin!(w << 0xA2u8),
            Instr::F64Div => write_bin!(w << 0xA3u8),
            Instr::F64Min => write_bin!(w << 0xA4u8),
            Instr::F64Max => write_bin!(w << 0xA5u8),
            Instr::F64Copysign => write_bin!(w << 0xA6u8),

            Instr::I32WrapI64 => write_bin!(w << 0xA7u8),
            Instr::I32TruncF32S => write_bin!(w << 0xA8u8),
            Instr::I32TruncF32U => write_bin!(w << 0xA9u8),
            Instr::I32TruncF64S => write_bin!(w << 0xAAu8),
            Instr::I32TruncF64U => write_bin!(w << 0xABu8),
            Instr::I64ExtendI32S => write_bin!(w << 0xACu8),
            Instr::I64ExtendI32U => write_bin!(w << 0xADu8),
            Instr::I64TruncF32S => write_bin!(w << 0xAEu8),
            Instr::I64TruncF32U => write_bin!(w << 0xAFu8),
            Instr::I64TruncF64S => write_bin!(w << 0xB0u8),
            Instr::I64TruncF64U => write_bin!(w << 0xB1u8),
            Instr::F32ConvertI32S => write_bin!(w << 0xB2u8),
            Instr::F32ConvertI32U => write_bin!(w << 0xB3u8),
            Instr::F32ConvertI64S => write_bin!(w << 0xB4u8),
            Instr::F32ConvertI64U => write_bin!(w << 0xB5u8),
            Instr::F32DemoteF64 => write_bin!(w << 0xB6u8),
            Instr::F64ConvertI32S => write_bin!(w << 0xB7u8),
            Instr::F64ConvertI32U => write_bin!(w << 0xB8u8),
            Instr::F64ConvertI64S => write_bin!(w << 0xB9u8),
            Instr::F64ConvertI64U => write_bin!(w << 0xBAu8),
            Instr::F64PromoteF32 => write_bin!(w << 0xBBu8),
            Instr::I32ReinterpretF32 => write_bin!(w << 0xBCu8),
            Instr::I64ReinterpretF64 => write_bin!(w << 0xBDu8),
            Instr::F32ReinterpretI32 => write_bin!(w << 0xBEu8),
            Instr::F64ReinterpretI64 => write_bin!(w << 0xBFu8),

            Instr::I32Extend8S => write_bin!(w << 0xC0u8),
            Instr::I32Extend16S => write_bin!(w << 0xC1u8),
            Instr::I64Extend8S => write_bin!(w << 0xC2u8),
            Instr::I64Extend16S => write_bin!(w << 0xC3u8),
            Instr::I64Extend32S => write_bin!(w << 0xC4u8),

            Instr::I32TruncSatF32S => write_bin!(w << 0xFCu8 << 0u32),
            Instr::I32TruncSatF32U => write_bin!(w << 0xFCu8 << 1u32),
            Instr::I32TruncSatF64S => write_bin!(w << 0xFCu8 << 2u32),
            Instr::I32TruncSatF64U => write_bin!(w << 0xFCu8 << 3u32),
            Instr::I64TruncSatF32S => write_bin!(w << 0xFCu8 << 4u32),
            Instr::I64TruncSatF32U => write_bin!(w << 0xFCu8 << 5u32),
            Instr::I64TruncSatF64S => write_bin!(w << 0xFCu8 << 6u32),
            Instr::I64TruncSatF64U => write_bin!(w << 0xFCu8 << 7u32),
        }
    }
}

impl WriteContext<MemArg> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &MemArg) -> io::Result<()> {
        WASMBinary::write(w, &input.align)?;
        WASMBinary::write(w, &input.offset)
    }
}

impl WriteContext<BlockType> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &BlockType) -> io::Result<()> {
        match input {
            BlockType::None => WASMBinary::write(w, &0x40u8),
            BlockType::ValType(t) => WASMBinary::write(w, t),
            BlockType::TypeIdx(x) => WASMBinary::write(w, &(*x as S64)),
        }
    }
}

impl WriteContext<Expr> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Expr) -> io::Result<()> {
        for instr in &input.instrs {
            WASMBinary::write(w, &instr)?;
        }
        WASMBinary::write(w, &0x0Bu8)
    }
}
