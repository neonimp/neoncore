//! Utilities for working with streams.
//! Like finding a signature in a stream, or reading a struct from a stream.

use crate::streams::{AnyInt, Endianness, SeekRead};
use byteorder::ReadBytesExt;
use std::io::{Error, ErrorKind, SeekFrom};

use super::LPWidth;

pub type StreamResult<T> = Result<T, Error>;

/// Finds a signature in a stream `S: Read + Seek` and returns it's position.
/// The stream is left at the position of the signature.
///
/// A skip parameter can be used to skip a number of bytes before searching for the signature,
/// this can speed up the search if the signature is known to be far away from
/// the start of the stream.
///
/// The limit parameter can be used to limit the search to a number of bytes, if not provided
/// the search will happen until the end of the stream.
///
/// The endianness parameter can be used to specify the endianness of the signature in the stream.
///
/// The rewind parameter can be used to rewind the stream to the position before the signature was found.
///
#[inline]
pub fn find_u32_signature<S: SeekRead>(
    stream: &mut S,
    sig: u32,
    skip: Option<u64>,
    limit: Option<u64>,
    endianness: Endianness,
    rewind: bool,
) -> StreamResult<u64> {
    let rewind_pos = stream.stream_position()?;
    let byte = &mut [0; 1];
    let sig_fbyte = match endianness {
        Endianness::LittleEndian => sig.to_le_bytes()[0],
        Endianness::BigEndian => sig.to_be_bytes()[0],
    };
    let skip = skip.unwrap_or(0);
    let limit = limit.unwrap_or(!0);

    stream.seek(SeekFrom::Start(skip))?;

    // Bytewise lookup
    let mut pos = skip;
    while pos < limit {
        let read = stream.read(byte)?;
        if read == 0 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Unexpected end of stream",
            ));
        }

        if byte[0] == sig_fbyte {
            // rewind 1 byte
            stream.seek(SeekFrom::Current(-1))?;
            // found first byte, check if the rest of the signature matches
            let sig_candidate = match endianness {
                Endianness::LittleEndian => stream.read_u32::<byteorder::LittleEndian>()?,
                Endianness::BigEndian => stream.read_u32::<byteorder::BigEndian>()?,
            };
            if sig_candidate == sig {
                break;
            }
            pos += 4;
            continue;
        }
        pos += 1;
    }

    if rewind {
        stream.seek(SeekFrom::Start(rewind_pos))?;
    }
    Ok(pos)
}

/// Finds a signature in a stream `S: Read + Seek` and returns it's position.
/// The stream is left at the position of the signature.
///
/// A skip parameter can be used to skip a number of bytes before searching for the signature,
/// this can speed up the search if the signature is known to be far away from
/// the start of the stream.
///
/// The limit parameter can be used to limit the search to a number of bytes, if not provided
/// the search will happen until the end of the stream.
///
/// The endianness parameter can be used to specify the endianness of the signature in the stream.
///
/// The rewind parameter can be used to rewind the stream to the position before the signature was found.
///
#[inline]
pub fn find_u64_signature<S: SeekRead>(
    stream: &mut S,
    sig: u64,
    skip: Option<u64>,
    limit: Option<u64>,
    endianness: Endianness,
    rewind: bool,
) -> StreamResult<u64> {
    let rewind_pos = stream.stream_position()?;
    let byte = &mut [0; 1];
    let sig_fbyte = match endianness {
        Endianness::LittleEndian => sig.to_le_bytes()[0],
        Endianness::BigEndian => sig.to_be_bytes()[0],
    };
    let skip = skip.unwrap_or(0);
    let limit = limit.unwrap_or(!0);

    stream.seek(SeekFrom::Start(skip))?;

    // Bytewise lookup
    let mut pos = skip;
    while pos < limit {
        let read = stream.read(byte)?;
        if read == 0 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Unexpected end of stream",
            ));
        }

        if byte[0] == sig_fbyte {
            // rewind 1 byte
            stream.seek(SeekFrom::Current(-1))?;
            // found first byte, check if the rest of the signature matches
            let sig_candidate = match endianness {
                Endianness::LittleEndian => stream.read_u64::<byteorder::LittleEndian>()?,
                Endianness::BigEndian => stream.read_u64::<byteorder::BigEndian>()?,
            };
            if sig_candidate == sig {
                break;
            }
            pos += 8;
            continue;
        }
        pos += 1;
    }

    if rewind {
        stream.seek(SeekFrom::Start(rewind_pos))?;
    }
    Ok(pos)
}

