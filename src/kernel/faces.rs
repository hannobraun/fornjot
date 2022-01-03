use std::collections::BTreeSet;

use decorum::R64;
use parry3d_f64::{
    bounding_volume::AABB,
    math::Isometry,
    query::{Ray, RayCast as _},
    shape::Triangle,
};

use crate::{
    debug::{DebugInfo, TriangleEdgeCheck},
    math::Point,
};

use super::edges::Edges;

/// The faces of a shape
pub struct Faces(pub Vec<Face>);

impl Faces {
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
pub enum Face {
    /// A face of a shape
    ///
    /// A face is a section of a surface, bounded by edges. At this point, the
    /// surface is implicit, and assumed to be the x-y plane.
    Face {
        /// The edges that bound the face
        edges: Edges,
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
    pub fn triangles(
        &self,
        tolerance: f64,
        out: &mut Vec<Triangle>,
        debug_info: &mut DebugInfo,
    ) {
        match self {
            Self::Face { edges } => {
                // TASK: This only works for faces that are convex and have no
                //       holes. These limitations should be lifted, ideally. At
                //       least, the presence of either of these should cause a
                //       panic, instead of incorrect results.

                let mut vertices = Vec::new();
                edges.approx_vertices(tolerance, &mut vertices);
                let mut triangles = triangulate(&vertices);

                // For the next step, we need to represent the face as a
                // polygon, but there aren't many requirements on how
                // specifically to do that. All we need is a list of polygon
                // edges. Anything else really doesn't matter.
                let mut face_as_polygon = Vec::new();
                edges.approx_segments(tolerance, &mut face_as_polygon);

                // We're also going to need a point outside of the polygon.
                let aabb = AABB::from_points(&vertices);
                let outside = aabb.maxs * 2.;

                triangles.retain(|triangle| {
                    for segment in triangle.edges() {
                        // If the segment is an edge of the face, we don't need
                        // to take a closer look.
                        if face_as_polygon.contains(&segment) {
                            continue;
                        }

                        // To determine if the edge is within the polygon, we
                        // determine if its center point is in the polygon.
                        let center = segment.a + (segment.b - segment.a) * 0.5;

                        let ray = Ray {
                            origin: center,
                            dir: outside - center,
                        };
                        let check = TriangleEdgeCheck::new(ray);

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

                            let intersection =
                                edge.cast_local_ray(&ray, f64::INFINITY, true);

                            if let Some(t) = intersection {
                                // TASK: If the ray goes through an edge that is
                                //       parallel to it, then this code will
                                //       count that as two hits.

                                // Due to slight inaccuracies, we might get
                                // different values for the same intersections.
                                // Let's round `t` before using it.
                                let eps = 1_000_000.0;
                                let t = (t * eps).round() / eps;

                                let t: R64 = t.into();
                                hits.insert(t);
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

                out.extend(triangles);
            }
            Self::Triangles(triangles) => out.extend(triangles),
        }
    }

    pub fn transform(&mut self, transform: &Isometry<f64>) {
        match self {
            Self::Face { edges: _ } => {
                // TASK: Implement.
                todo!()
            }
            Self::Triangles(triangles) => {
                for triangle in triangles {
                    *triangle = triangle.transformed(transform);
                }
            }
        }
    }
}

/// Create a Delaunay triangulation of all vertices
pub fn triangulate(vertices: &[Point]) -> Vec<Triangle> {
    let points: Vec<_> = vertices
        .iter()
        .map(|vertex| delaunator::Point {
            x: vertex.x,
            y: vertex.y,
        })
        .collect();

    let triangulation = delaunator::triangulate(&points);

    let mut triangles = Vec::new();
    for triangle in triangulation.triangles.chunks(3) {
        let i0 = triangle[0];
        let i1 = triangle[1];
        let i2 = triangle[2];

        triangles.push([vertices[i0], vertices[i2], vertices[i1]].into());
    }

    triangles
}
