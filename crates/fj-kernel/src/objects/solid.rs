use std::collections::BTreeSet;

use fj_math::Scalar;

use crate::algorithms::TransformObject;

use super::{Face, Surface};

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
    /// Construct a solid from faces
    pub fn from_faces(faces: impl IntoIterator<Item = Face>) -> Self {
        let faces = faces.into_iter().collect();
        Self { faces }
    }

    /// Create a cube from the length of its edges
    pub fn cube_from_edge_length(edge_length: impl Into<Scalar>) -> Self {
        // Let's define a short-hand for half the edge length. We're going to
        // need it a lot.
        let h = edge_length.into() / 2.;

        let points = [[-h, -h], [h, -h], [h, h], [-h, h]];

        const Z: Scalar = Scalar::ZERO;
        let planes = [
            Surface::xy_plane().translate([Z, Z, -h]), // bottom
            Surface::xy_plane().translate([Z, Z, h]),  // top
            Surface::xz_plane().translate([Z, -h, Z]), // front
            Surface::xz_plane().translate([Z, h, Z]),  // back
            Surface::yz_plane().translate([-h, Z, Z]), // left
            Surface::yz_plane().translate([h, Z, Z]),  // right
        ];

        let faces = planes.map(|plane| {
            Face::builder(plane).with_exterior_polygon(points).build()
        });

        Solid::from_faces(faces)
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
