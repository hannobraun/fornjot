use std::collections::{btree_set, BTreeSet};

use fj_interop::mesh::Color;
use fj_math::Winding;

use crate::{builder::FaceBuilder, stores::Stores};

use super::{Cycle, Surface};

/// A collection of faces
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Faces {
    inner: BTreeSet<Face>,
}

impl Faces {
    /// Create an empty instance of `Faces`
    pub fn new() -> Self {
        Self::default()
    }

    /// Find the given face
    pub fn find(&self, face: &Face) -> Option<Face> {
        for f in self {
            if f == face {
                return Some(f.clone());
            }
        }

        None
    }
}

impl Extend<Face> for Faces {
    fn extend<T: IntoIterator<Item = Face>>(&mut self, iter: T) {
        self.inner.extend(iter)
    }
}

impl IntoIterator for Faces {
    type Item = Face;
    type IntoIter = btree_set::IntoIter<Face>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Faces {
    type Item = &'a Face;
    type IntoIter = btree_set::Iter<'a, Face>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

/// A face of a shape
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Face {
    surface: Surface,
    exterior: Cycle,
    interiors: Vec<Cycle>,
    color: Color,
}

impl Face {
    /// Build a `Face` using [`FaceBuilder`]
    pub fn builder(stores: &Stores, surface: Surface) -> FaceBuilder {
        FaceBuilder {
            stores,
            surface,
            exterior: None,
            interiors: Vec::new(),
        }
    }

    /// Construct a new instance of `Face`
    ///
    /// Creates the face with no interiors and the default color. This can be
    /// overridden using the `with_` methods.
    pub fn new(surface: Surface, exterior: Cycle) -> Self {
        Self {
            surface,
            exterior,
            interiors: Vec::new(),
            color: Color::default(),
        }
    }

    /// Add interior cycles to the face
    ///
    /// Consumes the face and returns the updated instance.
    ///
    /// # Panics
    ///
    /// Panics, if the added cycles are not defined in the face's surface.
    pub fn with_interiors(
        mut self,
        interiors: impl IntoIterator<Item = Cycle>,
    ) -> Self {
        for cycle in interiors.into_iter() {
            assert_eq!(
                self.surface(),
                cycle.surface(),
                "Cycles that bound a face must be in face's surface"
            );

            self.interiors.push(cycle);
        }

        self
    }

    /// Update the color of the face
    ///
    /// Consumes the face and returns the updated instance.
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Access this face's surface
    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    /// Access the cycles that bound the face on the outside
    pub fn exterior(&self) -> &Cycle {
        &self.exterior
    }

    /// Access the cycles that bound the face on the inside
    ///
    /// Each of these cycles defines a hole in the face.
    pub fn interiors(&self) -> impl Iterator<Item = &Cycle> + '_ {
        self.interiors.iter()
    }

    /// Access all cycles of this face
    pub fn all_cycles(&self) -> impl Iterator<Item = &Cycle> + '_ {
        [self.exterior()].into_iter().chain(self.interiors())
    }

    /// Access the color of the face
    pub fn color(&self) -> Color {
        self.color
    }

    /// Determine handed-ness of the face's front-side coordinate system
    ///
    /// A face is defined on a surface, which has a coordinate system. Since
    /// surfaces aren't considered to have an orientation, their coordinate
    /// system can be considered to be left-handed or right-handed, depending on
    /// which side of the surface you're looking at.
    ///
    /// Faces *do* have an orientation, meaning they have definite front and
    /// back sides. The front side is the side, where the face's exterior cycle
    /// is wound clockwise.
    pub fn coord_handedness(&self) -> Handedness {
        match self.exterior().winding() {
            Winding::Ccw => Handedness::RightHanded,
            Winding::Cw => Handedness::LeftHanded,
        }
    }
}

/// The handedness of a face's coordinate system
///
/// See [`Face::coord_handedness`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Handedness {
    /// The face's coordinate system is left-handed
    LeftHanded,

    /// The face's coordinate system is right-handed
    RightHanded,
}
