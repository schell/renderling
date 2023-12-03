//! Helpers for bitwise operations.

use core::ops::RangeInclusive;

/// Statically define a shift/mask range as a literal range of bits.
pub const fn bits(range: RangeInclusive<u32>) -> (u32, u32) {
    let mut start = *range.start();
    let end = *range.end();
    let mut mask = 0;
    while start <= end {
        mask = (mask << 1) | 1;
        start += 1;
    }
    (*range.start(), mask)
}

/// Insert the value of the bits defined by the shift/mask range.
pub fn insert(bits: &mut u32, (shift, mask): (u32, u32), value: u32) {
    // rotate right
    if shift >= 1 {
        *bits = (*bits >> shift) | (*bits << (32 - shift));
    }
    // unset
    *bits &= !mask;
    // set
    *bits |= value & mask;
    // unrotate (rotate left)
    if shift >= 1 {
        *bits = (*bits << shift) | (*bits >> (32 - shift));
    }
}

/// Extract the value of the bits defined by the shift/mask range.
pub fn extract(bits: u32, (shift, mask): (u32, u32)) -> u32 {
    (bits >> shift) & mask
}

/// The shift/mask range for the first 8 bits of a u32.
pub const U8_0_BITS: (u32, u32) = bits(0..=7);
/// The shift/mask range for the second 8 bits of a u32.
pub const U8_1_BITS: (u32, u32) = bits(8..=15);
/// The shift/mask range for the third 8 bits of a u32.
pub const U8_2_BITS: (u32, u32) = bits(16..=23);
/// The shift/mask range for the fourth 8 bits of a u32.
pub const U8_3_BITS: (u32, u32) = bits(24..=31);

/// The shift/mask range for the first 16 bits of a u32.
pub const U16_0_BITS: (u32, u32) = bits(0..=15);
/// The shift/mask range for the second 16 bits of a u32.
pub const U16_1_BITS: (u32, u32) = bits(16..=31);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bits_sanity() {
        let mut store = 0;
        assert_eq!(
            "00000000000000000000000000000000",
            &format!("{:032b}", store)
        );
        insert(&mut store, bits(0..=7), u8::MAX as u32);
        assert_eq!(
            "00000000000000000000000011111111",
            &format!("{:032b}", store)
        );
        store = 0;
        insert(&mut store, bits(8..=15), u8::MAX as u32);
        assert_eq!(
            "00000000000000001111111100000000",
            &format!("{:032b}", store)
        );
    }

    #[test]
    fn bits_u8_sanity() {
        let mut bits = 0;
        println!("bits: {:032b}", bits);
        super::insert(&mut bits, super::U8_0_BITS, 6u8 as u32);
        println!("bits: {:032b}", bits);
        assert_eq!(super::extract(bits, super::U8_0_BITS), 6);
        super::insert(&mut bits, super::U8_1_BITS, 5u8 as u32);
        println!("bits: {:032b}", bits);
        assert_eq!(super::extract(bits, super::U8_0_BITS), 6);
        assert_eq!(super::extract(bits, super::U8_1_BITS), 5);
        super::insert(&mut bits, super::U8_2_BITS, 4u8 as u32);
        println!("bits: {:032b}", bits);
        assert_eq!(super::extract(bits, super::U8_0_BITS), 6);
        assert_eq!(super::extract(bits, super::U8_1_BITS), 5);
        assert_eq!(super::extract(bits, super::U8_2_BITS), 4);
        super::insert(&mut bits, super::U8_3_BITS, 3u8 as u32);
        println!("bits: {:032b}", bits);
        assert_eq!(super::extract(bits, super::U8_0_BITS), 6);
        assert_eq!(super::extract(bits, super::U8_1_BITS), 5);
        assert_eq!(super::extract(bits, super::U8_2_BITS), 4);
        assert_eq!(super::extract(bits, super::U8_3_BITS), 3);
    }
}
