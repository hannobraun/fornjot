mod delaunay;
mod polygon;
mod ray;

use fj_interop::{debug::DebugInfo, mesh::Mesh};
use fj_math::Point;

use crate::objects::Face;

use self::polygon::Polygon;

use super::{FaceApprox, Tolerance};

/// Triangulate a shape
pub fn triangulate(
    faces: Vec<Face>,
    tolerance: Tolerance,
    debug_info: &mut DebugInfo,
) -> Mesh<Point<3>> {
    let mut mesh = Mesh::new();

    for face in faces {
        match &face {
            Face::Face(brep) => {
                let surface = brep.surface.get();
                let approx = FaceApprox::new(&face, tolerance);

                let points: Vec<_> = approx
                    .points
                    .into_iter()
                    .map(|vertex| {
                        // Can't panic, unless the approximation wrongfully
                        // generates points that are not in the surface.
                        surface.point_to_surface_coords(vertex.canonical())
                    })
                    .collect();
                let face_as_polygon = Polygon::new(surface)
                    .with_exterior(approx.exterior.points.into_iter().map(
                        |point| {
                            // Can't panic, unless the approximation wrongfully
                            // generates points that are not in the surface.
                            surface
                                .point_to_surface_coords(point.canonical())
                                .local()
                        },
                    ))
                    .with_interiors(approx.interiors.into_iter().map(
                        |interior| {
                            interior.points.into_iter().map(|point| {
                                // Can't panic, unless the approximation
                                // wrongfully generates points that are not in
                                // the surface.
                                surface
                                    .point_to_surface_coords(point.canonical())
                                    .local()
                            })
                        },
                    ));

                let mut triangles = delaunay::triangulate(points);
                triangles.retain(|triangle| {
                    face_as_polygon.contains_triangle(
                        triangle.map(|point| point.local()),
                        debug_info,
                    )
                });

                for triangle in triangles {
                    let points = triangle.map(|point| point.canonical());
                    mesh.push_triangle(points, brep.color);
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
        algorithms::Tolerance,
        objects::{Face, Surface},
        shape::Shape,
    };

    #[test]
    fn simple() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let a = [0., 0.];
        let b = [2., 0.];
        let c = [2., 2.];
        let d = [0., 1.];

        let face = Face::builder(Surface::xy_plane(), &mut shape)
            .with_exterior_polygon([a, b, c, d])
            .build()
            .get();

        let a = Point::from(a).to_xyz();
        let b = Point::from(b).to_xyz();
        let c = Point::from(c).to_xyz();
        let d = Point::from(d).to_xyz();

        let triangles = triangulate(face)?;

        assert!(triangles.contains_triangle([a, b, d]));
        assert!(triangles.contains_triangle([b, c, d]));
        assert!(!triangles.contains_triangle([a, b, c]));
        assert!(!triangles.contains_triangle([a, c, d]));

        Ok(())
    }

    #[test]
    fn simple_hole() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let a = [0., 0.];
        let b = [4., 0.];
        let c = [4., 4.];
        let d = [0., 4.];

        let e = [1., 1.];
        let f = [3., 1.];
        let g = [3., 3.];
        let h = [1., 2.];

        let face = Face::builder(Surface::xy_plane(), &mut shape)
            .with_exterior_polygon([a, b, c, d])
            .with_interior_polygon([e, f, g, h])
            .build()
            .get();

        let triangles = triangulate(face)?;

        let a = Point::from(a).to_xyz();
        let d = Point::from(d).to_xyz();
        let e = Point::from(e).to_xyz();
        let f = Point::from(f).to_xyz();
        let g = Point::from(g).to_xyz();
        let h = Point::from(h).to_xyz();

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

    fn triangulate(face: Face) -> anyhow::Result<Mesh<Point<3>>> {
        let tolerance = Tolerance::from_scalar(Scalar::ONE)?;

        let mut debug_info = DebugInfo::new();
        Ok(super::triangulate(vec![face], tolerance, &mut debug_info))
    }
}
