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

/// Extract the "nth" 8 bits of the u32 at the given index in the slab.
///
/// Returns the extracted value, the index of the next component and the index
/// of the next u32 in the slab.
pub fn extract_u8(
    // component index, eg 0 for the first 8 bits, 1 for the second 8 bits, etc
    n: u32,
    // index of the u32 in the slab
    u32_index: usize,
    // slab of u32s
    slab: &[u32],
) -> (u32, u32, usize) {
    match n {
        0 => (extract(slab[u32_index], U8_0_BITS), 1, u32_index),
        1 => (extract(slab[u32_index], U8_1_BITS), 2, u32_index),
        2 => (extract(slab[u32_index], U8_2_BITS), 3, u32_index),
        _ => (extract(slab[u32_index], U8_3_BITS), 0, u32_index + 1),
    }
}

/// Extract the "nth" 8 bits of the u32 at the given index in the slab.
///
/// Returns the extracted value, the index of the next component and the index
/// of the next u32 in the slab.
pub fn extract_i8(
    // component index, eg 0 for the first 8 bits, 1 for the second 8 bits, etc
    n: u32,
    // index of the u32 in the slab
    u32_index: usize,
    // slab of u32s
    slab: &[u32],
) -> (i32, u32, usize) {
    let (value, n, u32_index) = extract_u8(n, u32_index, slab);
    let value: i32 = (value as i32 & 0xFF) - ((value as i32 & 0x80) << 1);
    (value, n, u32_index)
}

/// Extract the "nth" 16 bits of the u32 at the given index in the slab.
pub fn extract_u16(
    // component index, eg 0 for the first 16 bits, 1 for the second 16 bits, etc
    n: u32,
    // index of the u32 in the slab
    u32_index: usize,
    // slab of u32s
    slab: &[u32],
) -> (u32, u32, usize) {
    match n {
        0 => (extract(slab[u32_index], U16_0_BITS), 1, u32_index),
        _ => (extract(slab[u32_index], U16_1_BITS), 0, u32_index + 1),
    }
}

/// Extract the "nth" 16 bits of the u32 at the given index in the slab.
pub fn extract_i16(
    // component index, eg 0 for the first 16 bits, 1 for the second 16 bits, etc
    n: u32,
    // index of the u32 in the slab
    u32_index: usize,
    // slab of u32s
    slab: &[u32],
) -> (i32, u32, usize) {
    let (value, n, u32_index) = extract_u16(n, u32_index, slab);
    let value: i32 = (value as i32 & 0xFFFF) - ((value as i32 & 0x8000) << 1);
    (value, n, u32_index)
}

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

    #[test]
    fn extract_u8_sanity() {
        let u8_slab = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 0u8, 0u8];
        let u32_slab: &[u32] = bytemuck::cast_slice(&u8_slab);
        let index = 0;
        let n = 0;
        let (a, n, index) = extract_u8(n, index, u32_slab);
        let (b, n, index) = extract_u8(n, index, u32_slab);
        let (c, n, index) = extract_u8(n, index, u32_slab);
        let (d, n, index) = extract_u8(n, index, u32_slab);
        let (e, n, index) = extract_u8(n, index, u32_slab);
        let (f, _, _) = extract_u8(n, index, u32_slab);
        assert_eq!([0, 1, 2, 3, 4, 5], [a, b, c, d, e, f]);
    }

    #[test]
    fn extract_i8_sanity() {
        let i8_slab = [0i8, -1i8, -2i8, -3i8, 4i8, 5i8, 0i8, 0i8];
        let u32_slab: &[u32] = bytemuck::cast_slice(&i8_slab);
        let index = 0;
        let n = 0;
        let (a, n, index) = extract_i8(n, index, u32_slab);
        let (b, n, index) = extract_i8(n, index, u32_slab);
        let (c, n, index) = extract_i8(n, index, u32_slab);
        let (d, n, index) = extract_i8(n, index, u32_slab);
        let (e, n, index) = extract_i8(n, index, u32_slab);
        let (f, _, _) = extract_i8(n, index, u32_slab);
        assert_eq!([0, -1, -2, -3, 4, 5], [a, b, c, d, e, f]);
    }

    #[test]
    fn extract_u16_sanity() {
        let u16_slab = [0u16, 1u16, 2u16, 3u16, 4u16, 5u16];
        let u32_slab: &[u32] = bytemuck::cast_slice(&u16_slab);
        let index = 0;
        let n = 0;
        let (a, n, index) = extract_u16(n, index, u32_slab);
        let (b, n, index) = extract_u16(n, index, u32_slab);
        let (c, n, index) = extract_u16(n, index, u32_slab);
        let (d, n, index) = extract_u16(n, index, u32_slab);
        let (e, n, index) = extract_u16(n, index, u32_slab);
        let (f, _, _) = extract_u16(n, index, u32_slab);
        assert_eq!([0, 1, 2, 3, 4, 5], [a, b, c, d, e, f]);
    }

    #[test]
    fn extract_i16_sanity() {
        let i16_slab = [0i16, -1i16, -2i16, -3i16, 4i16, 5i16, -12345i16, 0i16];
        let u32_slab: &[u32] = bytemuck::cast_slice(&i16_slab);
        let index = 0;
        let n = 0;
        let (a, n, index) = extract_i16(n, index, u32_slab);
        let (b, n, index) = extract_i16(n, index, u32_slab);
        let (c, n, index) = extract_i16(n, index, u32_slab);
        let (d, n, index) = extract_i16(n, index, u32_slab);
        let (e, n, index) = extract_i16(n, index, u32_slab);
        let (f, n, index) = extract_i16(n, index, u32_slab);
        let (g, _, _) = extract_i16(n, index, u32_slab);
        assert_eq!([0, -1, -2, -3, 4, 5, -12345], [a, b, c, d, e, f, g]);
    }
}
