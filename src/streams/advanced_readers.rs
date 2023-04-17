use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::io::Read;
use std::marker::PhantomData;

use byteorder::ReadBytesExt;

use super::read::StreamResult;
use super::{AnyInt, StreamError};

/// Read a number of elements from a stream,
///
/// usage of PatternReader is to build a pattern with the provided methods
/// and then call call the read method with the stream to read from,
/// the read method will return a vector of the read elements consuming the pattern,
/// and leaving the stream at the end of the last read element.
#[derive(Debug)]
pub struct PatternReader<Ord: byteorder::ByteOrder> {
    pattern: Vec<PatternReaderTokens>,
    endianess: PhantomData<Ord>,
}

pub enum PatternReaderTokens {
    Padding(usize),
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    USize,
    Expr((u8, Box<dyn Fn(AnyInt) -> bool>)),
}

impl Debug for PatternReaderTokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternReaderTokens::Padding(len) => write!(f, "Padding({})", len),
            PatternReaderTokens::Bool => write!(f, "Bool"),
            PatternReaderTokens::U8 => write!(f, "U8"),
            PatternReaderTokens::U16 => write!(f, "U16"),
            PatternReaderTokens::U32 => write!(f, "U32"),
            PatternReaderTokens::U64 => write!(f, "U64"),
            PatternReaderTokens::I8 => write!(f, "I8"),
            PatternReaderTokens::I16 => write!(f, "I16"),
            PatternReaderTokens::I32 => write!(f, "I32"),
            PatternReaderTokens::I64 => write!(f, "I64"),
            PatternReaderTokens::USize => write!(f, "USize"),
            PatternReaderTokens::Expr((w, _)) => write!(f, "Expr(par_width: {})", w),
        }
    }
}

impl PatternReader<byteorder::BigEndian> {
    pub fn new_be() -> Self {
        Self::new()
    }
}

impl PatternReader<byteorder::LittleEndian> {
    pub fn new_le() -> Self {
        Self::new()
    }
}

impl<Ord: byteorder::ByteOrder> PatternReader<Ord> {
    pub fn new() -> Self {
        let pattern = Vec::new();
        Self {
            pattern,
            endianess: PhantomData::<Ord>::default(),
        }
    }

    pub fn add_u8(&mut self) -> &mut Self {
        self.pattern.push(PatternReaderTokens::U8);
        self
    }

    pub fn add_u16(&mut self) -> &mut Self {
        self.pattern.push(PatternReaderTokens::U16);
        self
    }

    pub fn add_u32(&mut self) -> &mut Self {
        self.pattern.push(PatternReaderTokens::U32);
        self
    }

    pub fn add_u64(&mut self) -> &mut Self {
        self.pattern.push(PatternReaderTokens::U64);
        self
    }

    pub fn add_i8(&mut self) -> &mut Self {
        self.pattern.push(PatternReaderTokens::I8);
        self
    }

    pub fn add_i16(&mut self) -> &mut Self {
        self.pattern.push(PatternReaderTokens::I16);
        self
    }

    pub fn add_i32(&mut self) -> &mut Self {
        self.pattern.push(PatternReaderTokens::I32);
        self
    }

    pub fn add_i64(&mut self) -> &mut Self {
        self.pattern.push(PatternReaderTokens::I64);
        self
    }

    pub fn add_usize(&mut self) -> &mut Self {
        self.pattern.push(PatternReaderTokens::USize);
        self
    }

    pub fn add_padding(&mut self, len: usize) -> &mut Self {
        self.pattern.push(PatternReaderTokens::Padding(len));
        self
    }

    pub fn add_bool(&mut self) -> &mut Self {
        self.pattern.push(PatternReaderTokens::Bool);
        self
    }

    pub fn add_expr(
        &mut self,
        par_width: u8,
        expr: impl Fn(AnyInt) -> bool + 'static,
    ) -> &mut Self {
        self.pattern
            .push(PatternReaderTokens::Expr((par_width, Box::new(expr))));
        self
    }

    /// How many input bytes are required at least to statisfy this pattern.
    ///
    /// # Returns
    /// The number of bytes required to read this pattern.
    pub fn pattern_required_bytes(&self) -> u64 {
        let mut bytes = 0;
        for tkn in self.pattern.iter() {
            match tkn {
                // skip
                PatternReaderTokens::Padding(sz) => bytes += sz,
                PatternReaderTokens::U8 | PatternReaderTokens::I8 | PatternReaderTokens::Bool => {
                    bytes += 1
                }
                PatternReaderTokens::U16 | PatternReaderTokens::I16 => bytes += 2,
                PatternReaderTokens::U32 | PatternReaderTokens::I32 => bytes += 4,
                PatternReaderTokens::U64 | PatternReaderTokens::I64 => bytes += 8,
                PatternReaderTokens::USize => {
                    bytes += std::mem::size_of::<usize>();
                }
                PatternReaderTokens::Expr(_) => bytes += 0,
            }
        }
        bytes as u64
    }

