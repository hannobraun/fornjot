use fj_math::Point;

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
    pub other: Vec<ApproxPoint<2>>,
}

impl HalfEdgeApprox {
    pub fn with_evenly_distributed_local_coords(
        start: impl Into<Point<2>>,
        fixed_coord: FixedCoord,
        reverse: bool,
        half_edge: Index<HalfEdge>,
        vertices: &Store<Vertex>,
        half_edges: &Store<HalfEdge>,
    ) -> Self {
        let start = ApproxPoint {
            local: start.into(),
            global: vertices[half_edges[half_edge].boundary[0]].point,
        };

        Self {
            start,
            other: local_approx_coords(
                half_edge,
                fixed_coord,
                half_edges,
                reverse,
            ),
        }
    }

    pub fn points(&self) -> impl Iterator<Item = ApproxPoint<2>> {
        [self.start].into_iter().chain(self.other.iter().copied())
    }
}

pub fn local_approx_coords(
    half_edge: Index<HalfEdge>,
    fixed: FixedCoord,
    half_edges: &Store<HalfEdge>,
    reverse: bool,
) -> Vec<ApproxPoint<2>> {
    let half_edge = &half_edges[half_edge];

    let local = {
        let increment = 1. / (half_edge.approx.len() as f64 + 1.);

        let mut points = (0..half_edge.approx.len())
            .map(|i| increment * (i + 1) as f64)
            .collect::<Vec<_>>();

        if reverse {
            points.reverse();
        }

        points
    };
    let global = half_edge.approx.iter().copied();

    local
        .into_iter()
        .zip(global)
        .map(|(local, global)| {
            let (u, v) = match fixed {
                FixedCoord::U { value } => (value, local),
                FixedCoord::V { value } => (local, value),
            };

            ApproxPoint {
                local: Point::from([u, v]),
                global,
            }
        })
        .collect()
}

pub enum FixedCoord {
    U { value: f64 },
    V { value: f64 },
}
