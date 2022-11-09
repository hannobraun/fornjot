use std::collections::{btree_set, BTreeSet};

use fj_interop::mesh::Color;
use fj_math::Winding;

use crate::storage::Handle;

use super::{Cycle, Surface};

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
    exterior: Handle<Cycle>,
    interiors: Vec<Handle<Cycle>>,
    color: Color,
}

impl Face {
    /// Construct a new instance of `Face`
    pub fn new(
        exterior: Handle<Cycle>,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
        color: Color,
    ) -> Self {
        let interiors = interiors.into_iter().collect();

        Self {
            exterior,
            interiors,
            color,
        }
    }

    /// Access this face's surface
    pub fn surface(&self) -> &Handle<Surface> {
        self.exterior().surface()
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
pub struct FaceSet {
    inner: BTreeSet<Handle<Face>>,
}

impl FaceSet {
    /// Create an empty instance of `Faces`
    pub fn new() -> Self {
        Self::default()
    }

    /// Find the given face
    pub fn find(&self, face: &Handle<Face>) -> Option<Handle<Face>> {
        for f in self {
            if f == face {
                return Some(f.clone());
            }
        }

        None
    }
}

impl Extend<Handle<Face>> for FaceSet {
    fn extend<T: IntoIterator<Item = Handle<Face>>>(&mut self, iter: T) {
        self.inner.extend(iter)
    }
}

impl FromIterator<Handle<Face>> for FaceSet {
    fn from_iter<T: IntoIterator<Item = Handle<Face>>>(iter: T) -> Self {
        let mut faces = Self::new();
        faces.extend(iter);
        faces
    }
}

impl IntoIterator for FaceSet {
    type Item = Handle<Face>;
    type IntoIter = btree_set::IntoIter<Handle<Face>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a FaceSet {
    type Item = &'a Handle<Face>;
    type IntoIter = btree_set::Iter<'a, Handle<Face>>;

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
