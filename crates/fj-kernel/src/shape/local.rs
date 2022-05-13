use std::hash::{Hash, Hasher};

use super::{Handle, Object};

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
    canonical: Handle<Canonical>,
}

impl<Local, Canonical: Object> LocalForm<Local, Canonical> {
    /// Construct a new instance of `LocalForm`
    pub fn new(local: Local, canonical: Handle<Canonical>) -> Self {
        Self { local, canonical }
    }

    /// Access the local form of the referenced object
    pub fn local(&self) -> &Local {
        &self.local
    }

    /// Access the canonical form of the referenced object
    pub fn canonical(&self) -> &Handle<Canonical> {
        &self.canonical
    }
}

impl<Local, Canonical: Object> PartialEq for LocalForm<Local, Canonical>
where
    Local: PartialEq,
    Canonical: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.local == other.local
            && self.canonical.get() == other.canonical.get()
    }
}

impl<Local, Canonical: Object> Hash for LocalForm<Local, Canonical>
where
    Local: Hash,
    Canonical: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.local.hash(state);
        self.canonical.get().hash(state);
    }
}
