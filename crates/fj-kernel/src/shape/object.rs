use fj_math::Point;

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{validate::Validate, Shape, ValidationResult};

/// Marker trait for geometric and topological objects
pub trait Object:
    'static + Clone + PartialEq + Validate + private::Sealed
{
    /// Internal function
    ///
    /// Please consider using [`Shape::merge`] instead.
    fn merge_into(self, shape: &mut Shape) -> ValidationResult<Self>;
}

impl private::Sealed for Point<3> {}
impl private::Sealed for Curve {}
impl private::Sealed for Surface {}

impl private::Sealed for Vertex {}
impl private::Sealed for Edge {}
impl private::Sealed for Cycle {}
impl private::Sealed for Face {}

impl Object for Point<3> {
    fn merge_into(self, shape: &mut Shape) -> ValidationResult<Self> {
        shape.get_handle_or_insert(self)
    }
}

impl Object for Curve {
    fn merge_into(self, shape: &mut Shape) -> ValidationResult<Self> {
        shape.get_handle_or_insert(self)
    }
}

impl Object for Surface {
    fn merge_into(self, shape: &mut Shape) -> ValidationResult<Self> {
        shape.get_handle_or_insert(self)
    }
}

impl Object for Vertex {
    fn merge_into(self, shape: &mut Shape) -> ValidationResult<Self> {
        let point = self.point().merge_into(shape)?;
        shape.get_handle_or_insert(Vertex { point })
    }
}

impl Object for Edge {
    fn merge_into(self, shape: &mut Shape) -> ValidationResult<Self> {
        let curve = self.curve().merge_into(shape)?;

        // Can be cleaned up using `try_map`, once that is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.try_map
        let vertices = self
            .vertices()
            .map(|vertices| vertices.map(|vertex| vertex.merge_into(shape)));
        let vertices = match vertices {
            Some([a, b]) => Some([a?, b?]),
            None => None,
        };

        shape.get_handle_or_insert(Edge { curve, vertices })
    }
}

impl Object for Cycle {
    fn merge_into(self, shape: &mut Shape) -> ValidationResult<Self> {
        let mut edges = Vec::new();
        for edge in self.edges() {
            let edge = edge.merge_into(shape)?;
            edges.push(edge);
        }

        shape.get_handle_or_insert(Cycle { edges })
    }
}

impl Object for Face {
    fn merge_into(self, shape: &mut Shape) -> ValidationResult<Self> {
        // TASK: This API needs to work for tri-rep faces too, so the following
        //       won't work.

        let surface = self.surface().merge_into(shape)?;

        let mut exteriors = Vec::new();
        for cycle in self.exteriors() {
            let cycle = cycle.merge_into(shape)?;
            exteriors.push(cycle);
        }

        let mut interiors = Vec::new();
        for cycle in self.interiors() {
            let cycle = cycle.merge_into(shape)?;
            interiors.push(cycle);
        }

        shape.get_handle_or_insert(Face::Face {
            surface,
            exteriors,
            interiors,
            color: self.color,
        })
    }
}

mod private {
    pub trait Sealed {}
}
