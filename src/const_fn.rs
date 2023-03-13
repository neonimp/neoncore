macro_rules! impl_ascii_to_uint {
    ($t:ident, $w:literal) => {
        paste::item!{
            pub const fn [<ascii_to_ $t _le>](s: &[u8; $w]) -> $t {
                $t::from_le_bytes(*s)
            }
        }
        paste::item!{
            pub const fn [<ascii_to_ $t _be>](s: &[u8; $w]) -> $t {
                $t::from_be_bytes(*s)
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_ascii_to_uint_test {
    ($t:ident,$val:expr,$expect_le:expr,$expect_be:expr) => {
        paste::item!{
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

#[cfg(test)]
mod tests {
    use super::*;
    impl_ascii_to_uint_test!(u16, b"AB", 0x4241, 0x4142);
    impl_ascii_to_uint_test!(u32, b"ABCD", 0x44434241, 0x41424344);
    impl_ascii_to_uint_test!(u64, b"ABCDEFGH", 0x4847464544434241, 0x4142434445464748);
    impl_ascii_to_uint_test!(u128, b"ABCDEFGHIJKLMNOP", 0x504f4e4d4c4b4a494847464544434241, 0x4142434445464748494a4b4c4d4e4f50);
}
