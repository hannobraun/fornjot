use std::collections::{btree_set, BTreeSet};

use fj_interop::mesh::Color;
use fj_math::Winding;

use crate::{builder::FaceBuilder, storage::Handle};

use super::{Cycle, Objects, Surface};

/// A face of a shape
///
/// A `Face` is a bounded area of a [`Surface`], the [`Surface`] itself being an
/// infinite 2-dimensional object in 3D space. `Face`s are bound by one exterior
/// cycle, which defines the outer boundary, and an arbitrary number of interior
/// cycles (i.e. holes).
///
/// `Face` has a defined orientation, a front and a back side. When faces are
/// combined into [`Shell`]s, the face orientation defines what is inside and
/// outside of the shell. This stands in contrast to [`Surface`], which has no
/// defined orientation.
///
/// You can look at a `Face` from two directions: front and back. The winding of
/// the exterior cycle will be clockwise or counter-clockwise, depending on your
/// perspective. The front side of the face, is the side where from which the
/// exterior cycle appear counter-clockwise.
///
/// Interior cycles must have the opposite winding of the exterior cycle,
/// meaning on the front side of the face, they must appear clockwise. This
/// means that all [`HalfEdge`]s that bound a `Face` have the interior of the
/// face on their left side (on the face's front side).
///
/// [`HalfEdge`]: super::HalfEdge
/// [`Shell`]: super::Shell
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Face {
    surface: Handle<Surface>,
    exterior: Handle<Cycle>,
    interiors: Vec<Handle<Cycle>>,
    color: Color,
}

impl Face {
    /// Build a `Face` using [`FaceBuilder`]
    pub fn builder(objects: &Objects) -> FaceBuilder {
        FaceBuilder {
            objects,
            surface: None,
            exterior: None,
            interiors: Vec::new(),
        }
    }

    /// Construct a new instance of `Face`
    ///
    /// Creates the face with no interiors and the default color. This can be
    /// overridden using the `with_` methods.
    pub fn new(exterior: Handle<Cycle>) -> Self {
        let surface = exterior.surface().clone();
        let interiors = Vec::new();
        let color = Color::default();

        Self {
            surface,
            exterior,
            interiors,
            color,
        }
    }

    /// Add interior cycles to the face
    ///
    /// Consumes the face and returns the updated instance.
    ///
    /// # Panics
    ///
    /// Panics, if the added cycles are not defined in the face's surface.
    ///
    /// Panics, if the winding of the interior cycles is not opposite that of
    /// the exterior cycle.
    pub fn with_interiors(
        mut self,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
    ) -> Self {
        for interior in interiors.into_iter() {
            assert_eq!(
                self.surface().id(),
                interior.surface().id(),
                "Cycles that bound a face must be in face's surface"
            );
            assert_ne!(
                self.exterior().winding(),
                interior.winding(),
                "Interior cycles must have opposite winding of exterior cycle"
            );

            self.interiors.push(interior);
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
    pub fn surface(&self) -> &Handle<Surface> {
        &self.surface
    }

    /// Access the cycle that bounds the face on the outside
    pub fn exterior(&self) -> &Handle<Cycle> {
        &self.exterior
    }

    /// Access the cycles that bound the face on the inside
    ///
    /// Each of these cycles defines a hole in the face.
    pub fn interiors(&self) -> impl Iterator<Item = &Handle<Cycle>> + '_ {
        self.interiors.iter()
    }

    /// Access all cycles of this face
    pub fn all_cycles(&self) -> impl Iterator<Item = &Handle<Cycle>> + '_ {
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
    /// is wound counter-clockwise.
    pub fn coord_handedness(&self) -> Handedness {
        match self.exterior().winding() {
            Winding::Ccw => Handedness::RightHanded,
            Winding::Cw => Handedness::LeftHanded,
        }
    }
}

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
