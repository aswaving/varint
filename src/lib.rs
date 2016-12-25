///
pub trait VarIntEncode {
    /// Encodes the type into a VarInt
    fn to_varint(&self) -> Vec<u8>;
}

pub trait VarIntDecode {
    /// Decodes a byte array into the type
    /// Warning: overflow of the target type is not detected!
    fn from_varint(data: &[u8]) -> Self;
}

macro_rules! impl_varint_unsigned {
    ($t:ty) =>
    (
        impl VarIntEncode for $t {
            fn to_varint(&self) -> Vec<u8> {
                encode(*self as u64)
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
                let value = *self as i64;
                let value = (value << 1) ^ (value >> 63);
                encode(value as u64)
            }
        }
        impl VarIntDecode for $t {
            fn from_varint(data: &[u8]) -> Self {
                let value = decode(data) as i64;
                ((value >> 1) ^ (-(value & 1))) as Self
            }
        }
    )
}
pub fn encode(value: u64) -> Vec<u8> {
    let mut value = value;
    let mut output = Vec::<u8>::with_capacity(8);
    while value > 127 {
        output.push(((value as u8) & 127) | 0x80);
        value >>= 7;
    }
    output.push((value as u8) & 127);
    output
}

/// Decodes the byte array into an unsigned 64bit integer.
pub fn decode(data: &[u8]) -> u64 {
    let mut output = 0u64;
    for (i, b) in data.into_iter().enumerate() {
        output |= ((b & 127) as u64) << (7 * i);
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

    #[test]
    fn encode_300_unsigned() {
        assert_eq!(300u16.to_varint(), vec![172, 2]);
    }

    #[test]
    fn encode_minus_300_signed() {
        assert_eq!((-300i16).to_varint(), 599u16.to_varint());
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
