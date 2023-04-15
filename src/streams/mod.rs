//! This module has utilities for reading and writing to streams
//! of binary data see [`mod@read`] and [`mod@write`] for more information.

use byteorder::WriteBytesExt;
use std::io::{Cursor, Read, Seek, Write};
use thiserror::Error;

pub mod advanced_readers;
mod helpers;
pub mod read;
pub mod write;

pub trait SeekRead: Read + Seek {}
pub trait SeekWrite: Write + Seek {}
pub trait SeekReadWrite: Read + Write + Seek {}

pub trait LPType<T, R: ?Sized> {
    fn lpwidth(&self) -> &LPWidth;
    fn lpendian(&self) -> &Endianness;
    fn set_endian(&mut self, endianness: Endianness);
    fn lp(&self) -> usize;
    fn val(&self) -> &R;
}

impl<T: Read + Seek> SeekRead for T {}
impl<T: Write + Seek> SeekWrite for T {}
impl<T: Read + Write + Seek> SeekReadWrite for T {}

#[derive(Debug, Error)]
pub enum StreamError {
    #[error("Stream error: {0}")]
    StreamError(String),
    #[error("Invalid character on pattern: {0} at position {1}")]
    InvalidChar(char, usize),
    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),
    #[error("Stream error: {0}")]
    IOError(#[from] std::io::Error),
}

/// The endianness of a stream
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

/// The width of a length prefix for lp family functions
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
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

/// Length prefixed String
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct LPString {
    lpwidth: LPWidth,
    lpendian: Endianness,
    lp: usize,
    val: String,
}

impl LPType<String, str> for LPString {
    fn lpwidth(&self) -> &LPWidth {
        &self.lpwidth
    }

    fn lpendian(&self) -> &Endianness {
        &self.lpendian
    }

    fn set_endian(&mut self, endianness: Endianness) {
        self.lpendian = endianness;
    }

    fn lp(&self) -> usize {
        self.lp
    }

    fn val(&self) -> &str {
        &self.val
    }
}

impl From<String> for LPString {
    fn from(s: String) -> Self {
        LPString {
            lpwidth: LPWidth::LP32,
            lpendian: Endianness::LittleEndian,
            lp: s.len(),
            val: s,
        }
    }
}

impl From<&str> for LPString {
    fn from(s: &str) -> Self {
        LPString {
            lpwidth: LPWidth::LP32,
            lpendian: Endianness::LittleEndian,
            lp: s.len(),
            val: s.to_string(),
        }
    }
}

impl From<LPString> for String {
    fn from(s: LPString) -> Self {
        s.val
    }
}

/// Length prefixed &str
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct LPStr<'data> {
    lpwidth: LPWidth,
    lpendian: Endianness,
    lp: usize,
    val: &'data str,
}

impl<'data> LPType<&str, str> for LPStr<'data> {
    fn lpwidth(&self) -> &LPWidth {
        &self.lpwidth
    }

    fn lpendian(&self) -> &Endianness {
        &self.lpendian
    }

    fn set_endian(&mut self, endianness: Endianness) {
        self.lpendian = endianness;
    }

    fn lp(&self) -> usize {
        self.lp
    }

    fn val(&self) -> &'data str {
        self.val
    }
}

impl<'data> From<&'data str> for LPStr<'data> {
    fn from(s: &'data str) -> Self {
        LPStr {
            lpwidth: LPWidth::LP32,
            lpendian: Endianness::LittleEndian,
            lp: s.len(),
            val: s,
        }
    }
}

impl<'data> From<LPStr<'data>> for &'data str {
    fn from(s: LPStr<'data>) -> Self {
        s.val
    }
}

/// Length prefixed buffer
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct LPBuffer<'data> {
    lpwidth: LPWidth,
    lpendian: Endianness,
    lp: usize,
    val: &'data [u8],
}

impl<'data> LPType<&[u8], [u8]> for LPBuffer<'data> {
    fn lpwidth(&self) -> &LPWidth {
        &self.lpwidth
    }

    fn lpendian(&self) -> &Endianness {
        &self.lpendian
    }

    fn set_endian(&mut self, endianness: Endianness) {
        self.lpendian = endianness;
    }

    fn lp(&self) -> usize {
        self.lp
    }

    fn val(&self) -> &[u8] {
        self.val
    }
}

impl<'data> From<&'data [u8]> for LPBuffer<'data> {
    fn from(s: &'data [u8]) -> Self {
        LPBuffer {
            lpwidth: LPWidth::LP32,
            lpendian: Endianness::LittleEndian,
            lp: s.len(),
            val: s,
        }
    }
}

impl<'data> From<LPBuffer<'data>> for &'data [u8] {
    fn from(s: LPBuffer<'data>) -> Self {
        s.val
    }
}

