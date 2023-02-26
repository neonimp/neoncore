use std::io::{BufReader, Cursor, Read, Seek, SeekFrom, Write};


pub type Result<T> = std::result::Result<T, std::io::Error>;

pub enum Endianness {
    LittleEndian,
    BigEndian,
}

/// A trait for reading integers from a stream, with a specified endianness
/// When a read function is called, the stream is advanced by the number of bytes read
/// This is blanketed for all types that implement `Read`.
///
/// This is a sealed trait, and cannot be implemented outside of this crate.
pub trait StreamReadInt: private::Sealed + Read {
    fn rad_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u16(&mut self, endianness: Endianness) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(match endianness {
            Endianness::LittleEndian => u16::from_le_bytes(buf),
            Endianness::BigEndian => u16::from_be_bytes(buf),
        })
    }

    fn read_u32(&mut self, endianness: Endianness) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(match endianness {
            Endianness::LittleEndian => u32::from_le_bytes(buf),
            Endianness::BigEndian => u32::from_be_bytes(buf),
        })
    }

    fn read_u64(&mut self, endianness: Endianness) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(match endianness {
            Endianness::LittleEndian => u64::from_le_bytes(buf),
            Endianness::BigEndian => u64::from_be_bytes(buf),
        })
    }

    fn read_u128(&mut self, endianness: Endianness) -> Result<u128> {
        let mut buf = [0u8; 16];
        self.read_exact(&mut buf)?;
        Ok(match endianness {
            Endianness::LittleEndian => u128::from_le_bytes(buf),
            Endianness::BigEndian => u128::from_be_bytes(buf),
        })
    }

    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(i8::from_le_bytes(buf))
    }

    fn read_i16(&mut self, endianness: Endianness) -> Result<i16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(match endianness {
            Endianness::LittleEndian => i16::from_le_bytes(buf),
            Endianness::BigEndian => i16::from_be_bytes(buf),
        })
    }

    fn read_i32(&mut self, endianness: Endianness) -> Result<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(match endianness {
            Endianness::LittleEndian => i32::from_le_bytes(buf),
            Endianness::BigEndian => i32::from_be_bytes(buf),
        })
    }

    fn read_i64(&mut self, endianness: Endianness) -> Result<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(match endianness {
            Endianness::LittleEndian => i64::from_le_bytes(buf),
            Endianness::BigEndian => i64::from_be_bytes(buf),
        })
    }

    fn read_i128(&mut self, endianness: Endianness) -> Result<i128> {
        let mut buf = [0u8; 16];
        self.read_exact(&mut buf)?;
        Ok(match endianness {
            Endianness::LittleEndian => i128::from_le_bytes(buf),
            Endianness::BigEndian => i128::from_be_bytes(buf),
        })
    }
}

/// Blanket implementation for all types that implement `Read`.
impl<T> StreamReadInt for T where T: Read {}

/// A trait for writing integers to a stream, with a specified endianness
/// When a write function is called, the stream is advanced by the number of bytes written
/// This is blanketed for all types that implement `Write`.
///
/// This is a sealed trait, and cannot be implemented outside of this crate.
pub trait StreamWriteInt: private::Sealed + Write {
    fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write_all(&[value])
    }

    fn write_u16(&mut self, value: u16, endianness: Endianness) -> Result<()> {
        let buf = match endianness {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)
    }

    fn write_u32(&mut self, value: u32, endianness: Endianness) -> Result<()> {
        let buf = match endianness {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)
    }

    fn write_u64(&mut self, value: u64, endianness: Endianness) -> Result<()> {
        let buf = match endianness {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)
    }

    fn write_u128(&mut self, value: u128, endianness: Endianness) -> Result<()> {
        let buf = match endianness {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)
    }

    fn write_i8(&mut self, value: i8) -> Result<()> {
        self.write_all(&value.to_le_bytes())
    }

    fn write_i16(&mut self, value: i16, endianness: Endianness) -> Result<()> {
        let buf = match endianness {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)
    }

    fn write_i32(&mut self, value: i32, endianness: Endianness) -> Result<()> {
        let buf = match endianness {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)
    }

    fn write_i64(&mut self, value: i64, endianness: Endianness) -> Result<()> {
        let buf = match endianness {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)
    }

    fn write_i128(&mut self, value: i128, endianness: Endianness) -> Result<()> {
        let buf = match endianness {
            Endianness::LittleEndian => value.to_le_bytes(),
            Endianness::BigEndian => value.to_be_bytes(),
        };
        self.write_all(&buf)
    }
}

