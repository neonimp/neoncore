use crate::streams::read::StreamResult;
use crate::streams::{Endianness, LPWidth};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::Read;

pub(crate) fn read_lpend<S: Read>(
    mut stream: S,
    lptype: LPWidth,
    lpend: Endianness,
) -> StreamResult<usize> {
    Ok(match lpend {
        Endianness::BigEndian => match lptype {
            LPWidth::LP8 => stream.read_u8()? as usize,
            LPWidth::LP16 => stream.read_u16::<BigEndian>()? as usize,
            LPWidth::LP32 => stream.read_u32::<BigEndian>()? as usize,
            LPWidth::LP64 => stream.read_u64::<BigEndian>()? as usize,
        },
        Endianness::LittleEndian => match lptype {
            LPWidth::LP8 => stream.read_u8()? as usize,
            LPWidth::LP16 => stream.read_u16::<LittleEndian>()? as usize,
            LPWidth::LP32 => stream.read_u32::<LittleEndian>()? as usize,
            LPWidth::LP64 => stream.read_u64::<LittleEndian>()? as usize,
        },
    })
}