/// Scans `stream` for occurrences of `sig` and returns their positions.
/// The stream is left at the position of the last occurrence of `sig`.
pub fn find_all_u32_signatures<S: SeekRead>(
    stream: &mut S,
    sig: u32,
    endianness: Endianness,
) -> StreamResult<Vec<u64>> {
    let mut positions = Vec::new();
    loop {
        let pos = find_u32_signature(stream, sig, None, None, endianness, true)?;
        positions.push(pos);
    }
}

/// Scans `stream` for occurrences of `sig` and returns their positions.
/// The stream is left at the position of the last occurrence of `sig`.
pub fn find_all_u64_signatures<S: SeekRead>(
    stream: &mut S,
    sig: u64,
    endianness: Endianness,
) -> StreamResult<Vec<u64>> {
    let mut positions = Vec::new();
    loop {
        let pos = find_u64_signature(stream, sig, None, None, endianness, true)?;
        positions.push(pos);
    }
}

/// Reads `n` consecutive `u16` from `stream` while advancing the stream `n * 2` bytes.
///
/// # Arguments
///
/// * `stream`: The stream to read from.
/// * `n`: The number of `u16` to read.
/// * `endianess`: The endianness of the `u16`s in the stream.
///
/// returns: vector of the read `u16`s in order.
///
/// # Example
///
/// ```rust
/// use std::io::Cursor;
/// use neoncore::streams::read::{Endianness, read_n_u16};
///
/// let mut stream = Cursor::new(vec![0x01, 0x00, 0x02, 0x00, 0x03, 0x00]);
/// let v = read_n_u16(&mut stream, 3, Endianness::LittleEndian).unwrap();
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
pub fn read_n_u16<S: SeekRead>(
    mut stream: S,
    n: u64,
    endianess: Endianness,
) -> StreamResult<Vec<u16>> {
    let mut vl = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let v = match endianess {
            Endianness::LittleEndian => stream.read_u16::<byteorder::LittleEndian>()?,
            Endianness::BigEndian => stream.read_u16::<byteorder::BigEndian>()?,
        };
        vl.push(v);
    }
    Ok(vl)
}

/// Reads `n` consecutive `u32` from `stream` while advancing the stream `n * 4` bytes.
///
/// # Arguments
///
/// * `stream`: The stream to read from.
/// * `n`: The number of `u32` to read.
/// * `endianess`: The endianness of the `u32`s in the stream.
///
/// returns: vector of the read `u32`s in order.
///
/// # Example
///
/// ```rust
/// use std::io::Cursor;
/// use neoncore::streams::read::{Endianness, read_n_u32};
///
/// let mut stream = Cursor::new(vec![0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00]);
/// let v = read_n_u32(&mut stream, 3, Endianness::LittleEndian).unwrap();
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
pub fn read_n_u32<S: SeekRead>(
    mut stream: S,
    n: u64,
    endianess: Endianness,
) -> StreamResult<Vec<u32>> {
    let mut vl = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let v = match endianess {
            Endianness::LittleEndian => stream.read_u32::<byteorder::LittleEndian>()?,
            Endianness::BigEndian => stream.read_u32::<byteorder::BigEndian>()?,
        };
        vl.push(v);
    }
    Ok(vl)
}

