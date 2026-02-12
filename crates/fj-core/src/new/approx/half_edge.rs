use fj_math::Point;

use crate::new::{
    approx::{ApproxAxis, ApproxPoint},
    topology::{HalfEdge, Handle, Store, Vertex},
};

/// # A representation of a half-edge, for the purpose of approximation
///
/// When creating the approximation of a face, 2D coordinates are required for
/// the Delaunay triangulation. `ApproxHalfEdge` provides these, in addition to
/// global 3D coordinates.
///
/// These 2D coordinates may already be available, for example when creating a
/// face from a sketch. Or they may not be, making it necessary to create
/// suitable 2D coordinates for the approximation. `ApproxHalfEdge` provides
/// constructors for both cases.
#[derive(Clone)]
pub struct ApproxHalfEdge {
    /// # The start point of the half-edge
    ///
    /// An end point is not provided, as `ApproxHalfEdge` exists for the express
    /// purpose of approximating faces. In a face boundary, the end point of one
    /// half-edge is the start point of the next one, so storing only the start
    /// point of each half-edge is enough.
    pub start: ApproxPoint<2>,

    /// # The points that approximate the curve between start and end points
    ///
    /// This is equivalent to [`HalfEdge`]'s `approx` field, and does not
    /// include start or end points.
    pub curve: Vec<ApproxPoint<2>>,
}

impl ApproxHalfEdge {
    /// # Construct `ApproxHalfEdge` by providing all points
    ///
    /// This constructor is a suitable choice, if 2D coordinates for all points
    /// are already available.
    pub fn from_points(
        start: impl Into<Point<2>>,
        curve: Vec<ApproxPoint<2>>,
        half_edge: Handle<HalfEdge>,
        vertices: &Store<Vertex>,
        half_edges: &Store<HalfEdge>,
    ) -> Self {
        let start = {
            let local = start.into();
            let global = {
                let [vertex, _] = half_edges[half_edge].boundary;
                vertices[vertex].point
            };

            ApproxPoint { local, global }
        };

        Self { start, curve }
    }

    /// # Construct `ApproxHalfEdge` from axes
    ///
    /// Only the `start` point needs to be provided directly. The other points
    /// are taken from the provided `u` and `v` axes. Usually, you'd combine an
    /// [`ApproxAxis::Fixed`] with an [`ApproxAxis::Uniform`], to get uniformly
    /// distributed points along one axis, like `[0, 0.1]`, `[0, 0.2]`, ...
    ///
    /// This is often good enough. The key insight here is, that you don't need
    /// a perfectly accurate local representation of the face's 3D points (like
    /// their projection into a surface) to create a triangulation. All you need
    /// is the correct number of points, relatively position in the correct way.
    ///
    /// As long as we connect the points with the right triangles, it doesn't
    /// matter at all what those triangles are in local coordinates. We throw
    /// away the local coordinates afterwards and only remember global ones. As
    /// long as those are connected correctly, it doesn't matter how we did it.
    pub fn from_start_and_axes(
        start: impl Into<Point<2>>,
        u: ApproxAxis,
        v: ApproxAxis,
        half_edge: Handle<HalfEdge>,
        vertices: &Store<Vertex>,
        half_edges: &Store<HalfEdge>,
    ) -> Self {
        let curve = {
            let half_edge = &half_edges[half_edge];
            let num_coords = half_edge.approx.len();

            let local = u
                .into_iter(num_coords)
                .zip(v.into_iter(num_coords))
                .map(|(u, v)| Point::from([u, v]));
            let global = half_edge.approx.iter().copied();

            local
                .into_iter()
                .zip(global)
                .map(|(local, global)| ApproxPoint { local, global })
                .collect()
        };

        Self::from_points(start, curve, half_edge, vertices, half_edges)
    }

    /// # Iterate over all points
    ///
    /// This includes the start point and all curve points, but not the end
    /// point, which is not stored in this struct. Since in the context of a
    /// face boundary, the end point of one half-edge is the start point of the
    /// next, end points are not needed here.
    pub fn points(&self) -> impl Iterator<Item = ApproxPoint<2>> {
        [self.start].into_iter().chain(self.curve.iter().copied())
    }
}
