use std::collections::BTreeSet;

use crate::builder::SolidBuilder;

use super::Shell;

/// A 3-dimensional shape
///
/// # Implementation Note
///
/// The faces that make up the solid must form a closed shape. This is not
/// currently validated.
///
/// In fact, solids could be made up of several closed shells. One outer shell,
/// and multiple inner ones (cavities within the solid). There should probably
/// a separate `Shell` object that is a collection of faces, and validates that
/// those faces form a closed shape. `Solid` should be a collection of such
/// `Shell`s, and validate that those `Shell`s don't intersect.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Solid {
    shells: BTreeSet<Shell>,
}

impl Solid {
    /// Build a solid using [`SolidBuilder`]
    pub fn build() -> SolidBuilder {
        SolidBuilder
    }

    /// Construct an empty instance of `Solid`
    pub fn new() -> Self {
        Self {
            shells: BTreeSet::new(),
        }
    }

    /// Add shells to the solid
    ///
    /// Consumes the solid and returns the updated instance.
    pub fn with_shells(
        mut self,
        shells: impl IntoIterator<Item = impl Into<Shell>>,
    ) -> Self {
        let shells = shells.into_iter().map(Into::into);
        self.shells.extend(shells);
        self
    }

    /// Access the solid's shells
    pub fn shells(&self) -> impl Iterator<Item = &Shell> {
        self.shells.iter()
    }

    /// Convert the solid into a list of shells
    pub fn into_shells(self) -> impl Iterator<Item = Shell> {
        self.shells.into_iter()
    }
}

impl Default for Solid {
    fn default() -> Self {
        Self::new()
    }
}