/// Reads `n` consecutive `u64` from `stream` while advancing the stream `n * 8` bytes.
///
/// # Arguments
///
/// * `stream`: The stream to read from.
/// * `n`: The number of `u64` to read.
/// * `endianess`: The endianness of the `u64`s in the stream.
///
/// returns: vector of the read `u64`s in order.
///
/// # Example
///
/// ```rust
/// use std::io::Cursor;
/// use neoncore::streams::read::{Endianness, read_n_u64};
///
/// let mut stream = Cursor::new(b"\x01\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00");
/// let v = read_n_u64(&mut stream, 3, Endianness::LittleEndian).unwrap();
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
pub fn read_n_u64<S: SeekRead>(
    mut stream: S,
    n: u64,
    endianess: Endianness,
) -> StreamResult<Vec<u64>> {
    let mut vl = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let v = match endianess {
            Endianness::LittleEndian => stream.read_u64::<byteorder::LittleEndian>()?,
            Endianness::BigEndian => stream.read_u64::<byteorder::BigEndian>()?,
        };
        vl.push(v);
    }
    Ok(vl)
}

pub fn read_bytes<S: SeekRead>(mut stream: S, n: u64) -> StreamResult<Vec<u8>> {
    let mut vl = Vec::with_capacity(n as usize);
    let mut byte = [0; 1];
    for _ in 0..n {
        let read = stream.read(&mut byte)?;
        if read != 1 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Unexpected EOF while reading bytes",
            ));
        }
        vl.push(byte[0]);
    }
    Ok(vl)
}

/// How many input bytes are required at least to read the given format string.
///
/// # Arguments
/// * `format`: The format string.
///
/// # Format characters
/// | Char | Width | Meaning                |
/// |------|-------|------------------------|
/// | !    | -     | BigEndian              |
/// | @    | -     | Little endian          |
/// | x    | 1     | skips a single byte    |
/// | h    | 2     | Little endian          |
/// | H    | 2     | Big endian             |
/// | w    | 4     | Little endian          |
/// | W    | 4     | Big endian             |
/// | q    | 8     | Little endian          |
/// | Q    | 8     | Big endian             |
/// | P    | usize | Platform dependent     |
/// | s    | var   | String null terminated |
/// | S    | var   | String length prefixed |
/// 
/// # Returns
/// The number of bytes required to read the given format string with [`read_format`].
/// 
pub fn format_required_bytes(format: &str) -> u64 {
    let mut bytes = 0;
    let mut chars = format.chars();
    while let Some(c) = chars.next() {
        match c {
            // endianness
            '!' | '@' => {}
            // skip
            'x' => bytes += 1,
            'h' | 'H' => bytes += 2,
            'w' | 'W' => bytes += 4,
            'q' | 'Q' => bytes += 8,
            _ => panic!("Unknown format character: {}", c),
        }
    }
    bytes
}

/// Read the stream according to the given `format` and return the result.
///
/// # Arguments
/// * `stream`: The stream to read from.
/// * `format`: The format string.
///
/// # Format characters
/// See [`format_required_bytes`] for a list of format characters.
///
/// # Returns
/// a `Vec<AnyInt>` containing the read values.
pub fn read_format<S: SeekRead>(mut stream: S, format: &str) -> StreamResult<Vec<AnyInt>> {
    let mut values = Vec::new();
    let mut chars = format.chars();
    let endianess = match chars.next() {
        Some('!') => Endianness::BigEndian,
        Some('@') => Endianness::LittleEndian,
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Format string must start with either ! or @",
            ))
        }
    };

    while let Some(c) = chars.next() {
        if c == 'x' {
            stream.seek(SeekFrom::Current(1))?;
            continue;
        }
        let v = match endianess {
            Endianness::BigEndian => match c {
                'x' => AnyInt::U8(stream.read_u8()?),
                'h' => AnyInt::U16(stream.read_u16::<byteorder::BigEndian>()?),
                'w' => AnyInt::U32(stream.read_u32::<byteorder::BigEndian>()?),
                'q' => AnyInt::U64(stream.read_u64::<byteorder::BigEndian>()?),
                'H' => AnyInt::I16(stream.read_i16::<byteorder::BigEndian>()?),
                'W' => AnyInt::I32(stream.read_i32::<byteorder::BigEndian>()?),
                'Q' => AnyInt::I64(stream.read_i64::<byteorder::BigEndian>()?),
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("Unknown format character: {}", c),
                    ))
                }
            },
            Endianness::LittleEndian => match c {
                'x' => AnyInt::U8(stream.read_u8()?),
                'h' => AnyInt::U16(stream.read_u16::<byteorder::LittleEndian>()?),
                'w' => AnyInt::U32(stream.read_u32::<byteorder::LittleEndian>()?),
                'q' => AnyInt::U64(stream.read_u64::<byteorder::LittleEndian>()?),
                'H' => AnyInt::I16(stream.read_i16::<byteorder::LittleEndian>()?),
                'W' => AnyInt::I32(stream.read_i32::<byteorder::LittleEndian>()?),
                'Q' => AnyInt::I64(stream.read_i64::<byteorder::LittleEndian>()?),
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("Unknown format character: {}", c),
                    ))
                }
            },
        };
        values.push(v);
    }
    Ok(values)
}

