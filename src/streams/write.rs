use std::ops::Deref;
use byteorder::WriteBytesExt;
use crate::streams::{Endianness, SeekWrite};

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
    stream: &mut S,
    bytes: I,
) -> Result<u64, std::io::Error>
{
    let mut written = 0;
    for byte in bytes.as_ref() {
        stream.write_u8(*byte)?;
        written += 1;
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
