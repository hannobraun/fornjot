use std::collections::BTreeSet;

use decorum::R64;
use parry2d_f64::{
    bounding_volume::AABB,
    query::{Ray as Ray2, RayCast as _},
    shape::Triangle as Triangle2,
    utils::point_in_triangle::{corner_direction, Orientation},
};
use parry3d_f64::{
    math::Isometry, query::Ray as Ray3, shape::Triangle as Triangle3,
};

use crate::{
    debug::{DebugInfo, TriangleEdgeCheck},
    kernel::geometry::Surface,
    math::Point,
};

use super::edges::Edges;

/// The faces of a shape
#[derive(Clone)]
pub struct Faces(pub Vec<Face>);

impl Faces {
    pub fn transform(mut self, transform: &Isometry<f64>) -> Self {
        for face in &mut self.0 {
            face.transform(transform);
        }

        self
    }

    pub fn triangles(
        &self,
        tolerance: f64,
        out: &mut Vec<Triangle3>,
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
    /// A face is a section of a surface, bounded by edges that lie in that
    /// surface. At this point, the surface is implicit, and assumed to be the
    /// x-y plane.
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

        /// The surface that this face is a section of
        surface: Surface,
    },

    /// The triangles of the face
    ///
    /// Representing faces as a collection of triangles is a temporary state.
    /// The plan is to eventually represent faces as a geometric surface,
    /// bounded by edges. While the transition is being made, this variant is
    /// still required.
    Triangles(Vec<Triangle3>),
}

impl Face {
    pub fn transform(&mut self, transform: &Isometry<f64>) {
        match self {
            Self::Face { edges, surface } => {
                edges.transform(transform);
                surface.transform(transform);
            }
            Self::Triangles(triangles) => {
                for triangle in triangles {
                    *triangle = triangle.transformed(transform);
                }
            }
        }
    }

    pub fn triangles(
        &self,
        tolerance: f64,
        out: &mut Vec<Triangle3>,
        debug_info: &mut DebugInfo,
    ) {
        match self {
            Self::Face { edges, surface } => {
                let approx = edges.approx(tolerance, surface);
                let mut triangles = triangulate(&approx.vertices);
                let face_as_polygon = approx.segments;

                // We're also going to need a point outside of the polygon.
                let aabb = AABB::from_points(&approx.vertices);
                let outside = aabb.maxs * 2.;

                triangles.retain(|triangle| {
                    'outer: for segment in triangle.edges() {
                        let mut inverted_segment = segment;
                        inverted_segment.swap();

                        // If the segment is an edge of the face, we don't need
                        // to take a closer look.
                        //
                        // We can't use `contains` here, as that compares the
                        // segments directly, without taking floating point
                        // accuracy into account.
                        //
                        // This is not great. See this issue:
                        // https://github.com/hannobraun/Fornjot/issues/78
                        for s in &face_as_polygon {
                            // This epsilon value is small enough to not mistake
                            // different vertices as equal, for common use
                            // cases, while still being a few orders of
                            // magnitude larger than the differences between
                            // equal vertices that I've seen so far.
                            let eps = 1e-12;

                            let aa = (s.a - segment.a).magnitude();
                            let bb = (s.b - segment.b).magnitude();
                            let ab = (s.a - segment.b).magnitude();
                            let ba = (s.b - segment.a).magnitude();

                            let segment_is_face_edge = aa < eps && bb < eps;
                            let segment_is_inverted_face_edge =
                                ab < eps && ba < eps;

                            if segment_is_face_edge
                                || segment_is_inverted_face_edge
                            {
                                continue 'outer;
                            }
                        }

                        // To determine if the edge is within the polygon, we
                        // determine if its center point is in the polygon.
                        let center = segment.a + (segment.b - segment.a) * 0.5;

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

                out.extend(triangles.into_iter().map(
                    |Triangle2 { a, b, c }| {
                        let a = surface.point_surface_to_model(a);
                        let b = surface.point_surface_to_model(b);
                        let c = surface.point_surface_to_model(c);

                        Triangle3 { a, b, c }
                    },
                ));
            }
            Self::Triangles(triangles) => out.extend(triangles),
        }
    }
}

/// Create a Delaunay triangulation of all vertices
pub fn triangulate(vertices: &[Point<2>]) -> Vec<Triangle2> {
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

        let v0 = vertices[i0];
        let v1 = vertices[i1];
        let v2 = vertices[i2];

        let triangle = match corner_direction(&v0, &v1, &v2) {
            Orientation::Ccw => [v0, v1, v2].into(),
            Orientation::Cw => [v0, v2, v1].into(),
            Orientation::None => {
                panic!(
                    "Triangle returned from triangulation isn't actually a\
                    triangle"
                );
            }
        };

        triangles.push(triangle);
    }

    triangles
}