/// Read a lenght prefixed buffer from the stream.
/// 
/// # Arguments
/// * `stream`: The stream to read from.
/// * `lptype`: The width of the lenght prefix.
/// * `lpend`: The endianness of the lenght prefix.
/// 
/// # Returns
/// The read buffer.
/// 
/// # Errors
/// This function will return an error in the following cases:
/// * The stream ends before `len` bytes are read.
/// * The stream returns an error.
pub fn read_lpbuf<S: SeekRead>(
    mut stream: S,
    lptype: LPWidth,
    lpend: Endianness,
) -> StreamResult<Vec<u8>> {
    let len = match lpend {
        Endianness::BigEndian => match lptype {
            LPWidth::LP8 => stream.read_u8()? as usize,
            LPWidth::LP16 => stream.read_u16::<byteorder::BigEndian>()? as usize,
            LPWidth::LP32 => stream.read_u32::<byteorder::BigEndian>()? as usize,
            LPWidth::LP64 => stream.read_u64::<byteorder::BigEndian>()? as usize,
        },
        Endianness::LittleEndian => match lptype {
            LPWidth::LP8 => stream.read_u8()? as usize,
            LPWidth::LP16 => stream.read_u16::<byteorder::LittleEndian>()? as usize,
            LPWidth::LP32 => stream.read_u32::<byteorder::LittleEndian>()? as usize,
            LPWidth::LP64 => stream.read_u64::<byteorder::LittleEndian>()? as usize,
        },
    };

    let mut buf = vec![0; len];
    stream.read_exact(&mut buf)?;

    Ok(buf)
}

