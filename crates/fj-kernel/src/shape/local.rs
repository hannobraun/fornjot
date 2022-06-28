use std::hash::{Hash, Hasher};

use super::Object;

/// A reference to an object, which includes a local form
///
/// This type is used by topological objects to reference other objects, while
/// also keeping track of a local representation of that object, which is often
/// more appropriate for various tasks.
///
/// # Equality
///
/// Since `LocalForm` is used by topological objects, its equality is defined in
/// terms that are useful to those objects. Two instances of `LocalForm` are
/// equal, if both the local and the canonical forms are equal. The equality of
/// the handle that refers to the canonical form is disregarded.
#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub struct LocalForm<Local, Canonical: Object> {
    local: Local,
    canonical: Canonical,
}

impl<Local, Canonical: Object> LocalForm<Local, Canonical> {
    /// Construct a new instance of `LocalForm`
    ///
    /// It is the caller's responsibility to make sure that the local and
    /// canonical forms passed to this method actually match.
    pub fn new(local: Local, canonical: Canonical) -> Self {
        Self { local, canonical }
    }

    /// Access the local form of the referenced object
    pub fn local(&self) -> &Local {
        &self.local
    }

    /// Access the canonical form of the referenced object
    pub fn canonical(&self) -> Canonical {
        self.canonical.clone()
    }
}

impl<Canonical: Object> LocalForm<Canonical, Canonical> {
    /// Construct a new instance of `LocalForm` without a local form
    ///
    /// It's possible that an object's local and canonical forms are the same.
    /// This is a convenience constructor that constructs a `LocalForm` instance
    /// for such a situation.
    pub fn canonical_only(canonical: Canonical) -> Self {
        Self::new(canonical.clone(), canonical)
    }
}

impl<Local, Canonical: Object> PartialEq for LocalForm<Local, Canonical>
where
    Local: PartialEq,
    Canonical: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.local == other.local && self.canonical == other.canonical
    }
}

impl<Local, Canonical: Object> Hash for LocalForm<Local, Canonical>
where
    Local: Hash,
    Canonical: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.local.hash(state);
        self.canonical.hash(state);
    }
}
