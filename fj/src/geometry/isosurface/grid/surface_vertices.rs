use std::collections::BTreeMap;

use nalgebra::Point;

use super::{
    edge::{Axis, Sign},
    Edge, Index,
};

#[derive(Debug)]
pub struct SurfaceVertices(pub BTreeMap<Index, Point<f32, 3>>);

impl SurfaceVertices {
    pub fn neighbors_of_edge(&self, edge: Edge) -> [Point<f32, 3>; 4] {
        let direction = edge.direction();

        #[rustfmt::skip]
        let [a, b, c, d] = match direction.axis {
            Axis::Z => [
                [ 0, -1, 0],
                [-1, -1, 0],
                [-1,  0, 0],
                [ 0,  0, 0],
            ],
            Axis::Y => [
                [-1, 0, -1],
                [ 0, 0, -1],
                [ 0, 0,  0],
                [-1, 0,  0],
            ],
            Axis::X => [
                [0,  0, -1],
                [0, -1, -1],
                [0, -1,  0],
                [0,  0,  0],
            ],
        };

        let start = match direction.sign {
            Sign::Neg => edge.b,
            Sign::Pos => edge.a,
        };
        let start = start.index;

        let [a, b, c, d] = if direction.sign == Sign::Pos
            && edge.a.distance < edge.b.distance
            || direction.sign == Sign::Neg && edge.b.distance < edge.a.distance
        {
            [b, a, d, c]
        } else {
            [a, b, c, d]
        };

        let neighbors = [
            self.0[&(start + a)],
            self.0[&(start + b)],
            self.0[&(start + c)],
            self.0[&(start + d)],
        ];

        neighbors
    }
}