/// Read a lenght prefixed string from the stream.
/// 
/// # Arguments
/// * `stream`: The stream to read from.
/// * `lptype`: The width of the lenght prefix.
/// * `lpend`: The endianness of the lenght prefix.
/// 
/// # Returns
/// The read string.
/// 
/// # Errors
/// This function will return an error in the following cases:
/// * The stream ends before `len` bytes are read.
/// * The read bytes are not valid UTF-8.
/// * The stream returns an error.
pub fn read_lpstr<S: SeekRead>(
    mut stream: S,
    lptype: LPWidth,
    lpend: Endianness,
) -> StreamResult<String> {
    let buf = read_lpbuf(&mut stream, lptype, lpend)?;

    match String::from_utf8(buf) {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
}

/// read a null terminated string from the stream of at most `maxlen` bytes.
/// 
/// # Arguments
/// * `stream`: The stream to read from.
/// * `maxlen`: The maximum length of the string.
/// 
/// # Returns
/// The read string.
pub fn read_cstr<S: SeekRead>(mut stream: S, maxlen: usize) -> StreamResult<String> {
    let mut buf = vec![0; maxlen];
    let mut i = 0;
    loop {
        if i >= maxlen {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "String is longer than maxlen",
            ));
        }
        let b = stream.read_u8()?;
        if b == 0 {
            break;
        }
        buf[i] = b;
        i += 1;
    }
    buf.truncate(i);
    match String::from_utf8(buf) {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streams::read::Endianness::LittleEndian;

    const DATA: [u8; 168] = [
        0x00, 0x2F, 0x6D, 0x61, 0x78, 0x5F, 0x73, 0x69, 0x7A, 0x65, 0x2E, 0x72, 0x73, 0x55, 0x54,
        0x05, 0x00, 0x01, 0xA9, 0xBA, 0xEE, 0x63, 0x50, 0x4B, 0x01, 0x02, 0x00, 0x00, 0x0A, 0x00,
        0x00, 0x00, 0x08, 0x00, 0xC8, 0x7A, 0x50, 0x56, 0xDB, 0x87, 0xEE, 0xBA, 0x1A, 0x02, 0x00,
        0x00, 0x8C, 0x09, 0x00, 0x00, 0x1D, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xF5, 0xEC, 0x00, 0x00, 0x70, 0x6F, 0x73, 0x74, 0x63, 0x61, 0x72,
        0x64, 0x2D, 0x6D, 0x61, 0x69, 0x6E, 0x2F, 0x74, 0x65, 0x73, 0x74, 0x73, 0x2F, 0x73, 0x63,
        0x68, 0x65, 0x6D, 0x61, 0x2E, 0x72, 0x73, 0x55, 0x54, 0x05, 0x00, 0x01, 0xA9, 0xBA, 0xEE,
        0x63, 0x50, 0x4B, 0x05, 0x06, 0x00, 0x00, 0x00, 0x00, 0x2C, 0x00, 0x2C, 0x00, 0x82, 0x0E,
        0x00, 0x00, 0x53, 0xEF, 0x00, 0x00, 0x28, 0x00, 0x61, 0x31, 0x63, 0x33, 0x61, 0x66, 0x34,
        0x37, 0x61, 0x65, 0x63, 0x34, 0x33, 0x33, 0x61, 0x34, 0x30, 0x30, 0x62, 0x39, 0x38, 0x37,
        0x31, 0x38, 0x64, 0x36, 0x37, 0x65, 0x32, 0x62, 0x38, 0x38, 0x33, 0x61, 0x36, 0x36, 0x38,
        0x64, 0x37, 0x37,
    ];

    #[test]
    fn test_find_signature() {
        let sig = 0x02014b50;
        let sig_2 = 0x06054b50;
        let mut stream = std::io::Cursor::new(DATA);

        let pos_1 = find_u32_signature(&mut stream, sig, None, None, LittleEndian, true).unwrap();
        let pos_2 =
            find_u32_signature(&mut stream, sig_2, Some(pos_1), None, LittleEndian, false).unwrap();

        assert_eq!(pos_1, 0x16);
        assert_eq!(pos_2, 0x6A);
    }

    #[test]
    fn test_find_signature64() {
        let sig = 0x4b5063eebaa90100;
        let mut stream = std::io::Cursor::new(DATA);

        let pos_1 = find_u64_signature(&mut stream, sig, None, None, LittleEndian, true).unwrap();

        assert_eq!(pos_1, 0x10);
    }

    #[test]
    fn test_read_n_u16() {
        let stream = std::io::Cursor::new(DATA);
        let v = read_n_u16(stream, 3, LittleEndian).unwrap();
        assert_eq!(v, vec![0x2F00, 0x616D, 0x5F78]);
    }

    #[test]
    fn test_read_n_u32() {
        let stream = std::io::Cursor::new(DATA);
        let v = read_n_u32(stream, 3, LittleEndian).unwrap();
        assert_eq!(v, vec![0x616D2F00, 0x69735F78, 0x722E657A]);
    }

    #[test]
    fn test_read_n_u64() {
        let stream = std::io::Cursor::new(DATA);
        let v = read_n_u64(stream, 3, LittleEndian).unwrap();
        assert_eq!(
            v,
            vec![0x69735f78616d2f00, 0x5545573722e657a, 0x4b5063eebaa90100]
        );
    }
}
