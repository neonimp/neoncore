use crate::varint::VarintError::DeserializeBadVarint;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum VarintError {
    DeserializeBadVarint,
}

impl Display for VarintError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeBadVarint => write!(f, "Attempted to deserialize bad varint"),
        }
    }
}

impl std::error::Error for VarintError {}

pub type Result<T> = core::result::Result<T, VarintError>;

/// Returns the maximum number of bytes required to encode T.
pub const fn varint_max<T: Sized>() -> usize {
    const BITS_PER_BYTE: usize = 8;
    const BITS_PER_VARINT_BYTE: usize = 7;

    // How many data bits do we need for this type?
    let bits = core::mem::size_of::<T>() * BITS_PER_BYTE;

    // We add (BITS_PER_BYTE - 1), to ensure any integer divisions
    // with a remainder will always add exactly one full byte, but
    // an evenly divided number of bits will be the same
    let roundup_bits = bits + (BITS_PER_BYTE - 1);

    // Apply division, using normal "round down" integer division
    roundup_bits / BITS_PER_VARINT_BYTE
}

/// Returns the maximum value stored in the last encoded byte.
pub const fn max_of_last_byte<T: Sized>() -> u8 {
    let max_bits = core::mem::size_of::<T>() * 8;
    let extra_bits = max_bits % 7;
    (1 << extra_bits) - 1
}

#[inline]
pub fn varint_usize(n: usize, out: &mut [u8; varint_max::<usize>()]) -> &mut [u8] {
    let mut value = n;
    for i in 0..varint_max::<usize>() {
        out[i] = value.to_le_bytes()[0];
        if value < 128 {
            return &mut out[..=i];
        }

        out[i] |= 0x80;
        value >>= 7;
    }
    debug_assert_eq!(value, 0);
    &mut out[..]
}

#[inline]
pub fn varint_u16(n: u16, out: &mut [u8; varint_max::<u16>()]) -> &mut [u8] {
    let mut value = n;
    for i in 0..varint_max::<u16>() {
        out[i] = value.to_le_bytes()[0];
        if value < 128 {
            return &mut out[..=i];
        }

        out[i] |= 0x80;
        value >>= 7;
    }
    debug_assert_eq!(value, 0);
    &mut out[..]
}

#[inline]
pub fn varint_u32(n: u32, out: &mut [u8; varint_max::<u32>()]) -> &mut [u8] {
    let mut value = n;
    for i in 0..varint_max::<u32>() {
        out[i] = value.to_le_bytes()[0];
        if value < 128 {
            return &mut out[..=i];
        }

        out[i] |= 0x80;
        value >>= 7;
    }
    debug_assert_eq!(value, 0);
    &mut out[..]
}

#[inline]
pub fn varint_u64(n: u64, out: &mut [u8; varint_max::<u64>()]) -> &mut [u8] {
    let mut value = n;
    for i in 0..varint_max::<u64>() {
        out[i] = value.to_le_bytes()[0];
        if value < 128 {
            return &mut out[..=i];
        }

        out[i] |= 0x80;
        value >>= 7;
    }
    debug_assert_eq!(value, 0);
    &mut out[..]
}

#[inline]
pub fn varint_u128(n: u128, out: &mut [u8; varint_max::<u128>()]) -> &mut [u8] {
    let mut value = n;
    for i in 0..varint_max::<u128>() {
        out[i] = value.to_le_bytes()[0];
        if value < 128 {
            return &mut out[..=i];
        }

        out[i] |= 0x80;
        value >>= 7;
    }
    debug_assert_eq!(value, 0);
    &mut out[..]
}

pub trait TryTakeVarint<T: Sized> {
    #[inline]
    fn try_take_varint_u16(data: &[u8; varint_max::<u16>()]) -> Result<u16> {
        let mut out = 0;
        for i in 0..varint_max::<u16>() {
            let val = data[i];
            let carry = (val & 0x7F) as u16;
            out |= carry << (7 * i);

            if (val & 0x80) == 0 {
                return if i == varint_max::<u16>() - 1 && val > max_of_last_byte::<u16>() {
                    Err(DeserializeBadVarint)
                } else {
                    Ok(out)
                };
            }
        }
        Err(DeserializeBadVarint)
    }

