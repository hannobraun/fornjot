//! # Tools for creating approximations
//!
//! Fornjot can be viewed as a hybrid b-rep/mesh-based kernel. Topology is
//! represented using typical b-rep primitives, but geometry is approximated
//! with polylines and triangle meshes.
//!
//! Topological primitives and geometrical approximations exist side by side and
//! approximations are built up together with the topological primitives. The
//! tools provided by this module help doing that.
//!
//! This module is intended for internal use, as well as more advanced users of
//! Fornjot. It is typically required to implement operations that create and
//! modify shapes. More basic users would just use operations that others have
//! implemented, never coming into contact with this module.

use fj_math::{Point, Scalar};

use crate::new::topology::{HalfEdge, Handle, Store, Vertex};

/// # A point in an approximation with both local and global representation
///
/// When creating an approximation, you often need to deal with local
/// coordinates, either 1-dimensional on a curve or 2-dimensional on a surface.
/// These local coordinates may be converted to global 3D coordinates later, or
/// may correspond to 3D points that already exist.
///
/// Either way, storing a local point together with its corresponding global
/// point is often advantageous or even necessary, and that's what this struct
/// provides.
///
/// `ApproxPoint` is generic over the dimension of its local point. Typically,
/// only `ApproxPoint<1>` and `ApproxPoint<2>` would be used.
///
/// `ApproxPoint<2>` [implements `spade::HasPosition`][`HasPosition`] and may be
/// used together with [`spade`] for a Delaunay triangulation.
///
/// [`HasPosition`]: #impl-HasPosition-for-ApproxPoint<2>
#[derive(Clone, Copy, Debug)]
pub struct ApproxPoint<const D: usize> {
    /// # The local form of the approximation point
    pub local: Point<D>,

    /// # The global form of the approximation point
    pub global: Point<3>,
}

impl spade::HasPosition for ApproxPoint<2> {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.local.coords.components.map(|s| s.into_f64());
        spade::Point2 { x, y }
    }
}

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
pub struct ApproxHalfEdge {
    /// # The start point of the approximated half-edge
    ///
    /// An end point is not provided, as `ApproxHalfEdge` exists for the express
    /// purpose of approximating faces. In a face boundary, the end point of one
    /// half-edge is the start point of the next one, so storing only the start
    /// point of each half-edge is enough.
    pub start: ApproxPoint<2>,

    /// # The points that approximate that half-edge
    ///
    /// This is equivalent to [`HalfEdge`]'s `approx` field, and does not
    /// include start or end points.
    pub inner: Vec<ApproxPoint<2>>,
}

impl ApproxHalfEdge {
    /// # Construct `ApproxHalfEdge` by providing all points
    ///
    /// This constructor is a suitable choice, if 2D coordinates for all points
    /// are already available.
    pub fn from_points(
        start: impl Into<Point<2>>,
        inner: Vec<ApproxPoint<2>>,
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

        Self { start, inner }
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
        let inner = {
            let half_edge = &half_edges[half_edge];
            let num_coords = half_edge.approx.len();

            let local = u
                .into_iter(num_coords)
                .into_iter()
                .zip(v.into_iter(num_coords))
                .map(|(u, v)| Point::from([u, v]));
            let global = half_edge.approx.iter().copied();

            local
                .into_iter()
                .zip(global)
                .map(|(local, global)| ApproxPoint { local, global })
                .collect()
        };

        Self::from_points(start, inner, half_edge, vertices, half_edges)
    }

    /// # Iterate over all points
    ///
    /// This includes the start point and all inner points, but not the end
    /// point, which is not stored in this struct. Since in the context of a
    /// face boundary, the end point of one half-edge is the start point of the
    /// next, end points are not needed here.
    pub fn points(&self) -> impl Iterator<Item = ApproxPoint<2>> {
        [self.start].into_iter().chain(self.inner.iter().copied())
    }
}

/// # Provides iterators over the coordinates of an axis
///
/// See [`ApproxHalfEdge::from_start_and_axes`].
pub enum ApproxAxis {
    /// # Provide one fixed coordinate for the whole axis
    Fixed {
        /// # The fixed coordinate value
        value: Scalar,
    },

    /// # Provide uniformly distributed coordinates between `0` and `1`
    ///
    /// The number of coordinates provided is determined by the argument passed
    /// when calling [`ApproxAxis::into_iter`]. The coordinates provided will be
    /// _between_ `0` and `1`, excluding those limits.
    Uniform {
        /// # Indicate whether to reverse the coordinates
        ///
        /// Start with the lowest coordinate (the one closest to `0`), if this
        /// is false. Start with the highest coordinate (the one closest to
        /// `1`), if this is true.
        reverse: bool,
    },
}

impl ApproxAxis {
    pub fn fixed(value: impl Into<Scalar>) -> Self {
        let value = value.into();
        Self::Fixed { value }
    }

    pub fn into_iter(self, num_coords: usize) -> Vec<f64> {
        match self {
            ApproxAxis::Fixed { value } => (0..num_coords)
                .map(|_| value.into_f64())
                .collect::<Vec<_>>(),
            ApproxAxis::Uniform { reverse } => {
                let increment = 1. / (num_coords as f64 + 1.);

                let mut coords = (0..num_coords)
                    .map(|i| increment * (i + 1) as f64)
                    .collect::<Vec<_>>();

                if reverse {
                    coords.reverse();
                }

                coords
            }
        }
    }
}
