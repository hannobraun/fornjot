use std::{
    cmp::PartialEq,
    collections::{btree_set, BTreeSet},
};

use fj_interop::mesh::Color;
use fj_math::Winding;

use crate::{
    objects::{Cycle, Surface},
    storage::Handle,
};

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
/// [`HalfEdge`]: crate::objects::HalfEdge
/// [`Shell`]: crate::objects::Shell
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Face {
    surface: Handle<Surface>,
    exterior: Handle<Cycle>,
    interiors: Vec<Handle<Cycle>>,
    color: Option<Color>,
}

impl Face {
    /// Construct an instance of `Face`
    pub fn new(
        surface: Handle<Surface>,
        exterior: Handle<Cycle>,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
        color: Option<Color>,
    ) -> Self {
        let interiors = interiors.into_iter().collect();

        Self {
            surface,
            exterior,
            interiors,
            color,
        }
    }

    /// Access the surface of the face
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

    /// Access all cycles of the face (both exterior and interior)
    pub fn all_cycles(&self) -> impl Iterator<Item = &Handle<Cycle>> + '_ {
        [self.exterior()].into_iter().chain(self.interiors())
    }

    /// Access the color of the face
    pub fn color(&self) -> Option<Color> {
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
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Set<T: PartialEq + Ord> {
    inner: BTreeSet<Handle<T>>,
}

impl<T: PartialEq + Ord> Default for Set<T> {
    fn default() -> Self {
        Self {
            inner: BTreeSet::default(),
        }
    }
}

impl<T: PartialEq + Ord> Set<T> {
    /// Create an empty instance of `Set`
    pub fn new() -> Self {
        Self::default()
    }

    /// Find the given object
    pub fn find(&self, other: &Handle<T>) -> Option<Handle<T>> {
        for f in self {
            if f == other {
                return Some(f.clone());
            }
        }

        None
    }
}

impl<T: PartialEq + Ord> Extend<Handle<T>> for Set<T> {
    fn extend<I: IntoIterator<Item = Handle<T>>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

impl<T: PartialEq + Ord> FromIterator<Handle<T>> for Set<T> {
    fn from_iter<I: IntoIterator<Item = Handle<T>>>(iter: I) -> Self {
        let mut items = Self::new();
        items.extend(iter);
        items
    }
}

impl<T: PartialEq + Ord> IntoIterator for Set<T> {
    type Item = Handle<T>;
    type IntoIter = btree_set::IntoIter<Handle<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T: PartialEq + Ord> IntoIterator for &'a Set<T> {
    type Item = &'a Handle<T>;
    type IntoIter = btree_set::Iter<'a, Handle<T>>;

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
