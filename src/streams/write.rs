use crate::streams::SeekWrite;
use byteorder::WriteBytesExt;

use super::{AnyInt, Endianness};

pub trait SizedBuffer: AsRef<[u8]> + AsMut<[u8]> + Default + Clone + Sized {}

impl<T: AsRef<[u8]> + AsMut<[u8]> + Default + Clone + Sized> SizedBuffer for T {}

/// Write a series of bytes to a stream
///
/// # Arguments
/// * `stream` - The stream to write to
/// * `bytes` - The bytes to write
///
/// # Returns
/// * `Ok(u64)` - The number of bytes written
/// * `Err(std::io::Error)` - The error encountered while writing
///
/// # Example
///
/// ```rust
/// use std::io::Cursor;
/// use neoncore::streams::write::write_bytes;
///
/// let mut cursor = Cursor::new(vec![]);
/// write_bytes(&mut cursor, b"Hello World").unwrap();
/// assert_eq!(cursor.into_inner(), b"Hello World");
/// ```
pub fn write_bytes<S: SeekWrite, I: SizedBuffer>(
    mut stream: S,
    bytes: I,
) -> Result<u64, std::io::Error> {
    let mut written = 0;
    for byte in bytes.as_ref() {
        stream.write_u8(*byte)?;
        written += 1;
    }
    Ok(written)
}

pub fn write_values<S: SeekWrite>(
    mut stream: S,
    values: &[AnyInt],
    endianess: Endianness,
) -> Result<u64, std::io::Error> {
    let mut written = 0;
    for v in values {
        match endianess {
            Endianness::LittleEndian => {
                let vb = v.to_bytes_le();
                written += write_bytes(&mut stream, vb)?;
            }
            Endianness::BigEndian => {
                let vb = v.to_bytes_be();
                written += write_bytes(&mut stream, vb)?;
            }
        }
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_write_bytes() {
        let mut cursor = Cursor::new(vec![]);
        write_bytes(&mut cursor, *b"Hello World").unwrap();
        assert_eq!(cursor.into_inner(), b"Hello World");
    }
}
