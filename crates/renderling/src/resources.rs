//! Types for managing resources.
use std::{
    ops::Deref,
    sync::{
        atomic::AtomicUsize,
        mpsc::{channel, Receiver, Sender},
        Arc, RwLock, RwLockReadGuard, RwLockWriteGuard,
    },
};

/// An identifier.
// TODO: Add a type variable for the type it identifies.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Id(pub(crate) usize);

impl Deref for Id {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct BankOfIds {
    next_id: Arc<AtomicUsize>,
    recycler: (Sender<Id>, Receiver<Id>),
}

impl Default for BankOfIds {
    fn default() -> Self {
        BankOfIds {
            next_id: Arc::new(AtomicUsize::new(0)),
            recycler: channel(),
        }
    }
}

impl BankOfIds {
    pub fn dequeue(&self) -> Id {
        if let Ok(id) = self.recycler.1.try_recv() {
            id
        } else {
            Id(self
                .next_id
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed))
        }
    }

    pub fn recycle(&self, id: Id) {
        let _ = self.recycler.0.send(id);
    }
}

#[derive(Default)]
pub(crate) struct Shared<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Shared<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.inner.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.inner.write().unwrap()
    }

    pub fn count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }
}
