// SPDX-FileCopyrightText: 2024 Schell Scivally <efsubenovex@gmail.com>>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Helpers for bitwise operations.

use core::ops::RangeInclusive;

use crabslab::{Id, Slab};

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

/// Extract 8 bits of the u32 at the given index in the slab.
///
/// Returns the extracted value, the index of the next component and the index
/// of the next u32 in the slab.
pub fn extract_u8(
    // index of the u32 in the slab
    u32_index: usize,
    // eg 0 for the first 8 bits, 1 for the second 8 bits, etc
    byte_offset: usize,
    // slab of u32s
    slab: &[u32],
) -> (u32, usize, usize) {
    const SHIFT_MASKS: [((u32, u32), usize); 4] = [
        (U8_0_BITS, 0),
        (U8_1_BITS, 0),
        (U8_2_BITS, 0),
        (U8_3_BITS, 1),
    ];
    let byte_mod = byte_offset % 4;
    let (shift_mask, index_inc) = SHIFT_MASKS[byte_mod];
    let u32_value = slab.read(Id::from(u32_index));
    let value = extract(u32_value, shift_mask);
    (value, u32_index + index_inc, byte_mod + 1)
}

/// Extract 8 bits of the u32 at the given index in the slab.
///
/// Returns the extracted value, the index of the next component and the index
/// of the next u32 in the slab.
pub fn extract_i8(
    // index of the u32 in the slab
    u32_index: usize,
    // eg 0 for the first 8 bits, 1 for the second 8 bits, etc
    byte_offset: usize,
    // slab of u32s
    slab: &[u32],
) -> (i32, usize, usize) {
    let (value, u32_index, n) = extract_u8(u32_index, byte_offset, slab);
    let value: i32 = (value as i32 & 0xFF) - ((value as i32 & 0x80) << 1);
    (value, u32_index, n)
}

/// Extract 16 bits of the u32 at the given index in the slab.
pub fn extract_u16(
    // index of the u32 in the slab
    u32_index: usize,
    // eg 0 for the first 16 bits, 2 for the second 16 bits, etc
    byte_offset: usize,
    // slab of u32s
    slab: &[u32],
) -> (u32, usize, usize) {
    // NOTE: This should only have two entries, but we'll still handle the case
    // where the extraction is not aligned to a u32 boundary by reading as if it
    // were, and then re-aligning.
    const SHIFT_MASKS: [((u32, u32), usize, usize); 4] = [
        (U16_0_BITS, 2, 0),
        (U16_0_BITS, 2, 0),
        (U16_1_BITS, 0, 1),
        (U16_1_BITS, 0, 1),
    ];
    let byte_mod = byte_offset % 4;
    crate::println!("byte_mod: {byte_mod}");
    let (shift_mask, next_byte_offset, index_inc) = SHIFT_MASKS[byte_mod];
    let u32_value = slab.read(Id::from(u32_index));
    crate::println!("u32: {:032b}", u32_value);
    let value = extract(u32_value, shift_mask);
    crate::println!("u16: {:016b}", value);
    crate::println!("u32: {:?}", u32_value);
    (value, u32_index + index_inc, next_byte_offset)
}

/// Extract 16 bits of the u32 at the given index in the slab.
pub fn extract_i16(
    // index of the u32 in the slab
    u32_index: usize,
    // eg 0 for the first 16 bits, 1 for the second 16 bits, etc
    byte_offset: usize,
    // slab of u32s
    slab: &[u32],
) -> (i32, usize, usize) {
    let (value, u32_index, n) = extract_u16(u32_index, byte_offset, slab);
    let value: i32 = (value as i32 & 0xFFFF) - ((value as i32 & 0x8000) << 1);
    (value, u32_index, n)
}

/// Extract 32 bits of the u32 at the given index in the slab.
pub fn extract_u32(
    // index of the u32 in the slab
    u32_index: usize,
    // ignored and always passed back as `0`
    _byte_offset: usize,
    // slab of u32s
    slab: &[u32],
) -> (u32, usize, usize) {
    (slab.read(Id::from(u32_index)), u32_index + 1, 0)
}

/// Extract 32 bits of the u32 at the given index in the slab.
pub fn extract_i32(
    // index of the u32 in the slab
    u32_index: usize,
    // ignored and always passed back as `0`
    _byte_offset: usize,
    // slab of u32s
    slab: &[u32],
) -> (i32, usize, usize) {
    let (value, _, _) = extract_u32(u32_index, 0, slab);
    (value as i32, u32_index + 1, 0)
}

