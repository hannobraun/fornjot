mod polygon;

use fj_debug::DebugInfo;
use fj_math::{Scalar, Triangle};
use parry2d_f64::utils::point_in_triangle::{corner_direction, Orientation};
use spade::HasPosition;

use crate::{geometry, shape::Shape, topology::Face};

use self::polygon::Polygon;

use super::FaceApprox;

/// Triangulate a shape
pub fn triangulate(
    mut shape: Shape,
    tolerance: Scalar,
    out: &mut Vec<Triangle<3>>,
    debug_info: &mut DebugInfo,
) {
    for face in shape.topology().faces() {
        let face = face.get();
        match &face {
            Face::Face { surface, color, .. } => {
                let surface = surface.get();
                let approx = FaceApprox::new(&face, tolerance);

                let points: Vec<_> = approx
                    .points
                    .into_iter()
                    .map(|vertex| {
                        // Can't panic, unless the approximation wrongfully
                        // generates points that are not in the surface.
                        surface.point_model_to_surface(vertex)
                    })
                    .collect();
                let face_as_polygon = Polygon::new(surface)
                    .with_exterior(approx.exterior.points)
                    .with_interiors(
                        approx
                            .interiors
                            .into_iter()
                            .map(|interior| interior.points),
                    );

                let mut triangles = delaunay(points);
                triangles.retain(|triangle| {
                    face_as_polygon.contains_triangle(
                        &triangle.map(|point| point.native()),
                        debug_info,
                    )
                });

                out.extend(triangles.into_iter().map(|triangle| {
                    let [a, b, c] = triangle.map(|point| point.canonical());
                    let mut t = Triangle::from([a, b, c]);
                    t.set_color(*color);
                    t
                }));
            }
            Face::Triangles(triangles) => out.extend(triangles),
        }
    }
}

/// Create a Delaunay triangulation of all points
fn delaunay(points: Vec<geometry::Point<2>>) -> Vec<[geometry::Point<2>; 3]> {
    use spade::Triangulation as _;

    let triangulation = spade::DelaunayTriangulation::<_>::bulk_load(points)
        .expect("Inserted invalid values into triangulation");

    let mut triangles = Vec::new();
    for triangle in triangulation.inner_faces() {
        let [v0, v1, v2] = triangle.vertices().map(|vertex| *vertex.data());
        let orientation = corner_direction(
            &v0.native().to_na(),
            &v1.native().to_na(),
            &v2.native().to_na(),
        );

        let triangle = match orientation {
            Orientation::Ccw => [v0, v1, v2],
            Orientation::Cw => [v0, v2, v1],
            Orientation::None => {
                panic!(
                    "Triangle returned from triangulation isn't actually a \
                    triangle"
                );
            }
        };

        triangles.push(triangle);
    }

    triangles
}

// Enables the use of `geometry::Point` in the triangulation.
impl HasPosition for geometry::Point<2> {
    type Scalar = Scalar;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        spade::Point2 {
            x: self.native().u,
            y: self.native().v,
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_debug::DebugInfo;
    use fj_math::{Scalar, Triangle};

    use crate::{geometry::Surface, shape::Shape, topology::Face};

    #[test]
    fn simple() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let a = [0., 0., 0.];
        let b = [2., 0., 0.];
        let c = [2., 2., 0.];
        let d = [0., 1., 0.];

        Face::builder(Surface::x_y_plane(), &mut shape)
            .with_exterior_polygon([a, b, c, d])
            .build()?;

        let triangles = triangulate(shape);
        assert!(triangles.contains([a, b, d]));
        assert!(triangles.contains([b, c, d]));
        assert!(!triangles.contains([a, b, c]));
        assert!(!triangles.contains([a, c, d]));

        Ok(())
    }

    #[test]
    fn simple_hole() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let a = [0., 0., 0.];
        let b = [4., 0., 0.];
        let c = [4., 4., 0.];
        let d = [0., 4., 0.];

        let e = [1., 1., 0.];
        let f = [3., 1., 0.];
        let g = [3., 3., 0.];
        let h = [1., 2., 0.];

        Face::builder(Surface::x_y_plane(), &mut shape)
            .with_exterior_polygon([a, b, c, d])
            .with_interior_polygon([e, f, g, h])
            .build()?;

        let triangles = triangulate(shape);

        // Should contain some triangles from the polygon. Don't need to test
        // them all.
        assert!(triangles.contains([a, e, h]));
        assert!(triangles.contains([a, d, h]));

        // Shouldn't contain any possible triangle from the hole.
        assert!(!triangles.contains([e, f, g]));
        assert!(!triangles.contains([e, g, h]));
        assert!(!triangles.contains([e, f, h]));
        assert!(!triangles.contains([f, g, h]));

        Ok(())
    }

    fn triangulate(shape: Shape) -> Triangles {
        let tolerance = Scalar::ONE;

        let mut triangles = Vec::new();
        let mut debug_info = DebugInfo::new();

        super::triangulate(shape, tolerance, &mut triangles, &mut debug_info);

        for triangle in &mut triangles {
            *triangle = triangle.normalize();
        }

        Triangles(triangles)
    }

    struct Triangles(Vec<Triangle<3>>);

    impl Triangles {
        fn contains(&self, triangle: impl Into<Triangle<3>>) -> bool {
            let triangle = triangle.into().normalize();
            self.0.contains(&triangle)
        }
    }
}
