use std::{array, collections::BTreeMap};

use nalgebra::Point;

use crate::geometry::attributes::Distance;

use super::{
    edge::{Axis, Sign},
    Edge, GridDescriptor, GridIndex, Value,
};

#[derive(Debug)]
pub struct Grid {
    descriptor: GridDescriptor,
    values: BTreeMap<GridIndex, f32>,
}

impl Grid {
    /// Create the grid from the descriptor and populate it with distance values
    pub fn from_descriptor(
        descriptor: GridDescriptor,
        isosurface: &impl Distance,
    ) -> Self {
        let values = descriptor
            .points()
            .filter_map(|(index, point)| {
                let distance = isosurface.distance(point);
                if distance <= descriptor.resolution {
                    Some((index, distance))
                } else {
                    None
                }
            })
            .collect();

        Self { descriptor, values }
    }

    /// Returns iterator over all grid edges
    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.values
            .iter()
            .map(move |(&index, &value)| {
                let next_z = [index.x(), index.y(), index.z() + 1];
                let next_y = [index.x(), index.y() + 1, index.z()];
                let next_x = [index.x() + 1, index.y(), index.z()];

                [
                    edge_to_next(index, value, next_z.into(), &self.values),
                    edge_to_next(index, value, next_y.into(), &self.values),
                    edge_to_next(index, value, next_x.into(), &self.values),
                ]
            })
            .map(|edges| array::IntoIter::new(edges))
            .flatten()
            .filter_map(|edge| edge)
    }

    /// Returns the 4 neighboring cube centers of a grid edge
    pub fn neighbors_of_edge(&self, edge: Edge) -> [Point<f32, 3>; 4] {
        let direction = edge.direction();

        // Offset from edge to cube centers around edge.
        let o = self.descriptor.resolution / 2.0;

        #[rustfmt::skip]
        let [a, b, c, d] = match direction.axis {
            Axis::Z => [
                [ o, -o, o],
                [-o, -o, o],
                [-o,  o, o],
                [ o,  o, o],
            ],
            Axis::Y => [
                [-o, o, -o],
                [ o, o, -o],
                [ o, o,  o],
                [-o, o,  o],
            ],
            Axis::X => [
                [o,  o, -o],
                [o, -o, -o],
                [o, -o,  o],
                [o,  o,  o],
            ],
        };

        let start = match direction.sign {
            Sign::Neg => edge.b,
            Sign::Pos => edge.a,
        };
        let start = start
            .index
            .to_coordinates(self.descriptor.min, self.descriptor.resolution);

        let [a, b, c, d] = if direction.sign == Sign::Pos
            && edge.a.value < edge.b.value
            || direction.sign == Sign::Neg && edge.b.value < edge.a.value
        {
            [b, a, d, c]
        } else {
            [a, b, c, d]
        };

        let neighbors = [
            start + Point::<_, 3>::from(a).coords,
            start + Point::<_, 3>::from(b).coords,
            start + Point::<_, 3>::from(c).coords,
            start + Point::<_, 3>::from(d).coords,
        ];

        neighbors
    }
}

fn edge_to_next(
    index: GridIndex,
    value: f32,
    next_index: GridIndex,
    values: &BTreeMap<GridIndex, f32>,
) -> Option<Edge> {
    let &next_value = values.get(&next_index)?;

    Some(Edge {
        a: Value { index, value },
        b: Value {
            index: next_index,
            value: next_value,
        },
    })
}

#[cfg(test)]
mod tests {
    use crate::geometry::{
        attributes::Distance,
        isosurface::{Edge, GridDescriptor, Value},
    };

    use super::Grid;

    #[test]
    fn edges_should_return_edges() {
        let grid = Grid::from_descriptor(
            GridDescriptor {
                min: [0.0, 0.0, 0.0].into(),
                max: [1.0, 1.0, 1.0].into(),
                resolution: 1.0,
            },
            &Geometry,
        );

        let edges: Vec<_> = grid
            .edges()
            .map(|edge| (edge.a.index, edge.b.index))
            .collect();

        assert_eq!(
            edges,
            vec![
                ([0, 0, 0].into(), [0, 0, 1].into()),
                ([0, 0, 0].into(), [0, 1, 0].into()),
                ([0, 0, 0].into(), [1, 0, 0].into()),
                ([0, 0, 1].into(), [0, 0, 2].into()),
                ([0, 0, 1].into(), [0, 1, 1].into()),
                ([0, 0, 1].into(), [1, 0, 1].into()),
                ([0, 0, 2].into(), [0, 1, 2].into()),
                ([0, 0, 2].into(), [1, 0, 2].into()),
                ([0, 1, 0].into(), [0, 1, 1].into()),
                ([0, 1, 0].into(), [0, 2, 0].into()),
                ([0, 1, 0].into(), [1, 1, 0].into()),
                ([0, 1, 1].into(), [0, 1, 2].into()),
                ([0, 1, 1].into(), [0, 2, 1].into()),
                ([0, 1, 1].into(), [1, 1, 1].into()),
                ([0, 1, 2].into(), [0, 2, 2].into()),
                ([0, 1, 2].into(), [1, 1, 2].into()),
                ([0, 2, 0].into(), [0, 2, 1].into()),
                ([0, 2, 0].into(), [1, 2, 0].into()),
                ([0, 2, 1].into(), [0, 2, 2].into()),
                ([0, 2, 1].into(), [1, 2, 1].into()),
                ([0, 2, 2].into(), [1, 2, 2].into()),
                ([1, 0, 0].into(), [1, 0, 1].into()),
                ([1, 0, 0].into(), [1, 1, 0].into()),
                ([1, 0, 0].into(), [2, 0, 0].into()),
                ([1, 0, 1].into(), [1, 0, 2].into()),
                ([1, 0, 1].into(), [1, 1, 1].into()),
                ([1, 0, 1].into(), [2, 0, 1].into()),
                ([1, 0, 2].into(), [1, 1, 2].into()),
                ([1, 0, 2].into(), [2, 0, 2].into()),
                ([1, 1, 0].into(), [1, 1, 1].into()),
                ([1, 1, 0].into(), [1, 2, 0].into()),
                ([1, 1, 0].into(), [2, 1, 0].into()),
                ([1, 1, 1].into(), [1, 1, 2].into()),
                ([1, 1, 1].into(), [1, 2, 1].into()),
                ([1, 1, 1].into(), [2, 1, 1].into()),
                ([1, 1, 2].into(), [1, 2, 2].into()),
                ([1, 1, 2].into(), [2, 1, 2].into()),
                ([1, 2, 0].into(), [1, 2, 1].into()),
                ([1, 2, 0].into(), [2, 2, 0].into()),
                ([1, 2, 1].into(), [1, 2, 2].into()),
                ([1, 2, 1].into(), [2, 2, 1].into()),
                ([1, 2, 2].into(), [2, 2, 2].into()),
                ([2, 0, 0].into(), [2, 0, 1].into()),
                ([2, 0, 0].into(), [2, 1, 0].into()),
                ([2, 0, 1].into(), [2, 0, 2].into()),
                ([2, 0, 1].into(), [2, 1, 1].into()),
                ([2, 0, 2].into(), [2, 1, 2].into()),
                ([2, 1, 0].into(), [2, 1, 1].into()),
                ([2, 1, 0].into(), [2, 2, 0].into()),
                ([2, 1, 1].into(), [2, 1, 2].into()),
                ([2, 1, 1].into(), [2, 2, 1].into()),
                ([2, 1, 2].into(), [2, 2, 2].into()),
                ([2, 2, 0].into(), [2, 2, 1].into()),
                ([2, 2, 1].into(), [2, 2, 2].into()),
            ]
        );
    }

