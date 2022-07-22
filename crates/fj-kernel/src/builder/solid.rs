use fj_math::Scalar;

use crate::{
    algorithms::TransformObject,
    objects::{Face, Solid, Surface},
};

/// API for building a [`Solid`]
pub struct SolidBuilder;

impl SolidBuilder {
    /// Create a cube from the length of its edges
    pub fn cube_from_edge_length(
        &self,
        edge_length: impl Into<Scalar>,
    ) -> Solid {
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

        let faces =
            planes.map(|plane| Face::build(plane).polygon_from_points(points));

        Solid::from_faces(faces)
    }
}
