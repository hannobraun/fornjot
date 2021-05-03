use std::{collections::HashMap, iter};

use nalgebra::Point;

use crate::geometry::attributes::Distance;

use super::{GridDescriptor, GridIndex};

pub struct Grid {
    _values: HashMap<GridIndex, f32>,
}

impl Grid {
    /// Create the grid from the descriptor and populate it with distance values
    pub fn from_descriptor(
        _descriptor: &GridDescriptor,
        _isosurface: impl Distance,
        _on_progress: impl FnMut(usize, usize),
    ) -> Self {
        // TASK: Implement
        todo!()
    }

    /// Returns iterator over all grid edges
    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        // TASK: Implement
        iter::empty()
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

pub struct Edge {
    pub a: Value,
    pub b: Value,
}

pub struct Value {
    pub index: Point<usize, 3>,
    pub value: f32,
}
