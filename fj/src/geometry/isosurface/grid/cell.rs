use std::array;

use nalgebra::{Point, Vector};

use crate::geometry::isosurface::grid;

/// A cell in a uniform grid used for isosurface extraction
#[derive(Debug, PartialEq)]
pub struct Cell {
    /// The index of the minimum (as defined by index ordering) cell vertex
    pub min_index: grid::Index,

    /// The position of the minimum (as defined by index ordering) cell vertex
    pub min_position: Point<f32, 3>,

    /// The resolution of the grid, i.e. the size of the cell
    ///
    /// Required to compute the positions of the cell's vertices.
    pub resolution: f32,
}

impl Cell {
    pub fn vertices(
        &self,
    ) -> impl Iterator<Item = (grid::Index, Point<f32, 3>)> {
        array::IntoIter::new([
            (
                self.min_index + [0, 0, 0],
                self.min_position + Vector::from([0.0, 0.0, 0.0]),
            ),
            (
                self.min_index + [0, 0, 1],
                self.min_position + Vector::from([0.0, 0.0, 1.0]),
            ),
            (
                self.min_index + [0, 1, 0],
                self.min_position + Vector::from([0.0, 1.0, 0.0]),
            ),
            (
                self.min_index + [0, 1, 1],
                self.min_position + Vector::from([0.0, 1.0, 1.0]),
            ),
            (
                self.min_index + [1, 0, 0],
                self.min_position + Vector::from([1.0, 0.0, 0.0]),
            ),
            (
                self.min_index + [1, 0, 1],
                self.min_position + Vector::from([1.0, 0.0, 1.0]),
            ),
            (
                self.min_index + [1, 1, 0],
                self.min_position + Vector::from([1.0, 1.0, 0.0]),
            ),
            (
                self.min_index + [1, 1, 1],
                self.min_position + Vector::from([1.0, 1.0, 1.0]),
            ),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn vertices_should_compute_cell_vertices() {
        let cube = Cell {
            min_index: [0, 0, 0].into(),
            min_position: [0.0, 0.0, 0.0].into(),
            resolution: 1.0,
        };

        let vertices: Vec<_> = cube.vertices().collect();

        assert_eq!(
            vertices,
            vec![
                ([0, 0, 0].into(), [0.0, 0.0, 0.0].into()),
                ([0, 0, 1].into(), [0.0, 0.0, 1.0].into()),
                ([0, 1, 0].into(), [0.0, 1.0, 0.0].into()),
                ([0, 1, 1].into(), [0.0, 1.0, 1.0].into()),
                ([1, 0, 0].into(), [1.0, 0.0, 0.0].into()),
                ([1, 0, 1].into(), [1.0, 0.0, 1.0].into()),
                ([1, 1, 0].into(), [1.0, 1.0, 0.0].into()),
                ([1, 1, 1].into(), [1.0, 1.0, 1.0].into()),
            ]
        );
    }
}
