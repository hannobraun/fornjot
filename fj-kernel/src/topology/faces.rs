use std::{
    collections::BTreeSet,
    hash::{Hash, Hasher},
};

use fj_debug::{DebugInfo, TriangleEdgeCheck};
use fj_math::{Aabb, Scalar, Segment, Triangle};
use parry2d_f64::query::{Ray as Ray2, RayCast as _};
use parry3d_f64::query::Ray as Ray3;

use crate::{
    algorithms::{delaunay, Approximation},
    geometry::Surface,
    shape::Handle,
};

use super::edges::Cycle;

/// A face of a shape
///
/// # Equality
///
/// Please refer to [`crate::kernel::topology`] for documentation on the
/// equality of topological objects.
#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub enum Face {
    /// A face of a shape
    ///
    /// A face is defined by a surface, and is bounded by edges that lie in that
    /// surface.
    Face {
        /// The surface that defines this face
        surface: Handle<Surface>,

        /// The cycles that bound the face
        ///
        /// # Implementation Note
        ///
        /// Since these cycles bound the face, the edges they consist of must
        /// lie in the surface. The data we're using here is 3-dimensional
        /// though, so no such limitation is enforced.
        ///
        /// It might be less error-prone to specify the edges in surface
        /// coordinates.
        cycles: Vec<Handle<Cycle>>,

        /// The color of the face
        color: [u8; 4],
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
    /// Access the surface that the face refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`].
    pub fn surface(&self) -> Surface {
        match self {
            Self::Face { surface, .. } => *surface.get(),
            _ => {
                // No code that still uses triangle representation is calling
                // this method.
                unreachable!()
            }
        }
    }

    /// Access the cycles that the face refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn cycles(&self) -> impl Iterator<Item = Cycle> + '_ {
        match self {
            Self::Face { cycles, .. } => {
                cycles.iter().map(|handle| handle.get().clone())
            }
            _ => {
                // No code that still uses triangle representation is calling
                // this method.
                unreachable!()
            }
        }
    }

    /// Triangulate the face
    pub fn triangles(
        &self,
        tolerance: Scalar,
        out: &mut Vec<Triangle<3>>,
        debug_info: &mut DebugInfo,
    ) {
        match self {
            Self::Face { surface, color, .. } => {
                let surface = surface.get();
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
            Self::Triangles(triangles) => out.extend(triangles),
        }
    }
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.surface() == other.surface() && self.cycles().eq(other.cycles())
    }
}

impl Hash for Face {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.surface().hash(state);
        for cycle in self.cycles() {
            cycle.hash(state);
        }
    }
}
