use std::collections::BTreeSet;

use fj_debug::{DebugInfo, TriangleEdgeCheck};
use fj_math::{Aabb, Scalar, Segment, Triangle};
use parry2d_f64::{
    query::{Ray as Ray2, RayCast as _},
    utils::point_in_triangle::{corner_direction, Orientation},
};
use parry3d_f64::query::Ray as Ray3;
use spade::HasPosition;

use crate::{geometry, shape::Shape, topology::Face};

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

                let segments: Vec<_> = approx
                    .exterior
                    .segments()
                    .into_iter()
                    .chain(
                        approx
                            .interiors
                            .into_iter()
                            .map(|cycle_approx| cycle_approx.segments())
                            .flatten(),
                    )
                    .into_iter()
                    .map(|segment| {
                        let [a, b] = segment.points();

                        // Can't panic, unless the approximation wrongfully
                        // generates points that are not in the surface.
                        let a = surface.point_model_to_surface(a);
                        let b = surface.point_model_to_surface(b);

                        [a, b]
                    })
                    .collect();

                // We're also going to need a point outside of the polygon, for
                // the point-in-polygon tests.
                let aabb = Aabb::<2>::from_points(
                    points.iter().map(|vertex| vertex.native()),
                );
                let outside = aabb.max * 2.;

                let mut triangles = delaunay(points);
                let face_as_polygon = segments;

                triangles.retain(|t| {
                    for segment in [t[0], t[1], t[2], t[0]].windows(2) {
                        // This can't panic, as we passed `2` to `windows`. It
                        // can be cleaned up a bit, once `array_windows` is
                        // stable.
                        let segment = [segment[0], segment[1]];
                        let inverted_segment = [segment[1], segment[0]];

                        // If the segment is an edge of the face, we don't need
                        // to take a closer look.
                        if face_as_polygon.contains(&segment) {
                            continue;
                        }
                        if face_as_polygon.contains(&inverted_segment) {
                            continue;
                        }

                        // To determine if the edge is within the polygon, we
                        // determine if its center point is in the polygon.
                        let center = segment[0]
                            + (segment[1] - segment[0]) / Scalar::TWO;

                        let origin = center;
                        let dir = outside - center;
                        let ray = Ray2 {
                            origin: origin.to_na(),
                            dir: dir.to_na(),
                        };

                        let mut check = TriangleEdgeCheck::new(Ray3 {
                            origin: surface
                                .point_surface_to_model(&origin)
                                .to_na(),
                            dir: surface.vector_surface_to_model(&dir).to_na(),
                        });

                        // We need to keep track of where our ray hits the
                        // edges. Otherwise, if the ray hits a vertex, we might
                        // count that hit twice, as every vertex is attached to
                        // two edges.
                        let mut hits = BTreeSet::new();

                        // Use ray-casting to determine if `center` is within
                        // the face-polygon.
                        for edge in &face_as_polygon {
                            // Please note that we if we get to this point, then
                            // the point is not on a polygon edge, due to the
                            // check above. We don't need to handle any edge
                            // cases that would arise from that case.

                            let edge =
                                Segment::from(edge.map(|point| point.native()));

                            let intersection = edge
                                .to_parry()
                                .cast_local_ray(&ray, f64::INFINITY, true)
                                .map(Scalar::from_f64);

                            if let Some(t) = intersection {
                                // Due to slight inaccuracies, we might get
                                // different values for the same intersections.
                                // Let's round `t` before using it.
                                let eps = 1_000_000.0;
                                let t = (t * eps).round() / eps;

                                if hits.insert(t) {
                                    check.hits.push(t.into_f64());
                                }
                            }
                        }

                        debug_info.triangle_edge_checks.push(check);

                        if hits.len() % 2 == 0 {
                            // The segment is outside of the face. This means we
                            // can throw away the whole triangle.
                            return false;
                        }
                    }

                    // If we didn't throw away the triangle up till now, this
                    // means all its edges are within the face.
                    true
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
