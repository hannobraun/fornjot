use std::array;

use crate::math::Point;

use super::Index;

/// A cell in a uniform grid used for isosurface extraction
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    /// The index of the minimum (as defined by index ordering) cell vertex
    pub min_index: Index,

    /// The position of the minimum (as defined by index ordering) cell vertex
    pub min_position: Point<3>,
}

impl Cell {
    /// Iterate over the indices of the cell's vertices
    pub fn vertices(&self) -> impl Iterator<Item = Index> + '_ {
        let vertices = [
            [0, 0, 0],
            [0, 0, 1],
            [0, 1, 0],
            [0, 1, 1],
            [1, 0, 0],
            [1, 0, 1],
            [1, 1, 0],
            [1, 1, 1],
        ];

        array::IntoIter::new(vertices)
            .map(move |cell_index| self.min_index + cell_index)
    }

    /// Iterate over the edges of the cell
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