// TODO: Constraint on K: Serialize, V: Serialize
/// Trait representing any map type that can be written to a stream
pub trait MapType<'a, K, V>: 'a
where
    K: 'a,
    V: 'a,
{
    type Iter: Iterator<Item = (&'a K, &'a V)>;
    fn new() -> Self;
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn keys(&self) -> Vec<&K>;
    fn values(&self) -> Vec<&V>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn iter(&'a self) -> Self::Iter;
}

impl<'a, K: 'a, V: 'a> MapType<'a, K, V> for std::collections::HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    type Iter = std::collections::hash_map::Iter<'a, K, V>;

    fn new() -> Self {
        Self::new()
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn keys(&self) -> Vec<&K> {
        self.keys().collect()
    }

    fn values(&self) -> Vec<&V> {
        self.values().collect()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, K: 'a, V: 'a> MapType<'a, K, V> for std::collections::BTreeMap<K, V>
where
    K: Ord,
{
    type Iter = std::collections::btree_map::Iter<'a, K, V>;

    fn new() -> Self {
        Self::new()
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn keys(&self) -> Vec<&K> {
        self.keys().collect()
    }

    fn values(&self) -> Vec<&V> {
        self.values().collect()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

/// A type that can hold any integer type.
///
/// it implements [`TryFrom`] and [`Into`] for all integer types
/// it also has the u48 and i48 types which are represented as u64 and i64
/// in memory but are serialized as 6 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnyInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U48(u64),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I48(i64),
    I64(i64),
    I128(i128),
    Bool(bool),
}

impl AnyInt {
    pub fn to_bytes_le(&self) -> Vec<u8> {
        let buf = Vec::with_capacity(self.size());
        let mut writer = Cursor::new(buf);

        match self {
            AnyInt::U8(v) => writer.write_u8(*v),
            AnyInt::U16(v) => writer.write_u16::<byteorder::LittleEndian>(*v),
            AnyInt::U32(v) => writer.write_u32::<byteorder::LittleEndian>(*v),
            AnyInt::U48(v) => {
                writer
                    .write_all(AnyInt::write_u48(*v, Endianness::LittleEndian).as_slice())
                    .unwrap();
                Ok(())
            }
            AnyInt::U64(v) => writer.write_u64::<byteorder::LittleEndian>(*v),
            AnyInt::U128(v) => writer.write_u128::<byteorder::LittleEndian>(*v),
            AnyInt::I8(v) => writer.write_i8(*v),
            AnyInt::I16(v) => writer.write_i16::<byteorder::LittleEndian>(*v),
            AnyInt::I32(v) => writer.write_i32::<byteorder::LittleEndian>(*v),
            AnyInt::I48(v) => {
                writer
                    .write_all(AnyInt::write_i48(*v, Endianness::LittleEndian).as_slice())
                    .unwrap();
                Ok(())
            }
            AnyInt::I64(v) => writer.write_i64::<byteorder::LittleEndian>(*v),
            AnyInt::I128(v) => writer.write_i128::<byteorder::LittleEndian>(*v),
            AnyInt::Bool(v) => writer.write_u8(*v as u8),
        }
        .unwrap();

        writer.into_inner()
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        let buf = Vec::with_capacity(self.size());
        let mut writer = Cursor::new(buf);

        match self {
            AnyInt::U8(v) => writer.write_u8(*v),
            AnyInt::U16(v) => writer.write_u16::<byteorder::BigEndian>(*v),
            AnyInt::U32(v) => writer.write_u32::<byteorder::BigEndian>(*v),
            AnyInt::U48(v) => {
                writer
                    .write_all(AnyInt::write_u48(*v, Endianness::BigEndian).as_slice())
                    .unwrap();
                Ok(())
            }
            AnyInt::U64(v) => writer.write_u64::<byteorder::BigEndian>(*v),
            AnyInt::U128(v) => writer.write_u128::<byteorder::BigEndian>(*v),
            AnyInt::I8(v) => writer.write_i8(*v),
            AnyInt::I16(v) => writer.write_i16::<byteorder::BigEndian>(*v),
            AnyInt::I32(v) => writer.write_i32::<byteorder::BigEndian>(*v),
            AnyInt::I48(v) => {
                writer
                    .write_all(AnyInt::write_i48(*v, Endianness::BigEndian).as_slice())
                    .unwrap();
                Ok(())
            }
            AnyInt::I64(v) => writer.write_i64::<byteorder::BigEndian>(*v),
            AnyInt::I128(v) => writer.write_i128::<byteorder::BigEndian>(*v),
            AnyInt::Bool(v) => writer.write_u8(*v as u8),
        }
        .unwrap();

        writer.into_inner()
    }

    /// In memory size of the integer
    pub fn size(&self) -> usize {
        match self {
            AnyInt::U8(_) => 1,
            AnyInt::U16(_) => 2,
            AnyInt::U32(_) => 4,
            AnyInt::U48(_) => 8,
            AnyInt::U64(_) => 8,
            AnyInt::U128(_) => 16,
            AnyInt::I8(_) => 1,
            AnyInt::I16(_) => 2,
            AnyInt::I32(_) => 4,
            AnyInt::I48(_) => 8,
            AnyInt::I64(_) => 8,
            AnyInt::I128(_) => 16,
            AnyInt::Bool(_) => 1,
        }
    }

    /// Size of the integer when serialized
    pub fn ser_size(&self) -> usize {
        match self {
            AnyInt::U8(_) => 1,
            AnyInt::U16(_) => 2,
            AnyInt::U32(_) => 4,
            AnyInt::U48(_) => 6,
            AnyInt::U64(_) => 8,
            AnyInt::U128(_) => 16,
            AnyInt::I8(_) => 1,
            AnyInt::I16(_) => 2,
            AnyInt::I32(_) => 4,
            AnyInt::I48(_) => 6,
            AnyInt::I64(_) => 8,
            AnyInt::I128(_) => 16,
            AnyInt::Bool(_) => 1,
        }
    }

    fn write_u48(v: u64, endianness: Endianness) -> Vec<u8> {
        let mut buf = [0u8; 8];
        let mut cur = Cursor::new(&mut buf[..]);
        match endianness {
            Endianness::LittleEndian => {
                cur.write_u64::<byteorder::LittleEndian>(v).unwrap();
                cur.into_inner()[..6].to_vec()
            }
            Endianness::BigEndian => {
                cur.write_u64::<byteorder::BigEndian>(v).unwrap();
                cur.into_inner()[2..].to_vec()
            }
        }
    }

    fn write_i48(v: i64, endianness: Endianness) -> Vec<u8> {
        let mut buf = [0u8; 8];
        let mut cur = Cursor::new(&mut buf[..]);
        match endianness {
            Endianness::LittleEndian => {
                cur.write_i64::<byteorder::LittleEndian>(v).unwrap();
                cur.into_inner()[..6].to_vec()
            }
            Endianness::BigEndian => {
                cur.write_i64::<byteorder::BigEndian>(v).unwrap();
                cur.into_inner()[2..].to_vec()
            }
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

impl From<bool> for AnyInt {
    fn from(v: bool) -> Self {
        AnyInt::Bool(v)
    }
}

impl TryFrom<AnyInt> for u8 {
    type Error = std::io::Error;

    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::U8(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to u8", v),
            )),
        }
    }
}

impl TryFrom<AnyInt> for u16 {
    type Error = std::io::Error;

    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::U16(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to u16", v),
            )),
        }
    }
}

impl TryFrom<AnyInt> for u32 {
    type Error = std::io::Error;

    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::U32(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to u32", v),
            )),
        }
    }
}

