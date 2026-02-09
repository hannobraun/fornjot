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

pub struct HalfEdgeApprox {
    pub start: ApproxPoint<2>,
    pub inner: Vec<ApproxPoint<2>>,
}

impl HalfEdgeApprox {
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

    pub fn from_start_and_axes(
        start: impl Into<Point<2>>,
        u: Axis,
        v: Axis,
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

    pub fn points(&self) -> impl Iterator<Item = ApproxPoint<2>> {
        [self.start].into_iter().chain(self.inner.iter().copied())
    }
}

pub enum Axis {
    Fixed { value: Scalar },
    Uniform { reverse: bool },
}

impl Axis {
    pub fn fixed(value: impl Into<Scalar>) -> Self {
        let value = value.into();
        Self::Fixed { value }
    }

    pub fn into_iter(self, num_coords: usize) -> Vec<f64> {
        match self {
            Axis::Fixed { value } => (0..num_coords)
                .map(|_| value.into_f64())
                .collect::<Vec<_>>(),
            Axis::Uniform { reverse } => {
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
