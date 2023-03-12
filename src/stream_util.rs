//! Utilities for working with streams.
//! Like finding a signature in a stream, or reading a struct from a stream.

use byteorder::ReadBytesExt;
use std::io::{Read, Seek, SeekFrom};
use pest::Parser;

#[derive(Clone, Copy)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

#[derive(Parser)]
#[grammar = "stream_query.pest"]
pub struct StreamQueryParser;

pub type StreamResult<T> = Result<T, std::io::Error>;

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
pub fn find_u32_signature<S: Read + Seek>(
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
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
pub fn find_u64_signature<S: Read + Seek>(
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream_util::Endianness::LittleEndian;

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
}
