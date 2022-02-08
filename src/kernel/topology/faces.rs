use std::collections::BTreeSet;

use decorum::R64;
use parry2d_f64::{
    bounding_volume::AABB,
    query::{Ray as Ray2, RayCast as _},
    shape::Segment as Segment2,
};
use parry3d_f64::{
    math::Isometry,
    query::Ray as Ray3,
    shape::{Segment as Segment3, Triangle},
};
use tracing::warn;

use crate::{
    debug::{DebugInfo, TriangleEdgeCheck},
    kernel::{geometry::Surface, triangulation::triangulate},
};

use super::edges::Edges;

/// The faces of a shape
#[derive(Clone)]
pub struct Faces(pub Vec<Face>);

impl Faces {
    /// Transform all the faces
    #[must_use]
    pub fn transform(self, transform: &Isometry<f64>) -> Self {
        let faces = self
            .0
            .into_iter()
            .map(|face| face.transform(transform))
            .collect();

        Self(faces)
    }

    pub fn triangles(
        &self,
        tolerance: f64,
        out: &mut Vec<Triangle>,
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

        /// The surface that defines this face
        surface: Surface,
    },

    /// The triangles of the face
    ///
    /// Representing faces as a collection of triangles is a temporary state.
    /// The plan is to eventually represent faces as a geometric surface,
    /// bounded by edges. While the transition is being made, this variant is
    /// still required.
    Triangles(Vec<Triangle>),
}

impl Face {
    /// Transform the face
    #[must_use]
    pub fn transform(self, transform: &Isometry<f64>) -> Self {
        match self {
            Self::Face { edges, surface } => Self::Face {
                edges: edges.transform(transform),
                surface: surface.transform(transform),
            },
            Self::Triangles(mut triangles) => {
                for triangle in &mut triangles {
                    *triangle = triangle.transformed(transform);
                }

                Self::Triangles(triangles)
            }
        }
    }

    pub fn triangles(
        &self,
        tolerance: f64,
        out: &mut Vec<Triangle>,
        debug_info: &mut DebugInfo,
    ) {
        match self {
            Self::Face { edges, surface } => {
                let approx = edges.approx(tolerance);

                // Can't make this a panic, as the current approximation code
                // actually produces invalid approximations.
                if let Err(err) = approx.validate() {
                    warn!("Invalid approximation: {:?}", err);
                }

                let points: Vec<_> = approx
                    .points
                    .into_iter()
                    .map(|vertex| {
                        // Can't panic, unless the approximation wrongfully
                        // generates points that are not in the surface.
                        surface.point_model_to_surface(vertex).unwrap()
                    })
                    .collect();

                let segments: Vec<_> = approx
                    .segments
                    .into_iter()
                    .map(|Segment3 { a, b }| {
                        // Can't panic, unless the approximation wrongfully
                        // generates points that are not in the surface.
                        let a = surface.point_model_to_surface(a).unwrap();
                        let b = surface.point_model_to_surface(b).unwrap();

                        [a, b]
                    })
                    .collect();

                // We're also going to need a point outside of the polygon, for
                // the point-in-polygon tests.
                let aabb = AABB::from_points(
                    points.iter().map(|vertex| &vertex.value),
                );
                let outside = aabb.maxs * 2.;

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
                        let center =
                            segment[0] + (segment[1] - segment[0]) * 0.5;

                        let ray = Ray2 {
                            origin: center,
                            dir: outside - center,
                        };
                        let mut check = TriangleEdgeCheck::new(Ray3 {
                            origin: surface.point_surface_to_model(ray.origin),
                            dir: surface.vector_surface_to_model(ray.dir),
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
                                Segment2::from(edge.map(|point| point.value));

                            let intersection =
                                edge.cast_local_ray(&ray, f64::INFINITY, true);

                            if let Some(t) = intersection {
                                // Due to slight inaccuracies, we might get
                                // different values for the same intersections.
                                // Let's round `t` before using it.
                                let eps = 1_000_000.0;
                                let t = (t * eps).round() / eps;

                                let t_r64: R64 = t.into();
                                if hits.insert(t_r64) {
                                    check.hits.push(t);
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
                    let [a, b, c] = triangle.map(|point| point.from);
                    Triangle { a, b, c }
                }));
            }
            Self::Triangles(triangles) => out.extend(triangles),
        }
    }
}
