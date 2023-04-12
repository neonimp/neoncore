//! Utilities for working with streams.
//! Like finding a signature in a stream, or reading a struct from a stream.

use crate::streams::helpers::read_lpend;
use crate::streams::{AnyInt, Endianness, MapType, SeekRead, StreamError};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read, SeekFrom};
use std::marker::PhantomData;

use super::LPWidth;

pub type StreamResult<T> = Result<T, StreamError>;

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
            return Err(StreamError::from(Error::new(
                ErrorKind::UnexpectedEof,
                "Unexpected end of stream",
            )));
        }

        if byte[0] == sig_fbyte {
            // rewind 1 byte
            stream.seek(SeekFrom::Current(-1))?;
            // found first byte, check if the rest of the signature matches
            let sig_candidate = match endianness {
                Endianness::LittleEndian => stream.read_u32::<LittleEndian>()?,
                Endianness::BigEndian => stream.read_u32::<BigEndian>()?,
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
            return Err(StreamError::from(Error::new(
                ErrorKind::UnexpectedEof,
                "Unexpected end of stream",
            )));
        }

        if byte[0] == sig_fbyte {
            // rewind 1 byte
            stream.seek(SeekFrom::Current(-1))?;
            // found first byte, check if the rest of the signature matches
            let sig_candidate = match endianness {
                Endianness::LittleEndian => stream.read_u64::<LittleEndian>()?,
                Endianness::BigEndian => stream.read_u64::<BigEndian>()?,
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

/// Read a length prefixed buffer from the stream.
///
/// # Arguments
/// * `stream`: The stream to read from.
/// * `lptype`: The width of the length prefix.
/// * `lpend`: The endianness of the length prefix.
///
/// # Returns
/// The read buffer.
///
/// # Errors
/// This function will return an error in the following cases:
/// * The stream ends before `len` bytes are read.
/// * The stream returns an error.
#[inline]
pub fn read_lpbuf<S: Read>(
    mut stream: S,
    lptype: LPWidth,
    lpend: Endianness,
) -> StreamResult<Vec<u8>> {
    let len = read_lpend(&mut stream, lptype, lpend)?;

    let mut buf = vec![0; len];
    stream.read_exact(&mut buf)?;

    Ok(buf)
}

/// Read a length prefixed string from the stream.
///
/// # Arguments
/// * `stream`: The stream to read from.
/// * `lptype`: The width of the length prefix.
/// * `lpend`: The endianness of the length prefix.
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

    String::from_utf8(buf).map_err(|e| StreamError::from(Error::new(ErrorKind::InvalidData, e)))
}

/// read a null terminated string from the stream of at most `maxlen` bytes.
///
/// # Arguments
/// * `stream`: The stream to read from.
/// * `maxlen`: The maximum length of the string.
///
/// # Returns
/// The read string.
pub fn read_cstr<S: Read>(mut stream: S, maxlen: usize) -> StreamResult<String> {
    let mut buf = vec![0; maxlen];
    let mut i = 0;
    loop {
        if i >= maxlen {
            return Err(StreamError::from(Error::new(
                ErrorKind::InvalidData,
                "String is longer than maxlen",
            )));
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
        Err(e) => Err(StreamError::from(Error::new(ErrorKind::InvalidData, e))),
    }
}

/// Read a length prefixed map from the stream.
/// # Arguments
/// * `stream`: The stream to read from.
/// * `endianness`: The endianness of the length prefix.
/// * `lpwidth`: The width of the length prefix.
///
/// # Returns
/// The read map.
pub fn read_map<S: Read, M: MapType<'static, String, AnyInt>>(
    mut stream: S,
    endianness: Endianness,
    lpwidth: LPWidth,
) -> StreamResult<M> {
    let mut map = M::new();
    let len = read_lpend(&mut stream, lpwidth, endianness)?;

    for _ in 0..len {
        let key = read_cstr(&mut stream, 256)?;
        let value = match endianness {
            Endianness::LittleEndian => AnyInt::from(stream.read_u64::<LittleEndian>()?),
            Endianness::BigEndian => AnyInt::from(stream.read_u64::<BigEndian>()?),
        };
        map.insert(key, value);
    }
    Ok(map)
}

pub struct ReadPattern<Ord: byteorder::ByteOrder> {
    pattern: Vec<ReadPatternTokens>,
    endianess: PhantomData<Ord>,
}

pub enum ReadPatternTokens {
    Padding(usize),
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    USize,
}

impl ReadPattern<byteorder::BigEndian> {
    pub fn new_be() -> Self {
        Self::new()
    }
}

impl ReadPattern<byteorder::LittleEndian> {
    pub fn new_le() -> Self {
        Self::new()
    }
}

impl<Ord: byteorder::ByteOrder> ReadPattern<Ord> {
    pub fn new() -> Self {
        let pattern = Vec::new();
        Self {
            pattern,
            endianess: PhantomData::<Ord>::default(),
        }
    }

    pub fn add_u8(&mut self) -> &mut Self {
        self.pattern.push(ReadPatternTokens::U8);
        self
    }

    pub fn add_u16(&mut self) -> &mut Self {
        self.pattern.push(ReadPatternTokens::U16);
        self
    }

    pub fn add_u32(&mut self) -> &mut Self {
        self.pattern.push(ReadPatternTokens::U32);
        self
    }

    pub fn add_u64(&mut self) -> &mut Self {
        self.pattern.push(ReadPatternTokens::U64);
        self
    }

    pub fn add_i8(&mut self) -> &mut Self {
        self.pattern.push(ReadPatternTokens::I8);
        self
    }

    pub fn add_i16(&mut self) -> &mut Self {
        self.pattern.push(ReadPatternTokens::I16);
        self
    }

    pub fn add_i32(&mut self) -> &mut Self {
        self.pattern.push(ReadPatternTokens::I32);
        self
    }

    pub fn add_i64(&mut self) -> &mut Self {
        self.pattern.push(ReadPatternTokens::I64);
        self
    }

    pub fn add_usize(&mut self) -> &mut Self {
        self.pattern.push(ReadPatternTokens::USize);
        self
    }

    pub fn add_padding(&mut self, len: usize) -> &mut Self {
        self.pattern.push(ReadPatternTokens::Padding(len));
        self
    }

    pub fn add_pad_byte(&mut self) -> &mut Self {
        self.pattern.push(ReadPatternTokens::Padding(1));
        self
    }

    /// How many input bytes are required at least to statisfy this pattern.
    ///
    /// # Returns
    /// The number of bytes required to read the given format string using [`read_pattern`].
    pub fn pattern_required_bytes(&self) -> u64 {
        let mut bytes = 0;
        for tkn in self.pattern.iter() {
            match tkn {
                // skip
                ReadPatternTokens::Padding(sz) => bytes += sz,
                ReadPatternTokens::U8 | ReadPatternTokens::I8 => bytes += 1,
                ReadPatternTokens::U16 | ReadPatternTokens::I16 => bytes += 2,
                ReadPatternTokens::U32 | ReadPatternTokens::I32 => bytes += 4,
                ReadPatternTokens::U64 | ReadPatternTokens::I64 => bytes += 8,
                ReadPatternTokens::USize => {
                    bytes += std::mem::size_of::<usize>();
                }
            }
        }
        bytes as u64
    }

    /// Read the stream according to the given `format` and return the result.
    ///
    /// # Arguments
    /// * `stream`: The stream to read from.
    /// * `format`: The format string.
    ///
    /// # Format characters
    /// See [`pattern_required_bytes`] for a list of format characters.
    ///
    /// # Returns
    /// a ```Vec<AnyInt>``` containing the read values.
    pub fn read_pattern<S: Read>(&self, mut stream: S) -> StreamResult<Vec<AnyInt>> {
        let mut values = Vec::new();

        for tkn in self.pattern.iter() {
            if let ReadPatternTokens::Padding(size) = tkn {
                for _ in 0..*size {
                    stream.read_u8()?;
                }
                continue;
            }

            let v = match tkn {
                ReadPatternTokens::U8 => Some(AnyInt::U8(stream.read_u8()?)),
                ReadPatternTokens::I8 => Some(AnyInt::I8(stream.read_i8()?)),
                _ => None,
            };

            if let Some(v) = v {
                values.push(v);
                continue;
            }

            // the rest of the format characters require at least 2 bytes
            let v = match tkn {
                ReadPatternTokens::U16 => AnyInt::U16(stream.read_u16::<Ord>()?),
                ReadPatternTokens::U32 => AnyInt::U32(stream.read_u32::<Ord>()?),
                ReadPatternTokens::U64 => AnyInt::U64(stream.read_u64::<Ord>()?),
                ReadPatternTokens::I16 => AnyInt::I16(stream.read_i16::<Ord>()?),
                ReadPatternTokens::I32 => AnyInt::I32(stream.read_i32::<Ord>()?),
                ReadPatternTokens::I64 => AnyInt::I64(stream.read_i64::<Ord>()?),
                ReadPatternTokens::USize => {
                    if std::mem::size_of::<usize>() == 4 {
                        AnyInt::U32(stream.read_u32::<Ord>()?)
                    } else {
                        AnyInt::U64(stream.read_u64::<Ord>()?)
                    }
                }
                _ => unreachable!(),
            };
            values.push(v);
        }
        Ok(values)
    }
}

pub struct ReadStruct<Ord: byteorder::ByteOrder> {
    fields: ReadPattern<Ord>,
    field_names: Vec<String>,
    results: HashMap<String, AnyInt>,
}

impl ReadStruct<byteorder::BigEndian> {
    pub fn new_be() -> Self {
        Self::new()
    }
}

impl ReadStruct<byteorder::LittleEndian> {
    pub fn new_le() -> Self {
        Self::new()
    }
}

impl<Ord: byteorder::ByteOrder> ReadStruct<Ord> {
    pub fn new() -> Self {
        Self {
            fields: ReadPattern::<Ord>::new(),
            field_names: Vec::new(),
            results: HashMap::new(),
        }
    }

    pub fn add_u8_field(mut self, name: &str) -> Self {
        self.fields.add_u8();
        self.field_names.push(name.to_string());
        self
    }

    pub fn add_u16_field(mut self, name: &str) -> Self {
        self.fields.add_u16();
        self.field_names.push(name.to_string());
        self
    }

    pub fn add_u32_field(mut self, name: &str) -> Self {
        self.fields.add_u32();
        self.field_names.push(name.to_string());
        self
    }

    pub fn add_u64_field(mut self, name: &str) -> Self {
        self.fields.add_u64();
        self.field_names.push(name.to_string());
        self
    }

    pub fn add_usize_field(mut self, name: &str) -> Self {
        self.fields.add_usize();
        self.field_names.push(name.to_string());
        self
    }

    pub fn add_i8_field(mut self, name: &str) -> Self {
        self.fields.add_i8();
        self.field_names.push(name.to_string());
        self
    }

    pub fn add_i16_field(mut self, name: &str) -> Self {
        self.fields.add_i16();
        self.field_names.push(name.to_string());
        self
    }

    pub fn add_i32_field(mut self, name: &str) -> Self {
        self.fields.add_i32();
        self.field_names.push(name.to_string());
        self
    }

    pub fn add_i64_field(mut self, name: &str) -> Self {
        self.fields.add_i64();
        self.field_names.push(name.to_string());
        self
    }

    pub fn add_padding(mut self, size: usize) -> Self {
        self.fields.add_padding(size);
        self
    }

    pub fn required_bytes(&self) -> u64 {
        self.fields.pattern_required_bytes()
    }

    pub fn read<S: Read>(mut self, mut stream: S) -> StreamResult<Self> {
        let values = self.fields.read_pattern(&mut stream)?;
        for (name, value) in self.field_names.iter().zip(values.iter()) {
            self.results.insert(name.clone(), *value);
        }
        Ok(self)
    }

    pub fn get(&self, name: &str) -> Option<AnyInt> {
        self.results.get(name).cloned()
    }
}

impl<Ord: byteorder::ByteOrder> Default for ReadStruct<Ord> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Ord: byteorder::ByteOrder> Default for ReadPattern<Ord> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streams::AnyInt;

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

        let pos_1 =
            find_u32_signature(&mut stream, sig, None, None, Endianness::LittleEndian, true)
                .unwrap();
        let pos_2 = find_u32_signature(
            &mut stream,
            sig_2,
            Some(pos_1),
            None,
            Endianness::LittleEndian,
            false,
        )
        .unwrap();

        assert_eq!(pos_1, 0x16);
        assert_eq!(pos_2, 0x6A);
    }

    #[test]
    fn test_find_signature64() {
        let sig = 0x4b5063eebaa90100;
        let mut stream = std::io::Cursor::new(DATA);

        let pos_1 =
            find_u64_signature(&mut stream, sig, None, None, Endianness::LittleEndian, true)
                .unwrap();

        assert_eq!(pos_1, 0x10);
    }

    #[test]
    fn test_pattern_req_bytes() {
        let v = ReadPattern::new_le()
            .add_padding(2)
            .add_u64()
            .add_u64()
            .add_i16()
            .add_padding(6)
            .pattern_required_bytes();
        assert_eq!(v, 26);
    }

    #[test]
    fn test_read_pattern() {
        let stream = std::io::Cursor::new(DATA);
        let v = ReadPattern::new_le()
            .add_u64()
            .add_u64()
            .add_u64()
            .read_pattern(stream)
            .unwrap();
        assert_eq!(
            v,
            vec![
                AnyInt::U64(0x69735f78616d2f00),
                AnyInt::U64(0x5545573722e657a),
                AnyInt::U64(0x4b5063eebaa90100)
            ]
        );
    }

    #[test]
    fn test_read_struct() {
        let stream = std::io::Cursor::new(DATA);
        let v = ReadStruct::new_le()
            .add_u64_field("test1")
            .add_u64_field("test2")
            .add_u64_field("test3")
            .read(stream)
            .unwrap();

        assert_eq!(
            TryInto::<u64>::try_into(v.get("test1").unwrap()).unwrap(),
            0x69735f78616d2f00
        );

        assert_eq!(
            TryInto::<u64>::try_into(v.get("test2").unwrap()).unwrap(),
            0x5545573722e657a
        );

        assert_eq!(
            TryInto::<u64>::try_into(v.get("test3").unwrap()).unwrap(),
            0x4b5063eebaa90100
        );
    }
}
