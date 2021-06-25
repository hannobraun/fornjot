use std::array;

use nalgebra::{Point, SVector};

use super::Index;

/// A cell in a uniform grid used for isosurface extraction
#[derive(Debug, PartialEq)]
pub struct Cell {
    /// The index of the minimum (as defined by index ordering) cell vertex
    pub min_index: Index,

    /// The position of the minimum (as defined by index ordering) cell vertex
    pub min_position: Point<f32, 3>,
}

impl Cell {
    pub fn vertices(
        &self,
        resolution: f32,
    ) -> impl Iterator<Item = (Index, Point<f32, 3>)> + '_ {
        let cell_vertices = [
            [0, 0, 0],
            [0, 0, 1],
            [0, 1, 0],
            [0, 1, 1],
            [1, 0, 0],
            [1, 0, 1],
            [1, 1, 0],
            [1, 1, 1],
        ];

        array::IntoIter::new(cell_vertices).map(move |cell_vertex| {
            let grid_index = self.min_index + cell_vertex;
            let grid_vertex = self.min_position
                + SVector::from(cell_vertex).map(|c| c as f32 * resolution);

            (grid_index, grid_vertex)
        })
    }

    pub fn edges(&self) -> impl Iterator<Item = (Index, Index)> + '_ {
        let edges = [
            ([0, 0, 0], [0, 0, 1]),
            ([0, 0, 0], [0, 1, 0]),
            ([0, 0, 0], [1, 0, 0]),
            ([0, 0, 1], [0, 1, 1]),
            ([0, 0, 1], [1, 0, 1]),
            ([0, 1, 0], [0, 1, 1]),
            ([0, 1, 0], [1, 1, 0]),
            ([0, 1, 1], [1, 1, 1]),
            ([1, 0, 0], [1, 0, 1]),
            ([1, 0, 0], [1, 1, 0]),
            ([1, 0, 1], [1, 1, 1]),
            ([1, 1, 0], [1, 1, 1]),
        ];

        array::IntoIter::new(edges)
            .map(move |(a, b)| (self.min_index + a, self.min_index + b))
    }
}
