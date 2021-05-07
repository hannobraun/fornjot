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
        isosurface: impl Distance,
    ) -> Self {
        let mut values = BTreeMap::new();

        for (index, point) in descriptor.points() {
            let value = isosurface.distance(point);
            values.insert(index, value);
        }

        Self { descriptor, values }
    }

    /// Returns iterator over all grid edges
    pub fn edges(&self) -> impl Iterator<Item = Edge<Value>> + '_ {
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
    pub fn neighbors_of_edge(
        &self,
        edge: Edge<GridIndex>,
    ) -> [Point<f32, 3>; 4] {
        let direction = edge.direction();

        let start = match direction.sign {
            Sign::Neg => edge.b,
            Sign::Pos => edge.a,
        };

        let start = start
            .to_coordinates(self.descriptor.min, self.descriptor.resolution);
        let o = self.descriptor.resolution / 2.0;

        #[rustfmt::skip]
        let [a, b, c, d] = match direction.axis {
            Axis::Z => [
                [-o, -o, o],
                [ o, -o, o],
                [ o,  o, o],
                [-o,  o, o],
            ],
            Axis::Y => [
                [-o, o, -o],
                [ o, o, -o],
                [ o, o,  o],
                [-o, o,  o],
            ],
            Axis::X => [
                [o, -o, -o],
                [o,  o, -o],
                [o,  o,  o],
                [o, -o,  o],
            ],
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
) -> Option<Edge<Value>> {
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
        isosurface::{Edge, GridDescriptor},
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
            Geometry,
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
            Geometry,
        );

        let neighbors_x_pos = grid.neighbors_of_edge(Edge {
            a: [1, 1, 1].into(),
            b: [2, 1, 1].into(),
        });
        let neighbors_x_neg = grid.neighbors_of_edge(Edge {
            a: [2, 1, 1].into(),
            b: [1, 1, 1].into(),
        });
        let neighbors_y_pos = grid.neighbors_of_edge(Edge {
            a: [1, 1, 1].into(),
            b: [1, 2, 1].into(),
        });
        let neighbors_y_neg = grid.neighbors_of_edge(Edge {
            a: [1, 2, 1].into(),
            b: [1, 1, 1].into(),
        });
        let neighbors_z_pos = grid.neighbors_of_edge(Edge {
            a: [1, 1, 1].into(),
            b: [1, 1, 2].into(),
        });
        let neighbors_z_neg = grid.neighbors_of_edge(Edge {
            a: [1, 1, 2].into(),
            b: [1, 1, 1].into(),
        });

        let neighbors_x_expected = [
            [1.0, 0.0, 0.0].into(),
            [1.0, 1.0, 0.0].into(),
            [1.0, 1.0, 1.0].into(),
            [1.0, 0.0, 1.0].into(),
        ];
        let neighbors_y_expected = [
            [0.0, 1.0, 0.0].into(),
            [1.0, 1.0, 0.0].into(),
            [1.0, 1.0, 1.0].into(),
            [0.0, 1.0, 1.0].into(),
        ];
        let neighbors_z_expected = [
            [0.0, 0.0, 1.0].into(),
            [1.0, 0.0, 1.0].into(),
            [1.0, 1.0, 1.0].into(),
            [0.0, 1.0, 1.0].into(),
        ];

        assert_eq!(neighbors_x_pos, neighbors_x_expected);
        assert_eq!(neighbors_x_neg, neighbors_x_expected);
        assert_eq!(neighbors_y_pos, neighbors_y_expected);
        assert_eq!(neighbors_y_neg, neighbors_y_expected);
        assert_eq!(neighbors_z_pos, neighbors_z_expected);
        assert_eq!(neighbors_z_neg, neighbors_z_expected);
    }

    struct Geometry;

    impl Distance for Geometry {
        fn distance(&self, _point: impl Into<nalgebra::Point<f32, 3>>) -> f32 {
            0.0
        }
    }
}
