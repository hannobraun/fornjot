use std::collections::BTreeSet;

use crate::{builder::SolidBuilder, storage::Handle};

use super::{Face, Objects, Shell};

/// A 3-dimensional shape
///
/// # Implementation Note
///
/// The shells that form the boundaries of the solid must not intersect. This is
/// not currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Solid {
    shells: BTreeSet<Handle<Shell>>,
}

impl Solid {
    /// Build a `Solid` using [`SolidBuilder`]
    pub fn builder(objects: &Objects) -> SolidBuilder {
        SolidBuilder {
            objects,
            shells: BTreeSet::new(),
        }
    }

    /// Construct an empty instance of `Solid`
    pub fn new(
        shells: impl IntoIterator<Item = Handle<Shell>>,
        objects: &Objects,
    ) -> Handle<Self> {
        objects.solids.insert(Self {
            shells: shells.into_iter().collect(),
        })
    }

    /// Access the solid's shells
    pub fn shells(&self) -> impl Iterator<Item = &Handle<Shell>> {
        self.shells.iter()
    }

    /// Find the given face in this solid
    pub fn find_face(&self, face: &Handle<Face>) -> Option<Handle<Face>> {
        for shell in self.shells() {
            if let Some(face) = shell.find_face(face) {
                return Some(face);
            }
        }

        None
    }
}