/// Blanket implementation for all types that implement `Write`.
impl<T> StreamWriteInt for T where T: Write {}

/// A trait for reading integers from a slice, with a specified endianness
/// at a specified index.
/// This is blanketed for all types that implement `AsRef<[u8]>`.
///
/// This is a sealed trait, and cannot be implemented outside of this crate.
pub trait SliceReadInt: AsRef<[u8]> + private::Sealed {
    fn read_u8(&self, index: usize) -> Result<u8> {
        let ref_slice = self.as_ref();
        let mut buf = [0u8; 1];
        buf.copy_from_slice(&ref_slice[index..index + 1]);
        Ok(buf[0])
    }

    fn read_u16(&self, index: usize, endianness: Endianness) -> Result<u16> {
        let ref_slice = self.as_ref();
        let mut buf = [0u8; 2];
        buf.copy_from_slice(&ref_slice[index..index + 2]);
        Ok(match endianness {
            Endianness::LittleEndian => u16::from_le_bytes(buf),
            Endianness::BigEndian => u16::from_be_bytes(buf),
        })
    }

    fn read_u32(&self, index: usize, endianness: Endianness) -> Result<u32> {
        let ref_slice = self.as_ref();
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&ref_slice[index..index + 4]);
        Ok(match endianness {
            Endianness::LittleEndian => u32::from_le_bytes(buf),
            Endianness::BigEndian => u32::from_be_bytes(buf),
        })
    }

    fn read_u64(&self, index: usize, endianness: Endianness) -> Result<u64> {
        let ref_slice = self.as_ref();
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&ref_slice[index..index + 8]);
        Ok(match endianness {
            Endianness::LittleEndian => u64::from_le_bytes(buf),
            Endianness::BigEndian => u64::from_be_bytes(buf),
        })
    }

    fn read_u128(&self, index: usize, endianness: Endianness) -> Result<u128> {
        let ref_slice = self.as_ref();
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&ref_slice[index..index + 16]);
        Ok(match endianness {
            Endianness::LittleEndian => u128::from_le_bytes(buf),
            Endianness::BigEndian => u128::from_be_bytes(buf),
        })
    }

    fn read_i8(&self, index: usize) -> Result<i8> {
        let ref_slice = self.as_ref();
        let mut buf = [0u8; 1];
        buf.copy_from_slice(&ref_slice[index..index + 1]);
        Ok(i8::from_le_bytes(buf))
    }

    fn read_i16(&self, index: usize, endianness: Endianness) -> Result<i16> {
        let ref_slice = self.as_ref();
        let mut buf = [0u8; 2];
        buf.copy_from_slice(&ref_slice[index..index + 2]);
        Ok(match endianness {
            Endianness::LittleEndian => i16::from_le_bytes(buf),
            Endianness::BigEndian => i16::from_be_bytes(buf),
        })
    }

    fn read_i32(&self, index: usize, endianness: Endianness) -> Result<i32> {
        let ref_slice = self.as_ref();
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&ref_slice[index..index + 4]);
        Ok(match endianness {
            Endianness::LittleEndian => i32::from_le_bytes(buf),
            Endianness::BigEndian => i32::from_be_bytes(buf),
        })
    }

    fn read_i64(&self, index: usize, endianness: Endianness) -> Result<i64> {
        let ref_slice = self.as_ref();
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&ref_slice[index..index + 8]);
        Ok(match endianness {
            Endianness::LittleEndian => i64::from_le_bytes(buf),
            Endianness::BigEndian => i64::from_be_bytes(buf),
        })
    }

    fn read_i128(&self, index: usize, endianness: Endianness) -> Result<i128> {
        let ref_slice = self.as_ref();
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&ref_slice[index..index + 16]);
        Ok(match endianness {
            Endianness::LittleEndian => i128::from_le_bytes(buf),
            Endianness::BigEndian => i128::from_be_bytes(buf),
        })
    }
}

/// Implement `SliceReadInt` for all types that implement `AsRef<[u8]>`.
impl<T> SliceReadInt for T where T: AsRef<[u8]> {}

mod private {
    pub trait Sealed {}

    impl<T> Sealed for T {}
}
