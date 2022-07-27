use std::collections::BTreeSet;

use crate::builder::SolidBuilder;

use super::Face;

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
    faces: BTreeSet<Face>,
}

impl Solid {
    /// Build a solid using [`SolidBuilder`]
    pub fn build() -> SolidBuilder {
        SolidBuilder
    }

    /// Construct an empty instance of `Solid`
    pub fn new() -> Self {
        Self {
            faces: BTreeSet::new(),
        }
    }

    /// Add faces to the solid
    ///
    /// Consumes the solid and returns the updated instance.
    pub fn with_faces(
        mut self,
        faces: impl IntoIterator<Item = impl Into<Face>>,
    ) -> Self {
        let faces = faces.into_iter().map(Into::into);
        self.faces.extend(faces);
        self
    }

    /// Access the solid's faces
    pub fn faces(&self) -> impl Iterator<Item = &Face> {
        self.faces.iter()
    }

    /// Convert the solid into a list of faces
    pub fn into_faces(self) -> BTreeSet<Face> {
        self.faces
    }
}

impl Default for Solid {
    fn default() -> Self {
        Self::new()
    }
}
