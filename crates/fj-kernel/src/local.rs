//! Infrastructure for types that have a local and a global form

/// A reference to an object, which includes a local form
///
/// This type is used by topological objects to reference other objects, while
/// also keeping track of a local representation of that object, which is often
/// more appropriate for various tasks.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct LocalForm<T, Canonical> {
    local: T,
    canonical: Canonical,
}

impl<T, Canonical> LocalForm<T, Canonical> {
    /// Construct a new instance of `LocalForm`
    ///
    /// It is the caller's responsibility to make sure that the local and
    /// canonical forms passed to this method actually match.
    pub fn new(local: T, canonical: Canonical) -> Self {
        Self { local, canonical }
    }

    /// Access the local form of the referenced object
    pub fn local(&self) -> &T {
        &self.local
    }

    /// Access the canonical form of the referenced object
    pub fn canonical(&self) -> &Canonical {
        &self.canonical
    }
}
