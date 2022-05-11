mod delaunay;
mod polygon;
mod ray;

use fj_interop::{debug::DebugInfo, mesh::Mesh};
use fj_math::Point;

use crate::{shape::Shape, topology::Face};

use self::polygon::Polygon;

use super::{FaceApprox, Tolerance};

/// Triangulate a shape
pub fn triangulate(
    shape: Shape,
    tolerance: Tolerance,
    debug_info: &mut DebugInfo,
) -> Mesh<Point<3>> {
    let mut mesh = Mesh::new();

    for face in shape.faces() {
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
                        surface.point_to_surface_coords(vertex)
                    })
                    .collect();
                let face_as_polygon = Polygon::new(surface)
                    .with_exterior(approx.exterior.points.into_iter().map(
                        |point| {
                            // Can't panic, unless the approximation wrongfully
                            // generates points that are not in the surface.
                            surface.point_to_surface_coords(point).native()
                        },
                    ))
                    .with_interiors(approx.interiors.into_iter().map(
                        |interior| {
                            interior.points.into_iter().map(|point| {
                                // Can't panic, unless the approximation
                                // wrongfully generates points that are not in
                                // the surface.
                                surface.point_to_surface_coords(point).native()
                            })
                        },
                    ));

                let mut triangles = delaunay::triangulate(points);
                triangles.retain(|triangle| {
                    face_as_polygon.contains_triangle(
                        triangle.map(|point| point.native()),
                        debug_info,
                    )
                });

                for triangle in triangles {
                    let points = triangle.map(|point| point.canonical());
                    mesh.push_triangle(points, *color);
                }
            }
            Face::Triangles(triangles) => {
                for &(triangle, color) in triangles {
                    mesh.push_triangle(triangle.points(), color);
                }
            }
        }
    }

    mesh
}

#[cfg(test)]
mod tests {
    use fj_interop::{debug::DebugInfo, mesh::Mesh};
    use fj_math::{Point, Scalar};

    use crate::{
        algorithms::Tolerance, geometry::Surface, shape::Shape, topology::Face,
    };

    #[test]
    fn simple() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let a = [0., 0., 0.];
        let b = [2., 0., 0.];
        let c = [2., 2., 0.];
        let d = [0., 1., 0.];

        Face::builder(Surface::xy_plane(), &mut shape)
            .with_exterior_polygon([a, b, c, d])
            .build()?;

        let triangles = triangulate(shape);
        assert!(triangles.contains_triangle([a, b, d]));
        assert!(triangles.contains_triangle([b, c, d]));
        assert!(!triangles.contains_triangle([a, b, c]));
        assert!(!triangles.contains_triangle([a, c, d]));

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

        Face::builder(Surface::xy_plane(), &mut shape)
            .with_exterior_polygon([a, b, c, d])
            .with_interior_polygon([e, f, g, h])
            .build()?;

        let triangles = triangulate(shape);

        // Should contain some triangles from the polygon. Don't need to test
        // them all.
        assert!(triangles.contains_triangle([a, e, h]));
        assert!(triangles.contains_triangle([a, d, h]));

        // Shouldn't contain any possible triangle from the hole.
        assert!(!triangles.contains_triangle([e, f, g]));
        assert!(!triangles.contains_triangle([e, g, h]));
        assert!(!triangles.contains_triangle([e, f, h]));
        assert!(!triangles.contains_triangle([f, g, h]));

        Ok(())
    }

    fn triangulate(shape: Shape) -> Mesh<Point<3>> {
        let tolerance = Tolerance::from_scalar(Scalar::ONE).unwrap();

        let mut debug_info = DebugInfo::new();
        super::triangulate(shape, tolerance, &mut debug_info)
    }
}
