use crate::streams::SeekWrite;
use byteorder::WriteBytesExt;

use super::{AnyInt, Endianness, LPWidth};

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

/// Write a lpbuff to a stream
///
/// # Arguments
/// * `stream` - The stream to write to
/// * `lptype` - The width of the length prefix
/// * `lpend` - The endianness of the length prefix
/// * `bytes` - The bytes to write
///
/// # Returns
/// * `Ok(u64)` - The number of bytes written
/// * `Err(std::io::Error)` - The error encountered while writing
pub fn write_lpbuf<S: SeekWrite>(
    mut stream: S,
    lptype: LPWidth,
    lpend: Endianness,
    bytes: &[u8],
) -> Result<u64, std::io::Error> {
    let mut written = 0;
    let len = bytes.len();
    if !LPWidth::usize_fits(lptype, len) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Length prefix does not fit in specified width",
        ));
    }
    match lptype {
        LPWidth::LP8 => {
            written += 1;
            stream.write_u8(len as u8)?;
        }
        LPWidth::LP16 => {
            written += 2;
            match lpend {
                Endianness::LittleEndian => {
                    stream.write_u16::<byteorder::LittleEndian>(len as u16)?;
                }
                Endianness::BigEndian => {
                    stream.write_u16::<byteorder::BigEndian>(len as u16)?;
                }
            }
        }
        LPWidth::LP32 => {
            written += 4;
            match lpend {
                Endianness::LittleEndian => {
                    stream.write_u32::<byteorder::LittleEndian>(len as u32)?;
                }
                Endianness::BigEndian => {
                    stream.write_u32::<byteorder::BigEndian>(len as u32)?;
                }
            }
        }
        LPWidth::LP64 => {
            written += 8;
            match lpend {
                Endianness::LittleEndian => {
                    stream.write_u64::<byteorder::LittleEndian>(len as u64)?;
                }
                Endianness::BigEndian => {
                    stream.write_u64::<byteorder::BigEndian>(len as u64)?;
                }
            }
        }
    }
    stream.write_all(bytes)?;
    written += len as u64;
    Ok(written)
}

/// Write a string to a stream as a lpbuff
/// 
/// # Arguments
/// * `stream` - The stream to write to
/// * `lptype` - The width of the length prefix
/// * `lpend` - The endianness of the length prefix
/// * `string` - The string to write
/// 
/// # Returns
/// * `Ok(u64)` - The number of bytes written
/// * `Err(std::io::Error)` - The error encountered while writing
/// 
/// # Note
/// This function is a wrapper around `write_lpbuf` that converts the string to bytes
pub fn write_lpstr<S: SeekWrite>(
    mut stream: S,
    lptype: LPWidth,
    lpend: Endianness,
    string: &str,
) -> Result<u64, std::io::Error> {
    Ok(write_lpbuf(&mut stream, lptype, lpend, string.as_bytes())?)
}


/// Write a string to a stream as a null-terminated string
/// 
/// # Arguments
/// * `stream` - The stream to write to
/// * `string` - The string to write
/// 
/// # Returns
/// * `Ok(u64)` - The number of bytes written
/// * `Err(std::io::Error)` - The error encountered while writing
pub fn write_cstr<S: SeekWrite>(mut stream: S, string: &str) -> Result<u64, std::io::Error> {
    stream.write_all(string.as_bytes())?;
    stream.write_u8(0)?;
    Ok(string.len() as u64 + 1)
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
