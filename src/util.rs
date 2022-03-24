const CONTINUATION_BIT: u8 = 1 << 7;
const SIGN_BIT: u8 = 1 << 6;

fn low_bits_of_byte(byte: u8) -> u8 {
    byte & !CONTINUATION_BIT
}

fn low_bits_of_u64(val: u64) -> u8 {
    let byte = val & (std::u8::MAX as u64);
    low_bits_of_byte(byte as u8)
}

use std::io;

/// Write `val` to the `std::io::Write` stream `w` as an unsigned LEB128 value.
///
/// On success, return the number of bytes written to `w`.
pub fn unsigned_leb128<W>(w: &mut W, mut val: u64) -> Result<usize, io::Error>
where
    W: ?Sized + io::Write,
{
    let mut bytes_written = 0;
    loop {
        let mut byte = low_bits_of_u64(val);
        val >>= 7;
        if val != 0 {
            // More bytes to come, so set the continuation bit.
            byte |= CONTINUATION_BIT;
        }

        let buf = [byte];
        w.write_all(&buf)?;
        bytes_written += 1;

        if val == 0 {
            return Ok(bytes_written);
        }
    }
}

/// Write `val` to the `std::io::Write` stream `w` as a signed LEB128 value.
///
/// On success, return the number of bytes written to `w`.
pub fn signed_leb128<W>(w: &mut W, mut val: i64) -> Result<usize, io::Error>
where
    W: ?Sized + io::Write,
{
    let mut bytes_written = 0;
    loop {
        let mut byte = val as u8;
        // Keep the sign bit for testing
        val >>= 6;
        let done = val == 0 || val == -1;
        if done {
            byte &= !CONTINUATION_BIT;
        } else {
            // Remove the sign bit
            val >>= 1;
            // More bytes to come, so set the continuation bit.
            byte |= CONTINUATION_BIT;
        }

        let buf = [byte];
        w.write_all(&buf)?;
        bytes_written += 1;

        if done {
            return Ok(bytes_written);
        }
    }
}
