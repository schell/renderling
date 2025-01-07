//! Managing ranges of values.

use crabslab::{Array, Slab, SlabItem};

use crate::runtime::SlabUpdate;

#[derive(Clone, Copy, PartialEq)]
pub struct Range {
    pub first_index: u32,
    pub last_index: u32,
}

impl core::fmt::Debug for Range {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&format!("{}..={}", self.first_index, self.last_index))
    }
}

impl<T: SlabItem> From<Array<T>> for Range {
    fn from(array: Array<T>) -> Self {
        let array = array.into_u32_array();
        let first_index = array.starting_index() as u32;
        Range {
            first_index,
            last_index: first_index + array.len() as u32 - 1,
        }
    }
}

impl Range {
    pub fn len(&self) -> u32 {
        1 + self.last_index - self.first_index
    }

    pub fn is_empty(&self) -> bool {
        self.last_index == self.first_index
    }

    pub fn intersects(&self, other: &Range) -> bool {
        !(self.first_index > other.last_index || self.last_index < other.first_index)
    }
}

/// Represents a block of contiguous numbers.
pub trait IsRange {
    /// Returns `true` if the two ranges overlap or "touch".
    fn should_merge_with(&self, other: &Self) -> bool;

    /// Returns the union of two ranges.
    fn union(&mut self, other: Self);
}

impl IsRange for Range {
    fn should_merge_with(&self, other: &Self) -> bool {
        debug_assert!(
            !self.intersects(other),
            "{self:?} intersects existing {other:?}, should never happen with Range"
        );

        self.last_index + 1 == other.first_index || self.first_index == other.last_index + 1
    }

    fn union(&mut self, other: Self) {
        *self = Range {
            first_index: self.first_index.min(other.first_index),
            last_index: self.last_index.max(other.last_index),
        };
    }
}

impl IsRange for SlabUpdate {
    fn should_merge_with(&self, other: &Self) -> bool {
        self.intersects(other)
    }

    fn union(&mut self, other: Self) {
        if self.array == other.array {
            *self = other;
            return;
        }

        let mut array = self.array;
        array.union(&other.array);

        let mut elements = vec![0u32; array.len()];

        let self_index = self.array.index - array.index;
        elements.write_indexed_slice(&self.elements, self_index as usize);
        let other_index = other.array.index - array.index;
        elements.write_indexed_slice(&other.elements, other_index as usize);

        self.array = array;
        self.elements = elements;
    }
}

/// Manages contiguous ranges.
pub struct RangeManager<R> {
    pub ranges: Vec<R>,
}

impl<R> Default for RangeManager<R> {
    fn default() -> Self {
        Self { ranges: vec![] }
    }
}

impl<R: IsRange> RangeManager<R> {
    /// Return the number of distinct ranges being managed.
    pub fn len(&self) -> usize {
        self.ranges.len()
    }

    /// Return whether this manager is managing any ranges.
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub fn add_range(&mut self, input_range: R) {
        for range in self.ranges.iter_mut() {
            if range.should_merge_with(&input_range) {
                range.union(input_range);
                return;
            }
        }
        self.ranges.push(input_range);
    }
}

impl RangeManager<Range> {
    /// Removes a range of `count` elements, if possible.
    pub fn remove(&mut self, count: u32) -> Option<Range> {
        let mut remove_index = usize::MAX;
        for (i, range) in self.ranges.iter_mut().enumerate() {
            // This is potentially a hot path, so use the `if` even
            // though clippy complains (because using match is slower)
            #[allow(clippy::comparison_chain)]
            if range.len() > count {
                let first_index = range.first_index;
                let last_index = range.first_index + count - 1;
                range.first_index += count;
                return Some(Range {
                    first_index,
                    last_index,
                });
            } else if range.len() == count {
                remove_index = i;
                break;
            }
        }

        if remove_index == usize::MAX {
            None
        } else {
            Some(self.ranges.swap_remove(remove_index))
        }
    }
}
