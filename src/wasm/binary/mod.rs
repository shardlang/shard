mod instr;
mod module;
mod numvalue;
mod typ;

use std::io;

pub enum WASMBinary {}

pub trait WriteContext<T: ?Sized> {
    fn write<W: io::Write>(w: &mut W, input: &T) -> io::Result<()>;

    #[allow(unused_must_use)]
    fn as_bytes(input: &T) -> io::Result<Vec<u8>> {
        let mut outvec = Vec::new();
        Self::write(&mut outvec, input)?;
        Ok(outvec)
    }
}

impl WriteContext<u8> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &u8) -> io::Result<()> {
        w.write_all(&[*input])
    }
}

impl WriteContext<[u8]> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &[u8]) -> io::Result<()> {
        w.write_all(input)
    }
}

impl<T, WASMBinary: WriteContext<T> + WriteContext<usize>> WriteContext<Vec<T>> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, inputs: &Vec<T>) -> io::Result<()> {
        let len = inputs.len();
        if len > u32::MAX as usize {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }
        WASMBinary::write(w, &len)?;
        for input in inputs {
            WASMBinary::write(w, input)?;
        }
        Ok(())
    }
}

impl<T, WASMBinary: WriteContext<T>> WriteContext<Option<T>> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &Option<T>) -> io::Result<()> {
        match input {
            Some(x) => WASMBinary::write(w, x),
            None => Ok(()),
        }
    }
}

impl WriteContext<String> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &String) -> io::Result<()> {
        WASMBinary::write(w, &Vec::from(input.as_bytes()))
    }
}

impl WriteContext<str> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &str) -> io::Result<()> {
        WASMBinary::write(w, &Vec::from(input.as_bytes()))
    }
}

impl<T, WASMBinary: WriteContext<T>> WriteContext<&T> for WASMBinary {
    fn write<W: io::Write>(w: &mut W, input: &&T) -> io::Result<()> {
        WASMBinary::write(w, *input)
    }
}