impl TryFrom<AnyInt> for u64 {
    type Error = std::io::Error;

    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::U48(v) => Ok(v),
            AnyInt::U64(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to u64", v),
            )),
        }
    }
}

impl TryFrom<AnyInt> for u128 {
    type Error = std::io::Error;

    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::U128(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to u128", v),
            )),
        }
    }
}

impl TryFrom<AnyInt> for i8 {
    type Error = std::io::Error;

    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::I8(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to i8", v),
            )),
        }
    }
}

impl TryFrom<AnyInt> for i16 {
    type Error = std::io::Error;

    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::I16(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to i16", v),
            )),
        }
    }
}

impl TryFrom<AnyInt> for i32 {
    type Error = std::io::Error;

    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::I32(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to i32", v),
            )),
        }
    }
}

impl TryFrom<AnyInt> for i64 {
    type Error = std::io::Error;

    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::I48(v) => Ok(v),
            AnyInt::I64(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to i64", v),
            )),
        }
    }
}

impl TryFrom<AnyInt> for i128 {
    type Error = std::io::Error;

    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::I128(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to i128", v),
            )),
        }
    }
}

impl TryFrom<AnyInt> for bool {
    type Error = std::io::Error;

    fn try_from(v: AnyInt) -> Result<Self, Self::Error> {
        match v {
            AnyInt::Bool(v) => Ok(v),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Cannot convert {:?} to bool", v),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anyint() {
        let v: AnyInt = 1u8.into();
        assert_eq!(v, AnyInt::U8(1));
        let v: AnyInt = 1u16.into();
        assert_eq!(v, AnyInt::U16(1));
        let v: AnyInt = 1u32.into();
        assert_eq!(v, AnyInt::U32(1));
        let v: AnyInt = 1u64.into();
        assert_eq!(v, AnyInt::U64(1));
        let v: AnyInt = 1u128.into();
        assert_eq!(v, AnyInt::U128(1));
        let v: AnyInt = 1i8.into();
        assert_eq!(v, AnyInt::I8(1));
        let v: AnyInt = 1i16.into();
        assert_eq!(v, AnyInt::I16(1));
        let v: AnyInt = 1i32.into();
        assert_eq!(v, AnyInt::I32(1));
        let v: AnyInt = 1i64.into();
        assert_eq!(v, AnyInt::I64(1));
        let v: AnyInt = 1i128.into();
        assert_eq!(v, AnyInt::I128(1));
    }

    #[test]
    fn test_try_from() {
        let v: AnyInt = 1u8.into();
        let v: u8 = v.try_into().unwrap();
        assert_eq!(v, 1);
        let v: AnyInt = 1u16.into();
        let v: u16 = v.try_into().unwrap();
        assert_eq!(v, 1);
        let v: AnyInt = 1u32.into();
        let v: u32 = v.try_into().unwrap();
        assert_eq!(v, 1);
        let v: AnyInt = 1u64.into();
        let v: u64 = v.try_into().unwrap();
        assert_eq!(v, 1);
        let v: AnyInt = 1u128.into();
        let v: u128 = v.try_into().unwrap();
        assert_eq!(v, 1);
        let v: AnyInt = 1i8.into();
        let v: i8 = v.try_into().unwrap();
        assert_eq!(v, 1);
        let v: AnyInt = 1i16.into();
        let v: i16 = v.try_into().unwrap();
        assert_eq!(v, 1);
        let v: AnyInt = 1i32.into();
        let v: i32 = v.try_into().unwrap();
        assert_eq!(v, 1);
        let v: AnyInt = 1i64.into();
        let v: i64 = v.try_into().unwrap();
        assert_eq!(v, 1);
        let v: AnyInt = 1i128.into();
        let v: i128 = v.try_into().unwrap();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_try_from_error_case() {
        let v: AnyInt = 1u8.into();
        let v: Result<u16, _> = v.try_into();
        assert!(v.is_err());
        let v: AnyInt = 1u16.into();
        let v: Result<u32, _> = v.try_into();
        assert!(v.is_err());
        let v: AnyInt = 1u32.into();
        let v: Result<u64, _> = v.try_into();
        assert!(v.is_err());
        let v: AnyInt = 1u64.into();
        let v: Result<u128, _> = v.try_into();
        assert!(v.is_err());
        let v: AnyInt = 1u128.into();
        let v: Result<u8, _> = v.try_into();
        assert!(v.is_err());
        let v: AnyInt = 1i8.into();
        let v: Result<i16, _> = v.try_into();
        assert!(v.is_err());
        let v: AnyInt = 1i16.into();
        let v: Result<i32, _> = v.try_into();
        assert!(v.is_err());
        let v: AnyInt = 1i32.into();
        let v: Result<i64, _> = v.try_into();
        assert!(v.is_err());
        let v: AnyInt = 1i64.into();
        let v: Result<i128, _> = v.try_into();
        assert!(v.is_err());
        let v: AnyInt = 1i128.into();
        let v: Result<i8, _> = v.try_into();
        assert!(v.is_err());
    }

    #[test]
    fn test_lpwidth_size() {
        let v = LPWidth::LP8;
        assert_eq!(v.size(), 1);
        let v = LPWidth::LP16;
        assert_eq!(v.size(), 2);
        let v = LPWidth::LP32;
        assert_eq!(v.size(), 4);
        let v = LPWidth::LP64;
        assert_eq!(v.size(), 8);
    }

    #[test]
    fn test_usize_fits() {
        assert!(LPWidth::usize_fits(LPWidth::LP8, 0));
        assert!(LPWidth::usize_fits(LPWidth::LP8, 255));
        assert!(!LPWidth::usize_fits(LPWidth::LP8, 256));
        assert!(LPWidth::usize_fits(LPWidth::LP16, 0));
        assert!(LPWidth::usize_fits(LPWidth::LP16, 65535));
        assert!(!LPWidth::usize_fits(LPWidth::LP16, 65536));
        assert!(LPWidth::usize_fits(LPWidth::LP32, 0));
        assert!(LPWidth::usize_fits(LPWidth::LP32, 4294967295));
        assert!(!LPWidth::usize_fits(LPWidth::LP32, 4294967296));
        assert!(LPWidth::usize_fits(LPWidth::LP64, 0));
        assert!(LPWidth::usize_fits(LPWidth::LP64, 18446744073709551615));
    }
}
