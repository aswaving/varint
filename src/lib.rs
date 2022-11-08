//! This crate contains traits to encode and decode to and from VarInt.
//!
//! ## Encoding
//! Signed
//!
//! ```
//!    use varint::VarIntEncode;
//!    assert_eq!((-300i32).to_varint(), vec![215, 4]);
//!
//! ```
//!
//! Unsigned
//!
//! ```
//!    use varint::VarIntEncode;
//!    assert_eq!(300u32.to_varint(), vec![172, 2]);
//!
//! ```
//!
//!
//! ## Decoding
//! Signed
//!
//! ```
//!    use varint::VarIntDecode;
//!    assert_eq!(-300i32, i32::from_varint(&vec![215, 4]));
//!
//! ```
//!
//! Unsigned
//!
//! ```
//!    use varint::VarIntDecode;
//!    assert_eq!(300u32, u32::from_varint(&vec![172, 2]));
//!
//! ```
//!
/// Trait to encode the type into a VarInt.
///
/// ZigZag encoding is used for signed integers to reduce the number of bytes in the varint
/// (without it, 10 bytes would be needed in the varint for all negative values).
pub trait VarIntEncode {
    fn to_varint(&self) -> Vec<u8>;
}

/// Trait to decode a byte array into the type.
///
/// Warning: overflow of the target type is not detected!
pub trait VarIntDecode {
    fn from_varint(data: &[u8]) -> Self;
}

macro_rules! impl_varint_unsigned {
    ($t:ty) =>
    (
        impl VarIntEncode for $t {
            fn to_varint(&self) -> Vec<u8> {
                encode(*self as u128)
            }
        }
        impl VarIntDecode for $t {
            fn from_varint(data: &[u8]) -> Self {
                decode(data) as Self
            }
        }
    )
}

macro_rules! impl_varint_signed {
    ($t:ty) =>
    (
        impl VarIntEncode for $t {
            fn to_varint(&self) -> Vec<u8> {
                let value = *self as i128;
                let value = (value << 1) ^ (value >> 63);
                encode(value as u128)
            }
        }
        impl VarIntDecode for $t {
            fn from_varint(data: &[u8]) -> Self {
                let value = decode(data) as i128;
                ((value >> 1) ^ (-(value & 1))) as Self
            }
        }
    )
}

/// Decodes an unsigned 64bit integer into a varint.
pub fn encode(value: u128) -> Vec<u8> {
    let mut value = value;
    let mut output = Vec::<u8>::with_capacity(8);
    while value > 127 {
        output.push(((value as u8) & 127) | 0x80);
        value >>= 7;
    }
    output.push((value as u8) & 127);
    output
}

/// Decodes a byte array into an unsigned 64bit integer.
pub fn decode(data: &[u8]) -> u128 {
    let mut output: u128 = 0;
    for (i, b) in data.iter().enumerate() {
        output |= ((b & 127) as u128) << (7 * i);
        if (b & 0x80) != 0x80 {
            // stop when Most Significant Bit not set (last byte)
            break;
        }
    }
    output
}

impl_varint_unsigned!(u16);
impl_varint_unsigned!(u32);
impl_varint_unsigned!(u64);
impl_varint_signed!(i16);
impl_varint_signed!(i32);
impl_varint_signed!(i64);

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cafe_encode() {
        assert_eq!(0xcafeu16.to_varint(), vec![254, 149, 3]);
    }

    quickcheck! {
        fn encode_decode_i16(val: i16) -> bool {
            val == i16::from_varint(&(val.to_varint()))
        }
    }

    quickcheck! {
        fn encode_decode_i32(val: i32) -> bool {
            val == i32::from_varint(&(val.to_varint()))
        }
    }

    quickcheck! {
        fn encode_decode_i64(val: i64) -> bool {
            val == i64::from_varint(&(val.to_varint()))
        }
    }

    quickcheck! {
        fn encode_decode_u16(val: u16) -> bool {
            val == u16::from_varint(&(val.to_varint()))
        }
    }

    quickcheck! {
        fn encode_decode_u32(val: u32) -> bool {
            val == u32::from_varint(&(val.to_varint()))
        }
    }
    quickcheck! {
        fn encode_decode_u64(val: u64) -> bool {
            val == u64::from_varint(&(val.to_varint()))
        }
    }
}
