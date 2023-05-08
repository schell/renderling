//! Types for managing resources.
use std::{
    marker::PhantomData,
    ops::Deref,
    sync::{
        atomic::AtomicUsize,
        Arc, RwLock, RwLockReadGuard, RwLockWriteGuard,
    },
};

use async_channel::{Sender, Receiver, unbounded};

/// An identifier.
#[derive(Ord)]
pub struct Id<T>(pub(crate) usize, PhantomData<T>);

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

impl<T> Deref for Id<T> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Id<T> {
    pub(crate) fn new(i: usize) -> Self {
        Id(i, PhantomData)
    }
}

pub(crate) struct BankOfIds<T> {
    next_id: Arc<AtomicUsize>,
    recycler: (Sender<Id<T>>, Receiver<Id<T>>),
}

impl<T> Default for BankOfIds<T> {
    fn default() -> Self {
        BankOfIds {
            next_id: Arc::new(AtomicUsize::new(0)),
            recycler: unbounded(),
        }
    }
}

impl<T> BankOfIds<T> {
    pub fn dequeue(&self) -> Id<T> {
        if let Ok(id) = self.recycler.1.try_recv() {
            id
        } else {
            Id(
                self.next_id
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                PhantomData,
            )
        }
    }

    pub fn recycle(&self, id: Id<T>) {
        let _ = self.recycler.0.send(id);
    }
}