    /// Read the stream according to the given `format` and return the result.
    ///
    /// # Returns
    /// a ```Vec<AnyInt>``` containing the read values.
    pub fn read_pattern<S: Read>(&self, mut stream: S) -> StreamResult<Vec<AnyInt>> {
        let mut values = Vec::new();

        for tkn in self.pattern.iter() {
            if let PatternReaderTokens::Padding(size) = tkn {
                for _ in 0..*size {
                    stream.read_u8()?;
                }
                continue;
            }

            let v = match tkn {
                PatternReaderTokens::U8 => Some(AnyInt::U8(stream.read_u8()?)),
                PatternReaderTokens::I8 => Some(AnyInt::I8(stream.read_i8()?)),
                _ => None,
            };

            if let Some(v) = v {
                values.push(v);
                continue;
            }

            // the rest of the format characters require at least 2 bytes
            let v = match tkn {
                PatternReaderTokens::U16 => AnyInt::U16(stream.read_u16::<Ord>()?),
                PatternReaderTokens::U32 => AnyInt::U32(stream.read_u32::<Ord>()?),
                PatternReaderTokens::U64 => AnyInt::U64(stream.read_u64::<Ord>()?),
                PatternReaderTokens::I16 => AnyInt::I16(stream.read_i16::<Ord>()?),
                PatternReaderTokens::I32 => AnyInt::I32(stream.read_i32::<Ord>()?),
                PatternReaderTokens::I64 => AnyInt::I64(stream.read_i64::<Ord>()?),
                PatternReaderTokens::USize => {
                    if std::mem::size_of::<usize>() == 4 {
                        AnyInt::U32(stream.read_u32::<Ord>()?)
                    } else {
                        AnyInt::U64(stream.read_u64::<Ord>()?)
                    }
                }
                PatternReaderTokens::Bool => {
                    let v = stream.read_u8()?;
                    if v == 0 {
                        AnyInt::Bool(false)
                    } else {
                        AnyInt::Bool(true)
                    }
                }
                PatternReaderTokens::Expr((par_width, expr)) => {
                    let v = match par_width {
                        1 => AnyInt::U8(stream.read_u8()?),
                        2 => AnyInt::U16(stream.read_u16::<Ord>()?),
                        4 => AnyInt::U32(stream.read_u32::<Ord>()?),
                        8 => AnyInt::U64(stream.read_u64::<Ord>()?),
                        _ => {
                            return Err(StreamError::InvalidPattern(
                                "invalid parameter width".into(),
                            ))
                        }
                    };
                    if expr(v) {
                        AnyInt::Bool(true)
                    } else {
                        AnyInt::Bool(false)
                    }
                }
                PatternReaderTokens::Padding(_)
                | PatternReaderTokens::U8
                | PatternReaderTokens::I8 => unreachable!(),
            };
            values.push(v);
        }
        Ok(values)
    }
}

#[derive(Debug)]
pub struct StructReader<Ord: byteorder::ByteOrder> {
    fields: PatternReader<Ord>,
    field_names: Vec<String>,
    results: BTreeMap<String, AnyInt>,
}

impl StructReader<byteorder::BigEndian> {
    pub fn new_be() -> Self {
        Self::new()
    }
}

impl StructReader<byteorder::LittleEndian> {
    pub fn new_le() -> Self {
        Self::new()
    }
}

impl<Ord: byteorder::ByteOrder> StructReader<Ord> {
    pub fn new() -> Self {
        Self {
            fields: PatternReader::<Ord>::new(),
            field_names: Vec::new(),
            results: BTreeMap::new(),
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

    pub fn add_bool_field(mut self, name: &str) -> Self {
        self.fields.add_bool();
        self.field_names.push(name.to_string());
        self
    }

    pub fn add_expr_field(mut self, name: &str, par_width: u8, expr: fn(AnyInt) -> bool) -> Self {
        self.fields.add_expr(par_width, expr);
        self.field_names.push(name.to_string());
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

    /// reuturns the results as a BTreeMap
    /// and consumes the StructReader
    pub fn into_inner(self) -> BTreeMap<String, AnyInt> {
        self.results
    }

    pub fn get_inner_pattern(&self) -> &PatternReader<Ord> {
        &self.fields
    }

    pub fn results(&self) -> &BTreeMap<String, AnyInt> {
        &self.results
    }

    pub fn into_vec(self) -> Vec<(String, AnyInt)> {
        self.results.into_iter().collect()
    }
}

impl<Ord: byteorder::ByteOrder> std::ops::Index<&str> for StructReader<Ord> {
    type Output = AnyInt;
    /// Warning: panics if the field is not found
    fn index(&self, name: &str) -> &Self::Output {
        self.results.get(name).unwrap()
    }
}

impl<Ord: byteorder::ByteOrder> Default for StructReader<Ord> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Ord: byteorder::ByteOrder> Default for PatternReader<Ord> {
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
    fn test_pattern_req_bytes() {
        let v = PatternReader::new_le()
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
        let v = PatternReader::new_le()
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
        let v = StructReader::new_le()
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
            TryInto::<u64>::try_into(v["test2"]).unwrap(),
            0x5545573722e657a
        );

        assert_eq!(
            TryInto::<u64>::try_into(v.get("test3").unwrap()).unwrap(),
            0x4b5063eebaa90100
        );
    }
}
