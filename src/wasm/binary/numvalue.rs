use super::{WASMBinary, WriteContext};
use std::io;

// https://en.wikipedia.org/wiki/LEB128
mod leb128 {
    use std::io;

    const CONTINUATION_BIT: u8 = 1 << 7;

    fn low_bits_of_byte(byte: u8) -> u8 {
        byte & !CONTINUATION_BIT
    }

    fn low_bits_of_u64(val: u64) -> u8 {
        let byte = val & (std::u8::MAX as u64);
        low_bits_of_byte(byte as u8)
    }

    pub fn unsigned<W>(w: &mut W, mut val: u64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        loop {
            let mut byte = low_bits_of_u64(val);
            val >>= 7;
            if val != 0 {
                byte |= CONTINUATION_BIT;
            }

            let buf = [byte];
            w.write_all(&buf)?;

            if val == 0 {
                return Ok(());
            }
        }
    }

    pub fn signed<W>(w: &mut W, mut val: i64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        loop {
            let mut byte = val as u8;
            val >>= 6;
            let done = val == 0 || val == -1;
            if done {
                byte &= !CONTINUATION_BIT;
            } else {
                val >>= 1;
                byte |= CONTINUATION_BIT;
            }

            let buf = [byte];
            w.write_all(&buf)?;

            if done {
                return Ok(());
            }
        }
    }
}

impl WriteContext<u16> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &u16) -> io::Result<()> {
        leb128::unsigned(w, *input as u64)
    }
}

impl WriteContext<u32> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &u32) -> io::Result<()> {
        leb128::unsigned(w, *input as u64)
    }
}

impl WriteContext<u64> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &u64) -> io::Result<()> {
        leb128::unsigned(w, *input)
    }
}

impl WriteContext<usize> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &usize) -> io::Result<()> {
        leb128::unsigned(w, *input as u64)
    }
}

impl WriteContext<i32> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &i32) -> io::Result<()> {
        leb128::signed(w, *input as i64)
    }
}

impl WriteContext<i64> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &i64) -> io::Result<()> {
        leb128::signed(w, *input)
    }
}

impl WriteContext<f32> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &f32) -> io::Result<()> {
        w.write_all(&input.to_le_bytes())
    }
}

impl WriteContext<f64> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &f64) -> io::Result<()> {
        w.write_all(&input.to_le_bytes())
    }
}