    #[test]
    fn neighbors_of_edge_should_return_neighboring_grid_centers() {
        let grid = Grid::from_descriptor(
            GridDescriptor {
                min: [0.0, 0.0, 0.0].into(),
                max: [1.0, 1.0, 1.0].into(),
                resolution: 1.0,
            },
            &Geometry,
        );

        let edges = TestEdges::new();

        let [x0, x1, x2, x3] = [
            [1.0, 1.0, 0.0].into(),
            [1.0, 0.0, 0.0].into(),
            [1.0, 0.0, 1.0].into(),
            [1.0, 1.0, 1.0].into(),
        ];
        let [y0, y1, y2, y3] = [
            [0.0, 1.0, 0.0].into(),
            [1.0, 1.0, 0.0].into(),
            [1.0, 1.0, 1.0].into(),
            [0.0, 1.0, 1.0].into(),
        ];
        let [z0, z1, z2, z3] = [
            [1.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 1.0, 1.0].into(),
            [1.0, 1.0, 1.0].into(),
        ];

        assert_eq!(grid.neighbors_of_edge(edges.x), [x0, x1, x2, x3]);
        assert_eq!(grid.neighbors_of_edge(edges.y), [y0, y1, y2, y3]);
        assert_eq!(grid.neighbors_of_edge(edges.z), [z0, z1, z2, z3]);

        assert_eq!(
            grid.neighbors_of_edge(edges.x.swap_values()),
            [x1, x0, x3, x2],
        );
        assert_eq!(
            grid.neighbors_of_edge(edges.y.swap_values()),
            [y1, y0, y3, y2],
        );
        assert_eq!(
            grid.neighbors_of_edge(edges.z.swap_values()),
            [z1, z0, z3, z2],
        );
    }

    #[test]
    fn neighbors_of_edge_should_work_regardless_of_direction() {
        let grid = Grid::from_descriptor(
            GridDescriptor {
                min: [0.0, 0.0, 0.0].into(),
                max: [1.0, 1.0, 1.0].into(),
                resolution: 1.0,
            },
            &Geometry,
        );

        let edges = TestEdges::new();

        assert_eq!(
            grid.neighbors_of_edge(edges.x),
            grid.neighbors_of_edge(edges.x.reverse()),
        );
        assert_eq!(
            grid.neighbors_of_edge(edges.y),
            grid.neighbors_of_edge(edges.y.reverse()),
        );
        assert_eq!(
            grid.neighbors_of_edge(edges.z),
            grid.neighbors_of_edge(edges.z.reverse()),
        );
    }

    pub struct TestEdges {
        pub x: Edge,
        pub y: Edge,
        pub z: Edge,
    }

    impl TestEdges {
        pub fn new() -> Self {
            Self {
                x: Edge {
                    a: Value {
                        index: [1, 1, 1].into(),
                        value: 1.0,
                    },
                    b: Value {
                        index: [2, 1, 1].into(),
                        value: 0.0,
                    },
                },
                y: Edge {
                    a: Value {
                        index: [1, 1, 1].into(),
                        value: 1.0,
                    },
                    b: Value {
                        index: [1, 2, 1].into(),
                        value: 0.0,
                    },
                },
                z: Edge {
                    a: Value {
                        index: [1, 1, 1].into(),
                        value: 1.0,
                    },
                    b: Value {
                        index: [1, 1, 2].into(),
                        value: 0.0,
                    },
                },
            }
        }
    }

    struct Geometry;

    impl Distance for Geometry {
        fn distance(&self, _point: impl Into<nalgebra::Point<f32, 3>>) -> f32 {
            0.0
        }
    }
}
