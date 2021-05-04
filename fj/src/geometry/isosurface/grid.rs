use std::{array, collections::BTreeMap};

use nalgebra::Point;

use crate::geometry::attributes::Distance;

use super::{Edge, GridDescriptor, GridIndex, Value};

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
    pub fn neighbors_of_edge(
        &self,
        a: GridIndex,
        b: GridIndex,
    ) -> [Point<f32, 3>; 4] {
        let mut direction = [
            b.x() as i32 - a.x() as i32,
            b.y() as i32 - a.y() as i32,
            b.z() as i32 - a.z() as i32,
        ];

        // Means the direction is left, front, or down. This check assumes that
        // the direction is valid, which might not be the case at this point. If
        // it's not, we'll catch it down in the `match` statement.
        let start = if direction.contains(&-1) {
            direction[0] = direction[0].abs();
            direction[1] = direction[1].abs();
            direction[2] = direction[2].abs();
            b
        } else {
            a
        };

        let start = start
            .to_coordinates(self.descriptor.min, self.descriptor.resolution);

        let neighbors = match direction {
            [0, 0, 1] => [
                start + Point::<_, 3>::from([-0.5, -0.5, 0.5]).coords,
                start + Point::<_, 3>::from([-0.5, 0.5, 0.5]).coords,
                start + Point::<_, 3>::from([0.5, -0.5, 0.5]).coords,
                start + Point::<_, 3>::from([0.5, 0.5, 0.5]).coords,
            ],
            [0, 1, 0] => [
                start + Point::<_, 3>::from([-0.5, 0.5, -0.5]).coords,
                start + Point::<_, 3>::from([-0.5, 0.5, 0.5]).coords,
                start + Point::<_, 3>::from([0.5, 0.5, -0.5]).coords,
                start + Point::<_, 3>::from([0.5, 0.5, 0.5]).coords,
            ],
            [1, 0, 0] => [
                start + Point::<_, 3>::from([0.5, -0.5, -0.5]).coords,
                start + Point::<_, 3>::from([0.5, -0.5, 0.5]).coords,
                start + Point::<_, 3>::from([0.5, 0.5, -0.5]).coords,
                start + Point::<_, 3>::from([0.5, 0.5, 0.5]).coords,
            ],
            direction => panic!(
                "Invalid direction ({:?}).\
                Only axis-aligned directions allowed.",
                direction
            ),
        };

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
    use crate::geometry::{attributes::Distance, isosurface::GridDescriptor};

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

        let x_neighbors = [
            [1.0, 0.0, 0.0].into(),
            [1.0, 0.0, 1.0].into(),
            [1.0, 1.0, 0.0].into(),
            [1.0, 1.0, 1.0].into(),
        ];
        let y_neighbors = [
            [0.0, 1.0, 0.0].into(),
            [0.0, 1.0, 1.0].into(),
            [1.0, 1.0, 0.0].into(),
            [1.0, 1.0, 1.0].into(),
        ];
        let z_neighbors = [
            [0.0, 0.0, 1.0].into(),
            [0.0, 1.0, 1.0].into(),
            [1.0, 0.0, 1.0].into(),
            [1.0, 1.0, 1.0].into(),
        ];

        assert_eq!(
            grid.neighbors_of_edge([1, 1, 1].into(), [2, 1, 1].into()),
            x_neighbors,
        );
        assert_eq!(
            grid.neighbors_of_edge([2, 1, 1].into(), [1, 1, 1].into()),
            x_neighbors,
        );
        assert_eq!(
            grid.neighbors_of_edge([1, 1, 1].into(), [1, 2, 1].into()),
            y_neighbors,
        );
        assert_eq!(
            grid.neighbors_of_edge([1, 2, 1].into(), [1, 1, 1].into()),
            y_neighbors,
        );
        assert_eq!(
            grid.neighbors_of_edge([1, 1, 1].into(), [1, 1, 2].into()),
            z_neighbors,
        );
        assert_eq!(
            grid.neighbors_of_edge([1, 1, 2].into(), [1, 1, 1].into()),
            z_neighbors,
        );
    }

    struct Geometry;

    impl Distance for Geometry {
        fn distance(&self, _point: impl Into<nalgebra::Point<f32, 3>>) -> f32 {
            0.0
        }
    }
}
