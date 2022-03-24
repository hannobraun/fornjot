use fj_math::Point;

use crate::shape::{Shape, ValidationResult};

use super::Vertex;

/// API for building a [`Vertex`]
pub struct VertexBuilder<'r> {
    shape: &'r mut Shape,
}

impl<'r> VertexBuilder<'r> {
    /// Construct a new instance of `VertexBuilder`
    pub fn new(shape: &'r mut Shape) -> Self {
        Self { shape }
    }

    /// Build a [`Vertex`] from a point
    pub fn from_point(
        self,
        point: impl Into<Point<3>>,
    ) -> ValidationResult<Vertex> {
        let point = self.shape.geometry().add_point(point.into());
        let vertex = self.shape.topology().add_vertex(Vertex { point })?;

        Ok(vertex)
    }
}
