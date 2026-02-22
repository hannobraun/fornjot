//! # Geometric utility code based on triangle meshes

use crate::{
    geometry::{Geometry, traits::GenTriMesh},
    interop::Tolerance,
    math::{Point, Vector},
};

/// # A triangle mesh, the uniform intermediate representation of a surface
///
/// ## Implementation
///
/// This struct is currently a placeholder, while the transition to the new
/// geometry system is ongoing.
pub struct TriMesh {}

impl TriMesh {
    /// # Construct an empty triangle mesh
    pub fn empty() -> Self {
        Self {}
    }
}

/// # Convert a point in surface coordinates to global coordinates
pub fn convert_point_surface_to_global(
    surface: &dyn GenTriMesh,
    point: impl Into<Point<2>>,
    tolerance: impl Into<Tolerance>,
    geometry: &Geometry,
) -> Point<3> {
    let (triangle, barycentric_coords) =
        surface.triangle_at(point.into(), tolerance.into(), geometry);
    triangle.point_from_barycentric_coords(barycentric_coords)
}

/// # Convert a vector in surface coordinates to global coordinates
pub fn convert_vector_surface_to_global(
    surface: &dyn GenTriMesh,
    vector: impl Into<Vector<2>>,
    tolerance: impl Into<Tolerance>,
    geometry: &Geometry,
) -> Vector<3> {
    let vector = vector.into();
    let point = convert_point_surface_to_global(
        surface,
        Point { coords: vector },
        tolerance,
        geometry,
    );
    point - surface.origin(geometry)
}
