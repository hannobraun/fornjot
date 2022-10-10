use std::collections::BTreeSet;

use crate::builder::SolidBuilder;

use super::{Face, Objects, Shell};

/// A 3-dimensional shape
///
/// # Implementation Note
///
/// The shells that form the boundaries of the solid must not intersect. This is
/// not currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Solid {
    shells: BTreeSet<Shell>,
}

impl Solid {
    /// Build a `Solid` using [`SolidBuilder`]
    pub fn builder(stores: &Objects) -> SolidBuilder {
        SolidBuilder { stores }
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

    /// Find the given face in this solid
    pub fn find_face(&self, face: &Face) -> Option<Face> {
        for shell in self.shells() {
            if let Some(face) = shell.find_face(face) {
                return Some(face);
            }
        }

        None
    }
}

impl Default for Solid {
    fn default() -> Self {
        Self::new()
    }
}
