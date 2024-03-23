use crate::{
    storage::Handle,
    topology::{ObjectSet, Shell},
};

/// A 3-dimensional shape, built from [`Shell`]s. Many Solids will contains only
/// one shell, but if the Solid contains cavities they will be represented by a
/// shell each, as well as a shell for the outside.
///
/// # Implementation Note
///
/// The shells that form the boundaries of the solid must not intersect. This is
/// not currently validated.
#[derive(Clone, Debug)]
pub struct Solid {
    shells: ObjectSet<Shell>,
}

impl Solid {
    /// Construct an empty instance of `Solid`
    pub fn new(shells: impl IntoIterator<Item = Handle<Shell>>) -> Self {
        Self {
            shells: shells.into_iter().collect(),
        }
    }

    /// Access the solid's shells
    pub fn shells(&self) -> &ObjectSet<Shell> {
        &self.shells
    }
}