    #[inline]
    fn try_take_varint_u32(data: &[u8; varint_max::<u32>()]) -> Result<u32> {
        let mut out = 0;
        for i in 0..varint_max::<u32>() {
            let val = data[i];
            let carry = (val & 0x7F) as u32;
            out |= carry << (7 * i);

            if (val & 0x80) == 0 {
                return if i == varint_max::<u32>() - 1 && val > max_of_last_byte::<u32>() {
                    Err(DeserializeBadVarint)
                } else {
                    Ok(out)
                };
            }
        }
        Err(DeserializeBadVarint)
    }

    #[inline]
    fn try_take_varint_u64(data: &[u8; varint_max::<u64>()]) -> Result<u64> {
        let mut out = 0;
        for i in 0..varint_max::<u64>() {
            let val = data[i];
            let carry = (val & 0x7F) as u64;
            out |= carry << (7 * i);

            if (val & 0x80) == 0 {
                return if i == varint_max::<u64>() - 1 && val > max_of_last_byte::<u64>() {
                    Err(DeserializeBadVarint)
                } else {
                    Ok(out)
                };
            }
        }
        Err(DeserializeBadVarint)
    }

    #[inline]
    fn try_take_varint_u128(data: &[u8; varint_max::<u128>()]) -> Result<u128> {
        let mut out = 0;
        for i in 0..varint_max::<u128>() {
            let val = data[i];
            let carry = (val & 0x7F) as u128;
            out |= carry << (7 * i);

            if (val & 0x80) == 0 {
                return if i == varint_max::<u128>() - 1 && val > max_of_last_byte::<u128>() {
                    Err(DeserializeBadVarint)
                } else {
                    Ok(out)
                };
            }
        }
        Err(DeserializeBadVarint)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_take_varint_usize(data: &[u8; varint_max::<usize>()]) -> Result<usize> {
        Self::try_take_varint_u16(data).map(|x| x as usize)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_take_varint_usize(data: &[u8; varint_max::<usize>()]) -> Result<usize> {
        Self::try_take_varint_u32(data).map(|x| x as usize)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_take_varint_usize(data: &[u8; varint_max::<usize>()]) -> Result<usize> {
        Self::try_take_varint_u64(data).map(|x| x as usize)
    }
}

impl TryTakeVarint<u16> for u16 {}
impl TryTakeVarint<u32> for u32 {}
impl TryTakeVarint<u64> for u64 {}
impl TryTakeVarint<u128> for u128 {}
impl TryTakeVarint<usize> for usize {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_u16() {
        let mut out = [0u8; varint_max::<u16>()];
        for i in 0..=u16::MAX {
            varint_u16(i, &mut out);
            let val = u16::try_take_varint_u16(&out).unwrap();
            assert_eq!(val, i);
        }
    }

    #[test]
    fn test_varint_u32() {
        let mut out = [0u8; varint_max::<u32>()];
        for i in 0..=u32::MAX {
            varint_u32(i, &mut out);
            let val = u32::try_take_varint_u32(&out).unwrap();
            assert_eq!(val, i);
        }
    }

    #[test]
    fn test_varint_u64() {
        let mut out = [0u8; varint_max::<u64>()];
        for i in 0..=u64::MAX {
            varint_u64(i, &mut out);
            let val = u64::try_take_varint_u64(&out).unwrap();
            assert_eq!(val, i);
        }
    }

    #[test]
    fn test_varint_u128() {
        let mut out = [0u8; varint_max::<u128>()];
        for i in 0..=u128::MAX {
            varint_u128(i, &mut out);
            let val = u128::try_take_varint_u128(&out).unwrap();
            assert_eq!(val, i);
        }
    }
}
