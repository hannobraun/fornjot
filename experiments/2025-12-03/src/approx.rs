use fj_math::{Point, Scalar};

use crate::{
    store::{Index, Store},
    topology::{HalfEdge, Vertex},
};

#[derive(Clone, Copy, Debug)]
pub struct ApproxPoint<const D: usize> {
    pub local: Point<D>,
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
        half_edge: Index<HalfEdge>,
        inner: Vec<ApproxPoint<2>>,
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

    pub fn from_axes(
        start: impl Into<Point<2>>,
        u: Axis,
        v: Axis,
        half_edge: Index<HalfEdge>,
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

        Self::from_points(start, half_edge, inner, vertices, half_edges)
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
        let increment = 1. / (num_coords as f64 + 1.);

        match self {
            Axis::Fixed { value } => (0..num_coords)
                .map(|_| value.into_f64())
                .collect::<Vec<_>>(),
            Axis::Uniform { reverse } => {
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
