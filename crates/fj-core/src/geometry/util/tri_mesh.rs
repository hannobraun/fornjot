//! # Geometric utility code based on triangle meshes

use fj_math::{Point, Vector};

use crate::geometry::{traits::GenTriMesh, Tolerance};

/// # Convert a point in surface coordinates to global coordinates
pub fn convert_point_surface_to_global(
    surface: &dyn GenTriMesh,
    point: impl Into<Point<2>>,
    tolerance: impl Into<Tolerance>,
) -> Point<3> {
    let (triangle, barycentric_coords) =
        surface.triangle_at(point.into(), tolerance.into());
    triangle.point_from_barycentric_coords(barycentric_coords)
}

/// # Convert a vector in surface coordinates to global coordinates
pub fn convert_vector_surface_to_global(
    surface: &dyn GenTriMesh,
    vector: impl Into<Vector<2>>,
    tolerance: impl Into<Tolerance>,
) -> Vector<3> {
    let vector = vector.into();
    let point = convert_point_surface_to_global(
        surface,
        Point { coords: vector },
        tolerance,
    );
    point - surface.origin()
}

#[cfg(test)]
mod tests {
    use fj_math::{Line, Point, Vector};
    use pretty_assertions::assert_eq;

    use crate::geometry::{
        util::tri_mesh::{
            convert_point_surface_to_global, convert_vector_surface_to_global,
        },
        Path, SweptCurve, Tolerance,
    };

    #[test]
    fn point_from_surface_coords() {
        let surface = SweptCurve {
            u: Path::Line(Line::from_origin_and_direction(
                Point::from([1., 1., 1.]),
                Vector::from([0., 2., 0.]),
            )),
            v: Vector::from([0., 0., 2.]),
        };

        // Value doesn't matter; we're dealing with a plane.
        let tolerance = Tolerance::from_scalar(1.).unwrap();

        assert_eq!(
            convert_point_surface_to_global(&surface, [2., 4.], tolerance),
            Point::from([1., 5., 9.]),
        );
    }

    #[test]
    fn vector_from_surface_coords() {
        let surface = SweptCurve {
            u: Path::Line(Line::from_origin_and_direction(
                Point::from([1., 0., 0.]),
                Vector::from([0., 2., 0.]),
            )),
            v: Vector::from([0., 0., 2.]),
        };

        // Value doesn't matter; we're dealing with a plane.
        let tolerance = Tolerance::from_scalar(1.).unwrap();

        assert_eq!(
            convert_vector_surface_to_global(&surface, [2., 4.], tolerance),
            Vector::from([0., 4., 8.]),
        );
    }
}
