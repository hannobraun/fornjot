use std::collections::{btree_set, BTreeSet};

use fj_math::Winding;

use crate::{
    objects::{Region, Surface},
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
    region: Handle<Region>,
}

impl Face {
    /// Construct an instance of `Face`
    pub fn new(surface: Handle<Surface>, region: Handle<Region>) -> Self {
        Self { surface, region }
    }

    /// Access the surface of the face
    pub fn surface(&self) -> &Handle<Surface> {
        &self.surface
    }

    /// Access the region of the face
    pub fn region(&self) -> &Handle<Region> {
        &self.region
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
        match self.region.exterior().winding() {
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
}

impl Extend<Handle<Face>> for FaceSet {
    fn extend<T: IntoIterator<Item = Handle<Face>>>(&mut self, iter: T) {
        self.inner.extend(iter);
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
