//! A collection of contiguous things that get recycled.
use crate::resources::{BankOfIds, Id};

/// A collection of contiguous things that get recycled.
pub struct Bank<T> {
    ids: BankOfIds<T>,
    things: Vec<Option<T>>,
}

impl<T> Default for Bank<T> {
    fn default() -> Self {
        Self {
            ids: Default::default(),
            things: Default::default(),
        }
    }
}

impl<T> Bank<T> {
    pub fn get(&self, id: &Id<T>) -> Option<&T> {
        let may_thing = self.things.get(id.0)?;
        may_thing.as_ref()
    }

    pub fn get_mut(&mut self, id: &Id<T>) -> Option<&mut T> {
        let may_thing = self.things.get_mut(id.0)?;
        may_thing.as_mut()
    }

    pub fn destroy(&mut self, id: Id<T>) {
        log::trace!(
            "destroying and recycling {} {:?}",
            std::any::type_name::<T>(),
            id
        );

        self.things[id.0] = None;
        self.ids.recycle(id);
    }

    fn next_id(&mut self) -> Id<T> {
        let id = self.ids.dequeue();
        if self.things.len() < id.0 + 1 {
            self.things.resize_with(id.0 + 1, || None);
        }
        id
    }

    pub fn iter(&self) -> impl Iterator<Item = Option<&T>> + '_ {
        self.things.iter().map(Option::as_ref)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = Option<&mut T>> + '_ {
        self.things.iter_mut().map(Option::as_mut)
    }

    /// Insert a new `T` using a function that receives the storage `Id`.
    pub fn insert_with(&mut self, f: impl FnOnce(Id<T>) -> T) -> Id<T> {
        let id = self.next_id();
        let t = f(id.clone());
        self.things[id.0] = Some(t);
        id
    }
}
