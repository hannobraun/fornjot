use std::{array, collections::BTreeMap};

use nalgebra::Point;

use crate::geometry::attributes::Distance;

use super::{Edge, GridDescriptor, GridIndex, Value};

pub struct Grid {
    _values: BTreeMap<GridIndex, f32>,
}

impl Grid {
    /// Create the grid from the descriptor and populate it with distance values
    pub fn from_descriptor(
        descriptor: &GridDescriptor,
        isosurface: impl Distance,
    ) -> Self {
        let mut values = BTreeMap::new();

        for (index, point) in descriptor.points() {
            let value = isosurface.distance(point);
            values.insert(index, value);
        }

        Self { _values: values }
    }

    /// Returns iterator over all grid edges
    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self._values
            .iter()
            .map(move |(&index, &value)| {
                let next_z = [index[0], index[1], index[2] + 1];
                let next_y = [index[0], index[1] + 1, index[2]];
                let next_x = [index[0] + 1, index[1], index[2]];

                [
                    edge_to_next(index, value, next_z.into(), &self._values),
                    edge_to_next(index, value, next_y.into(), &self._values),
                    edge_to_next(index, value, next_x.into(), &self._values),
                ]
            })
            .map(|edges| array::IntoIter::new(edges))
            .flatten()
            .filter_map(|edge| edge)
    }

    /// Returns the 4 neighboring cube centers of a grid edge
    pub fn neighbors_of_edge(
        &self,
        _a: GridIndex,
        _b: GridIndex,
    ) -> [Point<f32, 3>; 4] {
        // TASK: Implement
        todo!()
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
    use crate::geometry::{attributes::Distance, isosurface::GridDescriptor};

    use super::Grid;

    #[test]
    fn edges_should_return_edges() {
        let grid = Grid::from_descriptor(
            &GridDescriptor {
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
                ([0, 0, 0], [0, 0, 1].into()),
                ([0, 0, 0], [0, 1, 0].into()),
                ([0, 0, 0], [1, 0, 0].into()),
                ([0, 0, 1], [0, 0, 2].into()),
                ([0, 0, 1], [0, 1, 1].into()),
                ([0, 0, 1], [1, 0, 1].into()),
                ([0, 0, 2], [0, 1, 2].into()),
                ([0, 0, 2], [1, 0, 2].into()),
                ([0, 1, 0], [0, 1, 1].into()),
                ([0, 1, 0], [0, 2, 0].into()),
                ([0, 1, 0], [1, 1, 0].into()),
                ([0, 1, 1], [0, 1, 2].into()),
                ([0, 1, 1], [0, 2, 1].into()),
                ([0, 1, 1], [1, 1, 1].into()),
                ([0, 1, 2], [0, 2, 2].into()),
                ([0, 1, 2], [1, 1, 2].into()),
                ([0, 2, 0], [0, 2, 1].into()),
                ([0, 2, 0], [1, 2, 0].into()),
                ([0, 2, 1], [0, 2, 2].into()),
                ([0, 2, 1], [1, 2, 1].into()),
                ([0, 2, 2], [1, 2, 2].into()),
                ([1, 0, 0], [1, 0, 1].into()),
                ([1, 0, 0], [1, 1, 0].into()),
                ([1, 0, 0], [2, 0, 0].into()),
                ([1, 0, 1], [1, 0, 2].into()),
                ([1, 0, 1], [1, 1, 1].into()),
                ([1, 0, 1], [2, 0, 1].into()),
                ([1, 0, 2], [1, 1, 2].into()),
                ([1, 0, 2], [2, 0, 2].into()),
                ([1, 1, 0], [1, 1, 1].into()),
                ([1, 1, 0], [1, 2, 0].into()),
                ([1, 1, 0], [2, 1, 0].into()),
                ([1, 1, 1], [1, 1, 2].into()),
                ([1, 1, 1], [1, 2, 1].into()),
                ([1, 1, 1], [2, 1, 1].into()),
                ([1, 1, 2], [1, 2, 2].into()),
                ([1, 1, 2], [2, 1, 2].into()),
                ([1, 2, 0], [1, 2, 1].into()),
                ([1, 2, 0], [2, 2, 0].into()),
                ([1, 2, 1], [1, 2, 2].into()),
                ([1, 2, 1], [2, 2, 1].into()),
                ([1, 2, 2], [2, 2, 2].into()),
                ([2, 0, 0], [2, 0, 1].into()),
                ([2, 0, 0], [2, 1, 0].into()),
                ([2, 0, 1], [2, 0, 2].into()),
                ([2, 0, 1], [2, 1, 1].into()),
                ([2, 0, 2], [2, 1, 2].into()),
                ([2, 1, 0], [2, 1, 1].into()),
                ([2, 1, 0], [2, 2, 0].into()),
                ([2, 1, 1], [2, 1, 2].into()),
                ([2, 1, 1], [2, 2, 1].into()),
                ([2, 1, 2], [2, 2, 2].into()),
                ([2, 2, 0], [2, 2, 1].into()),
                ([2, 2, 1], [2, 2, 2].into()),
            ]
        );
    }

    struct Geometry;

    impl Distance for Geometry {
        fn distance(&self, _point: impl Into<nalgebra::Point<f32, 3>>) -> f32 {
            0.0
        }
    }
}
