//! This module has utilities for reading and writing to streams
//! of binary data see [`mod@read`] and [`mod@write`] for more information.

use byteorder::WriteBytesExt;
use std::io::{Read, Seek, Write};

pub mod write;
pub mod read;

pub trait SeekRead: Read + Seek {}
pub trait SeekWrite: Write + Seek {}
pub trait SeekReadWrite: Read + Write + Seek {}

impl<T: Read + Seek> SeekRead for T {}
impl<T: Write + Seek> SeekWrite for T {}
impl<T: Read + Write + Seek> SeekReadWrite for T {}

/// The endianness of a stream
#[derive(Clone, Copy)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

/// The width of a length prefix for lp family functions
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LPWidth {
    LP8,
    LP16,
    LP32,
    LP64,
}

impl LPWidth {
    pub fn size(&self) -> usize {
        match self {
            LPWidth::LP8 => 1,
            LPWidth::LP16 => 2,
            LPWidth::LP32 => 4,
            LPWidth::LP64 => 8,
        }
    }

    pub fn usize_fits(lptype: LPWidth, len: usize) -> bool {
        match lptype {
            LPWidth::LP8 => len <= u8::MAX as usize,
            LPWidth::LP16 => len <= u16::MAX as usize,
            LPWidth::LP32 => len <= u32::MAX as usize,
            LPWidth::LP64 => len <= u64::MAX as usize,
        }
    }
}

/// A type that can hold any integer type
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnyInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
}

impl AnyInt {
    pub fn to_bytes_le(&self) -> Vec<u8> {
        let buf = Vec::with_capacity(self.size());
        let mut writer = std::io::Cursor::new(buf);

        match self {
            AnyInt::U8(v) => writer.write_u8(*v),
            AnyInt::U16(v) => writer.write_u16::<byteorder::LittleEndian>(*v),
            AnyInt::U32(v) => writer.write_u32::<byteorder::LittleEndian>(*v),
            AnyInt::U64(v) => writer.write_u64::<byteorder::LittleEndian>(*v),
            AnyInt::U128(v) => writer.write_u128::<byteorder::LittleEndian>(*v),
            AnyInt::I8(v) => writer.write_i8(*v),
            AnyInt::I16(v) => writer.write_i16::<byteorder::LittleEndian>(*v),
            AnyInt::I32(v) => writer.write_i32::<byteorder::LittleEndian>(*v),
            AnyInt::I64(v) => writer.write_i64::<byteorder::LittleEndian>(*v),
            AnyInt::I128(v) => writer.write_i128::<byteorder::LittleEndian>(*v),
        }
        .unwrap();

        writer.into_inner()
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        let buf = Vec::with_capacity(self.size());
        let mut writer = std::io::Cursor::new(buf);

        match self {
            AnyInt::U8(v) => writer.write_u8(*v),
            AnyInt::U16(v) => writer.write_u16::<byteorder::BigEndian>(*v),
            AnyInt::U32(v) => writer.write_u32::<byteorder::BigEndian>(*v),
            AnyInt::U64(v) => writer.write_u64::<byteorder::BigEndian>(*v),
            AnyInt::U128(v) => writer.write_u128::<byteorder::BigEndian>(*v),
            AnyInt::I8(v) => writer.write_i8(*v),
            AnyInt::I16(v) => writer.write_i16::<byteorder::BigEndian>(*v),
            AnyInt::I32(v) => writer.write_i32::<byteorder::BigEndian>(*v),
            AnyInt::I64(v) => writer.write_i64::<byteorder::BigEndian>(*v),
            AnyInt::I128(v) => writer.write_i128::<byteorder::BigEndian>(*v),
        }
        .unwrap();

        writer.into_inner()
    }

    fn size(&self) -> usize {
        match self {
            AnyInt::U8(_) => 1,
            AnyInt::U16(_) => 2,
            AnyInt::U32(_) => 4,
            AnyInt::U64(_) => 8,
            AnyInt::U128(_) => 16,
            AnyInt::I8(_) => 1,
            AnyInt::I16(_) => 2,
            AnyInt::I32(_) => 4,
            AnyInt::I64(_) => 8,
            AnyInt::I128(_) => 16,
        }
    }
}

impl From<u8> for AnyInt {
    fn from(v: u8) -> Self {
        AnyInt::U8(v)
    }
}

impl From<u16> for AnyInt {
    fn from(v: u16) -> Self {
        AnyInt::U16(v)
    }
}

impl From<u32> for AnyInt {
    fn from(v: u32) -> Self {
        AnyInt::U32(v)
    }
}

impl From<u64> for AnyInt {
    fn from(v: u64) -> Self {
        AnyInt::U64(v)
    }
}

impl From<u128> for AnyInt {
    fn from(v: u128) -> Self {
        AnyInt::U128(v)
    }
}

impl From<i8> for AnyInt {
    fn from(v: i8) -> Self {
        AnyInt::I8(v)
    }
}

impl From<i16> for AnyInt {
    fn from(v: i16) -> Self {
        AnyInt::I16(v)
    }
}

impl From<i32> for AnyInt {
    fn from(v: i32) -> Self {
        AnyInt::I32(v)
    }
}

impl From<i64> for AnyInt {
    fn from(v: i64) -> Self {
        AnyInt::I64(v)
    }
}

impl From<i128> for AnyInt {
    fn from(v: i128) -> Self {
        AnyInt::I128(v)
    }
}

impl TryFrom<AnyInt> for u8 {
    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::U8(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to u8", v),
            ))
        }
    }

    type Error = std::io::Error;
}

impl TryFrom<AnyInt> for u16 {
    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::U16(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to u16", v),
            ))
        }
    }

    type Error = std::io::Error;
}

impl TryFrom<AnyInt> for u32 {
    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::U32(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to u32", v),
            ))
        }
    }

    type Error = std::io::Error;
}

impl TryFrom<AnyInt> for u64 {
    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::U64(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to u64", v),
            ))
        }
    }

    type Error = std::io::Error;
}

impl TryFrom<AnyInt> for u128 {
    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::U128(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to u128", v),
            ))
        }
    }

    type Error = std::io::Error;
}

impl TryFrom<AnyInt> for i8 {
    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::I8(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to i8", v),
            ))
        }
    }

    type Error = std::io::Error;
}

impl TryFrom<AnyInt> for i16 {
    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::I16(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to i16", v),
            ))
        }
    }

    type Error = std::io::Error;
}

impl TryFrom<AnyInt> for i32 {
    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::I32(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to i32", v),
            ))
        }
    }

    type Error = std::io::Error;
}

impl TryFrom<AnyInt> for i64 {
    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::I64(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to i64", v),
            ))
        }
    }

    type Error = std::io::Error;
}

impl TryFrom<AnyInt> for i128 {
    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::I128(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to i128", v),
            ))
        }
    }

    type Error = std::io::Error;
}
