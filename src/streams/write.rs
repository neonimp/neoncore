//! Utilities for writing to streams of binary data

use std::io::Write;

use crate::streams::SeekWrite;
use byteorder::WriteBytesExt;

use super::{AnyInt, Endianness, LPWidth, MapType};

/// Write a list of `AnyInt`s to a stream
pub fn write_values<S: Write>(
    mut stream: S,
    values: &[AnyInt],
    endianness: Endianness,
) -> Result<u64, std::io::Error> {
    let mut written = 0;
    for v in values {
        match endianness {
            Endianness::LittleEndian => {
                let vb = v.to_bytes_le();
                written += stream.write(vb.as_ref())?;
            }
            Endianness::BigEndian => {
                let vb = v.to_bytes_be();
                written += stream.write(vb.as_ref())?;
            }
        }
    }
    Ok(written as u64)
}

/// Write a lpbuf to a stream
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
pub fn write_lpbuf<S: Write>(
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

/// Write a string to a stream as a lpbuf
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
pub fn write_lpstr<S: Write>(
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

/// Write a map type to a stream
pub fn write_map<'a>(
    mut stream: impl Write,
    endianness: Endianness,
    map: &'a impl MapType<'a, String, AnyInt>,
    lpwidth: LPWidth,
) -> Result<u64, std::io::Error> {
    let mut written = 0;
    let entries = AnyInt::U48(map.len() as u64);
    written += write_values(&mut stream, &[entries], endianness)?;
    for (k, v) in map.iter() {
        match endianness {
            Endianness::LittleEndian => {
                written += write_lpstr(&mut stream, lpwidth, endianness, k)?;
                let vb = v.to_bytes_le();
                written += stream.write(vb.as_ref())? as u64;
            }
            Endianness::BigEndian => {
                written += write_lpstr(&mut stream, lpwidth, endianness, k)?;
                let vb = v.to_bytes_be();
                written += stream.write(vb.as_ref())? as u64;
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
    fn test_write_values() {
        let mut buf = [0u8; 8];
        let mut stream = Cursor::new(&mut buf[..]);
        let values = [AnyInt::U32(0x12345678u32), AnyInt::U32(0x9abcdef0u32)];
        write_values(&mut stream, &values, Endianness::LittleEndian).unwrap();
        assert_eq!(buf, [0x78, 0x56, 0x34, 0x12, 0xf0, 0xde, 0xbc, 0x9a]);
    }

    #[test]
    fn test_write_lpbuf() {
        let mut buf = [0u8; 8];
        let mut stream = Cursor::new(&mut buf[..]);
        let bytes = [0x12, 0x34, 0x56, 0x78];
        write_lpbuf(&mut stream, LPWidth::LP32, Endianness::LittleEndian, &bytes).unwrap();
        assert_eq!(buf, [0x04, 0x00, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_write_lpstr() {
        let mut buf = [0u8; 8];
        let mut stream = Cursor::new(&mut buf[..]);
        let string = "test";
        write_lpstr(&mut stream, LPWidth::LP32, Endianness::LittleEndian, string).unwrap();
        assert_eq!(buf, [0x04, 0x00, 0x00, 0x00, 0x74, 0x65, 0x73, 0x74]);
    }

    #[test]
    fn test_write_cstr() {
        let mut buf = [0u8; 8];
        let mut stream = Cursor::new(&mut buf[..]);
        let string = "test";
        write_cstr(&mut stream, string).unwrap();
        assert_eq!(buf, [0x74, 0x65, 0x73, 0x74, 0x00, 0x00, 0x00, 0x00]);
    }
}
