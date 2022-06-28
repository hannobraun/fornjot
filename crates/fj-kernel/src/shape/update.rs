use super::{stores::Stores, Object};

/// API to update a `Shape`
///
/// See [`Shape::update`].
pub struct Update<'r> {
    stores: &'r mut Stores,
}

impl<'r> Update<'r> {
    /// Update all objects of a specific type
    pub fn update_all<T: Object>(self, f: impl FnMut(&mut T)) -> Self {
        self.stores.get::<T>().update(f);
        self
    }
}
