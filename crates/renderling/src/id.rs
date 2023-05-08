//! Typed identifiers.
use std::{
    marker::PhantomData,
    //sync::{
    //    atomic::{AtomicUsize, AtomicU32},
    //    Arc,
    //},
};

//use async_channel::{Sender, Receiver, unbounded};

/// An identifier.
#[derive(Ord)]
#[repr(transparent)]
pub struct Id<T>(pub(crate) u32, PhantomData<T>);

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}

impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("Id<{}>", std::any::type_name::<T>()))
            .field(&self.0)
            .finish()
    }
}

impl<T> Id<T> {
    pub fn new(i: u32) -> Self {
        Id(i, PhantomData)
    }

    /// Convert this id into a usize for use as an index.
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

//pub(crate) struct BankOfIds<T> {
//    next_id: Arc<AtomicU32>,
//    recycler: (Sender<Id<T>>, Receiver<Id<T>>),
//}
//
//impl<T> Default for BankOfIds<T> {
//    fn default() -> Self {
//        BankOfIds {
//            next_id: Arc::new(AtomicU32::new(0)),
//            recycler: unbounded(),
//        }
//    }
//}
//
//impl<T> BankOfIds<T> {
//    pub fn dequeue(&self) -> Id<T> {
//        if let Ok(id) = self.recycler.1.try_recv() {
//            id
//        } else {
//            Id(
//                self.next_id
//                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
//                PhantomData,
//            )
//        }
//    }
//
//    pub fn recycle(&self, id: Id<T>) {
//        let _ = self.recycler.0.send(id);
//    }
//}
