use nalgebra::Point;

use crate::geometry::isosurface::grid;

/// A cell in a uniform grid used for isosurface extraction
#[derive(Debug, PartialEq)]
pub struct Cell {
    /// The index of the minimum (as defined by index ordering) cell vertex
    pub min_index: grid::Index,

    /// The position of the minimum (as defined by index ordering) cell vertex
    pub min_position: Point<f32, 3>,
}
