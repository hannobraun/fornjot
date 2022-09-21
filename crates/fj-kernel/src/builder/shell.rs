use fj_math::Scalar;

use crate::{
    algorithms::transform::TransformObject,
    objects::{Face, Shell, Surface},
    stores::Stores,
};

/// API for building a [`Shell`]
///
/// Also see [`Shell::builder`].
pub struct ShellBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,
}

impl<'a> ShellBuilder<'a> {
    /// Create a cube from the length of its edges
    pub fn build_cube_from_edge_length(
        &self,
        edge_length: impl Into<Scalar>,
    ) -> Shell {
        // Let's define a short-hand for half the edge length. We're going to
        // need it a lot.
        let h = edge_length.into() / 2.;

        let points = [[-h, -h], [h, -h], [h, h], [-h, h]];

        const Z: Scalar = Scalar::ZERO;
        let planes = [
            Surface::xy_plane().translate([Z, Z, -h], self.stores), // bottom
            Surface::xy_plane().translate([Z, Z, h], self.stores),  // top
            Surface::xz_plane().translate([Z, -h, Z], self.stores), // front
            Surface::xz_plane().translate([Z, h, Z], self.stores),  // back
            Surface::yz_plane().translate([-h, Z, Z], self.stores), // left
            Surface::yz_plane().translate([h, Z, Z], self.stores),  // right
        ];

        let faces = planes.map(|plane| {
            Face::builder(self.stores, plane)
                .build_polygon_from_points(points)
                .into_face()
        });

        Shell::new().with_faces(faces)
    }
}