/// Extract 32 bits of the u32 at the given index in the slab.
pub fn extract_f32(
    // index of the u32 in the slab
    u32_index: usize,
    // ignored and always passed back as `0`
    _byte_offset: usize,
    // slab of u32s
    slab: &[u32],
) -> (f32, usize, usize) {
    let (value, _, _) = extract_u32(u32_index, 0, slab);
    (f32::from_bits(value), u32_index + 1, 0)
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
        let (a, index, n) = extract_u8(index, n, u32_slab);
        let (b, index, n) = extract_u8(index, n, u32_slab);
        let (c, index, n) = extract_u8(index, n, u32_slab);
        let (d, index, n) = extract_u8(index, n, u32_slab);
        let (e, index, n) = extract_u8(index, n, u32_slab);
        let (f, _, _) = extract_u8(index, n, u32_slab);
        assert_eq!([0, 1, 2, 3, 4, 5], [a, b, c, d, e, f]);
    }

    #[test]
    fn extract_i8_sanity() {
        let i8_slab = [0i8, -1i8, -2i8, -3i8, 4i8, 5i8, 0i8, 0i8];
        let u32_slab: &[u32] = bytemuck::cast_slice(&i8_slab);
        let index = 0;
        let n = 0;
        let (a, index, n) = extract_i8(index, n, u32_slab);
        let (b, index, n) = extract_i8(index, n, u32_slab);
        let (c, index, n) = extract_i8(index, n, u32_slab);
        let (d, index, n) = extract_i8(index, n, u32_slab);
        let (e, index, n) = extract_i8(index, n, u32_slab);
        let (f, _, _) = extract_i8(index, n, u32_slab);
        assert_eq!([0, -1, -2, -3, 4, 5], [a, b, c, d, e, f]);
    }

    #[test]
    fn extract_u16_sanity() {
        let u16_slab = [0u16, 1u16, 2u16, 3u16, 4u16, 5u16];
        let u32_slab: &[u32] = bytemuck::cast_slice(&u16_slab);
        let index = 0;
        let n = 0;
        let (a, index, n) = extract_u16(index, n, u32_slab);
        let (b, index, n) = extract_u16(index, n, u32_slab);
        let (c, index, n) = extract_u16(index, n, u32_slab);
        let (d, index, n) = extract_u16(index, n, u32_slab);
        let (e, index, n) = extract_u16(index, n, u32_slab);
        let (f, _, _) = extract_u16(index, n, u32_slab);
        assert_eq!([0, 1, 2, 3, 4, 5], [a, b, c, d, e, f]);
    }

    #[test]
    fn extract_i16_sanity() {
        let i16_slab = [0i16, -1i16, -2i16, -3i16, 4i16, 5i16, -12345i16, 0i16];
        let u32_slab: &[u32] = bytemuck::cast_slice(&i16_slab);
        let index = 0;
        let n = 0;
        let (a, index, n) = extract_i16(index, n, u32_slab);
        let (b, index, n) = extract_i16(index, n, u32_slab);
        let (c, index, n) = extract_i16(index, n, u32_slab);
        let (d, index, n) = extract_i16(index, n, u32_slab);
        let (e, index, n) = extract_i16(index, n, u32_slab);
        let (f, index, n) = extract_i16(index, n, u32_slab);
        let (g, _, _) = extract_i16(index, n, u32_slab);
        assert_eq!([0, -1, -2, -3, 4, 5, -12345], [a, b, c, d, e, f, g]);
    }

    #[test]
    fn extract_u32_sanity() {
        let u32_slab = [0u32, 1u32, 2u32, 3u32, 4u32, 5u32];
        let u32_slab: &[u32] = bytemuck::cast_slice(&u32_slab);
        let index = 0;
        let n = 0;
        let (a, index, n) = extract_u32(index, n, u32_slab);
        let (b, index, n) = extract_u32(index, n, u32_slab);
        let (c, index, n) = extract_u32(index, n, u32_slab);
        let (d, index, n) = extract_u32(index, n, u32_slab);
        let (e, index, n) = extract_u32(index, n, u32_slab);
        let (f, _, _) = extract_u32(index, n, u32_slab);
        assert_eq!([0, 1, 2, 3, 4, 5], [a, b, c, d, e, f]);
    }

    #[test]
    fn extract_i32_sanity() {
        let i32_slab = [0i32, -1i32, -2i32, -3i32, 4i32, 5i32, -12345i32];
        let u32_slab: &[u32] = bytemuck::cast_slice(&i32_slab);
        let index = 0;
        let n = 0;
        let (a, index, n) = extract_i32(index, n, u32_slab);
        let (b, index, n) = extract_i32(index, n, u32_slab);
        let (c, index, n) = extract_i32(index, n, u32_slab);
        let (d, index, n) = extract_i32(index, n, u32_slab);
        let (e, index, n) = extract_i32(index, n, u32_slab);
        let (f, index, n) = extract_i32(index, n, u32_slab);
        let (g, _, _) = extract_i32(index, n, u32_slab);
        assert_eq!([0, -1, -2, -3, 4, 5, -12345], [a, b, c, d, e, f, g]);
    }

    #[test]
    fn extract_f32_sanity() {
        let f32_slab = [
            0.0f32,
            -1.0f32,
            -2.0f32,
            -3.0f32,
            4.0f32,
            5.0f32,
            -12345.0f32,
        ];
        let u32_slab: &[u32] = bytemuck::cast_slice(&f32_slab);
        let index = 0;
        let n = 0;
        let (a, index, n) = extract_f32(index, n, u32_slab);
        let (b, index, n) = extract_f32(index, n, u32_slab);
        let (c, index, n) = extract_f32(index, n, u32_slab);
        let (d, index, n) = extract_f32(index, n, u32_slab);
        let (e, index, n) = extract_f32(index, n, u32_slab);
        let (f, index, n) = extract_f32(index, n, u32_slab);
        let (g, _, _) = extract_f32(index, n, u32_slab);
        assert_eq!(
            [0f32, -1f32, -2f32, -3f32, 4f32, 5f32, -12345f32],
            [a, b, c, d, e, f, g]
        );
    }
}
