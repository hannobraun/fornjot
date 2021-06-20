use nalgebra::Point;

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
