use std::{collections::BTreeMap, iter};

use nalgebra::Point;

use crate::geometry::attributes::Distance;

use super::{GridDescriptor, GridIndex};

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

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub a: Value,
    pub b: Value,
}

#[derive(Debug, PartialEq)]
pub struct Value {
    pub index: GridIndex,
    pub value: f32,
}
