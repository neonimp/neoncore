//! Const functions for ascii to uint
//! and some related functions.

macro_rules! impl_ascii_to_uint {
    ($t:ident, $w:literal) => {
        paste::item! {
            pub const fn [<ascii_to_ $t _le>](s: &[u8; $w]) -> $t {
                $t::from_le_bytes(*s)
            }
        }
        paste::item! {
            pub const fn [<ascii_to_ $t _be>](s: &[u8; $w]) -> $t {
                $t::from_be_bytes(*s)
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_ascii_to_uint_test {
    ($t:ident,$val:expr,$expect_le:expr,$expect_be:expr) => {
        paste::item! {
            #[test]
            fn [<test_ $t>]() {
                assert_eq!([<ascii_to_ $t _le>]($val), $expect_le);
                assert_eq!([<ascii_to_ $t _be>]($val), $expect_be);
            }
        }
    };
}

impl_ascii_to_uint!(u16, 2);
impl_ascii_to_uint!(u32, 4);
impl_ascii_to_uint!(u64, 8);
impl_ascii_to_uint!(u128, 16);

pub const fn kib_to_byte(kib: usize) -> usize {
    kib * 1024
}

pub const fn mib_to_byte(mib: usize) -> usize {
    kib_to_byte(mib) * 1024
}

pub const fn gib_to_byte(gib: usize) -> usize {
    mib_to_byte(gib) * 1024
}

pub const fn tib_to_byte(tib: usize) -> usize {
    gib_to_byte(tib) * 1024
}

pub const fn byte_to_kib(byte: usize) -> usize {
    byte / 1024
}

pub const fn byte_to_mib(byte: usize) -> usize {
    byte_to_kib(byte) / 1024
}

pub const fn byte_to_gib(byte: usize) -> usize {
    byte_to_mib(byte) / 1024
}

pub const fn byte_to_tib(byte: usize) -> usize {
    byte_to_gib(byte) / 1024
}

#[cfg(test)]
mod tests {
    use super::*;
    impl_ascii_to_uint_test!(u16, b"AB", 0x4241, 0x4142);
    impl_ascii_to_uint_test!(u32, b"ABCD", 0x44434241, 0x41424344);
    impl_ascii_to_uint_test!(u64, b"ABCDEFGH", 0x4847464544434241, 0x4142434445464748);
    impl_ascii_to_uint_test!(
        u128,
        b"ABCDEFGHIJKLMNOP",
        0x504f4e4d4c4b4a494847464544434241,
        0x4142434445464748494a4b4c4d4e4f50
    );

    #[test]
    fn test_kib_to_byte() {
        assert_eq!(kib_to_byte(1), 1024);
        assert_eq!(kib_to_byte(2), 2048);
        assert_eq!(kib_to_byte(3), 3072);
    }

    #[test]
    fn test_mib_to_byte() {
        assert_eq!(mib_to_byte(1), 1024 * 1024);
        assert_eq!(mib_to_byte(2), 1024 * 1024 * 2);
        assert_eq!(mib_to_byte(3), 1024 * 1024 * 3);
    }

    #[test]
    fn test_gib_to_byte() {
        assert_eq!(gib_to_byte(1), 1024 * 1024 * 1024);
        assert_eq!(gib_to_byte(2), 1024 * 1024 * 1024 * 2);
        assert_eq!(gib_to_byte(3), 1024 * 1024 * 1024 * 3);
    }

    #[test]
    fn test_tib_to_byte() {
        assert_eq!(tib_to_byte(1), 1024 * 1024 * 1024 * 1024);
        assert_eq!(tib_to_byte(2), 1024 * 1024 * 1024 * 1024 * 2);
        assert_eq!(tib_to_byte(3), 1024 * 1024 * 1024 * 1024 * 3);
    }

    #[test]
    fn test_byte_to_kib() {
        assert_eq!(byte_to_kib(1024), 1);
        assert_eq!(byte_to_kib(2048), 2);
        assert_eq!(byte_to_kib(3072), 3);
    }

    #[test]
    fn test_byte_to_mib() {
        assert_eq!(byte_to_mib(1024 * 1024), 1);
        assert_eq!(byte_to_mib(1024 * 1024 * 2), 2);
        assert_eq!(byte_to_mib(1024 * 1024 * 3), 3);
    }

    #[test]
    fn test_byte_to_gib() {
        assert_eq!(byte_to_gib(1024 * 1024 * 1024), 1);
        assert_eq!(byte_to_gib(1024 * 1024 * 1024 * 2), 2);
        assert_eq!(byte_to_gib(1024 * 1024 * 1024 * 3), 3);
    }

    #[test]
    fn test_byte_to_tib() {
        assert_eq!(byte_to_tib(1024 * 1024 * 1024 * 1024), 1);
        assert_eq!(byte_to_tib(1024 * 1024 * 1024 * 1024 * 2), 2);
        assert_eq!(byte_to_tib(1024 * 1024 * 1024 * 1024 * 3), 3);
    }
}
