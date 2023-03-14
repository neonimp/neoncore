use std::io::{Read, Seek, Write};
use byteorder::WriteBytesExt;

pub mod read;
pub mod write;
pub mod structlang;

pub trait SeekRead: Read + Seek {}
pub trait SeekWrite: Write + Seek {}
pub trait SeekReadWrite: Read + Write + Seek {}

impl<T: Read + Seek> SeekRead for T {}
impl<T: Write + Seek> SeekWrite for T {}
impl<T: Read + Write + Seek> SeekReadWrite for T {}

#[derive(Clone, Copy)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}
pub enum AnyInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
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
            AnyInt::I8(v) => writer.write_i8(*v),
            AnyInt::I16(v) => writer.write_i16::<byteorder::LittleEndian>(*v),
            AnyInt::I32(v) => writer.write_i32::<byteorder::LittleEndian>(*v),
            AnyInt::I64(v) => writer.write_i64::<byteorder::LittleEndian>(*v),
        }.unwrap();

        writer.into_inner()
    }

    fn size(&self) -> usize {
        match self {
            AnyInt::U8(_) => 1,
            AnyInt::U16(_) => 2,
            AnyInt::U32(_) => 4,
            AnyInt::U64(_) => 8,
            AnyInt::I8(_) => 1,
            AnyInt::I16(_) => 2,
            AnyInt::I32(_) => 4,
            AnyInt::I64(_) => 8,
        }
    }
}

