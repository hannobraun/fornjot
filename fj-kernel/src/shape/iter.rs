use super::{handle::Handle, stores::Store};

/// An iterator over geometric or topological objects
///
/// Returned by various methods of the [`Shape`] API.
pub struct Iter<T> {
    elements: Vec<Handle<T>>,
}

impl<T> Iter<T> {
    pub(super) fn new(store: &Store<T>) -> Self {
        // The allocation here is unfortunate, but I think it's fine for now. If
        // this turns into a performance issue, it should be possible to avoid
        // it by adding methods to `Store`, that are geared at implementing
        // iterators.
        Self {
            elements: store.iter().collect(),
        }
    }

    /// Convert this iterator over handles into an iterator over values
    ///
    /// This is a convenience method, for cases where no `Handle` is needed.
    pub fn values(self) -> impl Iterator<Item = T>
    where
        T: Clone,
    {
        self.elements.into_iter().map(|handle| handle.get())
    }
}

impl<T> Iterator for Iter<T> {
    type Item = Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.pop()
    }
}
