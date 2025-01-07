//! GPU and CPU slab allocation.
//!
//! Re-exports [`Array`], [`Id`], [`Slab`] and [`SlabItem`] from
//! [`crabslab`](https://docs.rs/crabslab/latest/crabslab/).
//!
//! User types can automatically derive `SlabItem` in most cases. It is
//! required that your type's fields all implement `SlabItem` and `crabslab`
//! must be in scope.
//!
//! ```
//! use renderling::slab::SlabItem;
//!
//! #[derive(Clone, Copy, SlabItem)]
//! struct UserData {
//!     pos: (f32, f32, f32),
//!     acc: (f32, f32, f32),
//! }
//! ```

pub mod range;
pub mod runtime;
pub mod slab;
pub mod value;

pub mod prelude {
    //! Easy-include prelude module.

    pub use super::runtime::*;
    pub use super::slab::*;
    pub use super::value::*;
}

// #[cfg(feature = "wgpu")]
// mod wgpu_slab;
// #[cfg(feature = "wgpu")]
// pub use wgpu_slab::*;

// #[cfg(test)]
// mod test {
//     pub use crabslab::{Array, Id, Slab, SlabItem};

//     use crate::slab::SlabAllocator;

//     #[test]
//     fn mngr_updates_count_sanity() {
//         let mngr = SlabAllocator::<Mutex<Vec<u32>>>::default();
//         {
//             let value = mngr.new_value(666u32);
//             assert_eq!(
//                 1,
//                 value.ref_count(),
//                 "slab should not retain a count on value"
//             );
//         }
//         let _ = mngr.upkeep(());
//         assert_eq!(
//             0,
//             mngr.update_sources.read().unwrap().len(),
//             "value should have dropped with no refs"
//         );
//         {
//             let values = mngr.new_array([666u32, 420u32]);
//             assert_eq!(
//                 1,
//                 values.ref_count(),
//                 "slab should not retain a count on array"
//             );
//         }
//         let _ = mngr.upkeep(());
//         assert_eq!(
//             0,
//             mngr.update_sources.read().unwrap().len(),
//             "array should have dropped with no refs"
//         );
//     }

//     #[test]
//     fn range_sanity() {
//         let a = Range {
//             first_index: 1,
//             last_index: 2,
//         };
//         let b = Range {
//             first_index: 0,
//             last_index: 0,
//         };
//         assert!(!a.intersects(&b));
//         assert!(!b.intersects(&a));
//     }

//     #[test]
//     fn slab_manager_sanity() {
//         let m = SlabAllocator::<Mutex<Vec<u32>>>::default();
//         log::info!("allocating 4 unused u32 slots");
//         let _ = m.allocate::<u32>();
//         let _ = m.allocate::<u32>();
//         let _ = m.allocate::<u32>();
//         let _ = m.allocate::<u32>();

//         log::info!("creating 4 update sources");
//         let h4 = m.new_value(0u32);
//         let h5 = m.new_value(0u32);
//         let h6 = m.new_value(0u32);
//         let h7 = m.new_value(0u32);
//         log::info!("running upkeep");
//         let _ = m.upkeep(());
//         assert!(m.recycles.read().unwrap().ranges.is_empty());
//         assert_eq!(4, m.update_sources.read().unwrap().len());
//         let k = m.update_k.load(Ordering::Relaxed);
//         assert_eq!(4, k);

//         log::info!("dropping 4 update sources");
//         drop(h4);
//         drop(h5);
//         drop(h6);
//         drop(h7);
//         let _ = m.upkeep(());
//         assert_eq!(1, m.recycles.read().unwrap().ranges.len());
//         assert!(m.update_sources.read().unwrap().is_empty());

//         log::info!("creating 4 update sources, round two");
//         let h4 = m.new_value(0u32);
//         let h5 = m.new_value(0u32);
//         let h6 = m.new_value(0u32);
//         let h7 = m.new_value(0u32);
//         assert!(m.recycles.read().unwrap().ranges.is_empty());
//         assert_eq!(4, m.update_sources.read().unwrap().len());
//         let k = m.update_k.load(Ordering::Relaxed);
//         // MAYBE_TODO: recycle "update_k"s instead of incrementing for each new source
//         assert_eq!(8, k);

//         log::info!("creating one more update source, immediately dropping it and two others");
//         let h8 = m.new_value(0u32);
//         drop(h8);
//         drop(h4);
//         drop(h6);
//         let _ = m.upkeep(());
//         assert_eq!(3, m.recycles.read().unwrap().ranges.len());
//         assert_eq!(2, m.update_sources.read().unwrap().len());
//         assert_eq!(9, m.update_k.load(Ordering::Relaxed));

//         drop(h7);
//         drop(h5);
//         let _ = m.upkeep(());
//         m.defrag();
//         assert_eq!(
//             1,
//             m.recycles.read().unwrap().ranges.len(),
//             "ranges: {:#?}",
//             m.recycles.read().unwrap().ranges
//         );
//     }
// }
