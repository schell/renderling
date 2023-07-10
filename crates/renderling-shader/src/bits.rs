//! Helpers for bit packing/squashing.

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

/// Pack the value of the bits defined by the shift/mask range.
pub fn pack(bits: &mut u32, (shift, mask): (u32, u32), value: u32) {
    // rotate
    *bits = bits.rotate_right(shift);
    // unset
    *bits &= !mask;
    // set
    *bits |= value & mask;
    // unrotate
    *bits = bits.rotate_left(shift);
}

/// Unpack the value of the bits defined by the shift/mask range.
pub fn unpack(bits: u32, (shift, mask): (u32, u32)) -> u32 {
    (bits >> shift) & mask
}
