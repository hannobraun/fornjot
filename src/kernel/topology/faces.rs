use std::collections::BTreeSet;

use parry2d_f64::query::{Ray as Ray2, RayCast as _};
use parry3d_f64::query::Ray as Ray3;

use crate::{
    debug::{DebugInfo, TriangleEdgeCheck},
    kernel::{
        approximation::Approximation, geometry::Surface,
        triangulation::triangulate,
    },
    math::{Aabb, Scalar, Segment, Transform, Triangle},
};

use super::edges::Edges;

/// The faces of a shape
#[derive(Clone)]
pub struct Faces(pub Vec<Face>);

impl Faces {
    /// Transform all the faces
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        let faces = self
            .0
            .into_iter()
            .map(|face| face.transform(transform))
            .collect();

        Self(faces)
    }

    pub fn triangles(
        &self,
        tolerance: Scalar,
        out: &mut Vec<Triangle<3>>,
        debug_info: &mut DebugInfo,
    ) {
        for face in &self.0 {
            face.triangles(tolerance, out, debug_info);
        }
    }
}

/// A face of a shape
#[derive(Clone)]
pub enum Face {
    /// A face of a shape
    ///
    /// A face is defined by a surface, and is bounded by edges that lie in that
    /// surface.
    Face {
        /// The surface that defines this face
        surface: Surface,

        /// The edges that bound the face
        ///
        /// # Implementation Note
        ///
        /// Since these edges bound the face, they must lie in the face. We're
        /// using [`Edges`] here, however, which has no such limitation.
        ///
        /// It might be less error-prone, and possibly more efficient, to use a
        /// more specialized data structure here, that specifies the edges in
        /// surface coordinates.
        edges: Edges,
    },

    /// The triangles of the face
    ///
    /// Representing faces as a collection of triangles is a temporary state.
    /// The plan is to eventually represent faces as a geometric surface,
    /// bounded by edges. While the transition is being made, this variant is
    /// still required.
    Triangles(Vec<Triangle<3>>),
}

impl Face {
    /// Transform the face
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        match self {
            Self::Face { edges, surface } => Self::Face {
                edges: edges.transform(transform),
                surface: surface.transform(transform),
            },
            Self::Triangles(mut triangles) => {
                for triangle in &mut triangles {
                    *triangle = transform.transform_triangle(triangle);
                }

                Self::Triangles(triangles)
            }
        }
    }

    pub fn triangles(
        &self,
        tolerance: Scalar,
        out: &mut Vec<Triangle<3>>,
        debug_info: &mut DebugInfo,
    ) {
        match self {
            Self::Face { surface, .. } => {
                let approx = Approximation::for_face(self, tolerance);

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
                    .segments
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
                    points.iter().map(|vertex| vertex.native),
                );
                let outside = aabb.max * 2.;

                let mut triangles = triangulate(points);
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
                                Segment::from(edge.map(|point| point.native));

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
                    let [a, b, c] = triangle.map(|point| point.canonical);
                    Triangle::from([a, b, c])
                }));
            }
            Self::Triangles(triangles) => out.extend(triangles),
        }
    }
}
