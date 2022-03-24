use fj_math::{Point, Scalar, Vector};

use crate::{
    geometry::{Circle, Curve, Line},
    shape::{Handle, Shape, ValidationResult},
};

use super::{Edge, Vertex};

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
    ///
    /// If an identical point or vertex are already part of the shape, those
    /// objects are re-used.
    pub fn from_point(
        self,
        point: impl Into<Point<3>>,
    ) -> ValidationResult<Vertex> {
        let point = self.shape.get_handle_or_insert(point.into())?;
        let vertex = self.shape.get_handle_or_insert(Vertex { point })?;

        Ok(vertex)
    }
}

/// API for building an [`Edge`]
pub struct EdgeBuilder<'r> {
    shape: &'r mut Shape,
}

impl<'r> EdgeBuilder<'r> {
    /// Construct a new instance of `EdgeBuilder`
    pub fn new(shape: &'r mut Shape) -> Self {
        Self { shape }
    }

    /// Build a circle from a radius
    pub fn circle(self, radius: Scalar) -> ValidationResult<Edge> {
        let curve = self.shape.insert(Curve::Circle(Circle {
            center: Point::origin(),
            radius: Vector::from([radius, Scalar::ZERO]),
        }))?;
        let edge = self.shape.insert(Edge {
            curve,
            vertices: None,
        })?;

        Ok(edge)
    }

    /// Build a line segment from two points
    pub fn line_segment_from_points(
        self,
        vertices: [impl Into<Point<3>>; 2],
    ) -> ValidationResult<Edge> {
        // Can be cleaned up with `try_map`, once that is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.try_map
        let vertices =
            vertices.map(|point| Vertex::build(self.shape).from_point(point));
        let vertices = match vertices {
            [Ok(a), Ok(b)] => Ok([a, b]),
            [Err(err), _] | [_, Err(err)] => Err(err),
        }?;

        let edge = self.line_segment_from_vertices(vertices)?;

        Ok(edge)
    }

    /// Build a line segment from two vertices
    pub fn line_segment_from_vertices(
        self,
        vertices: [Handle<Vertex>; 2],
    ) -> ValidationResult<Edge> {
        let curve = self.shape.insert(Curve::Line(Line::from_points(
            vertices.clone().map(|vertex| vertex.get().point()),
        )))?;
        let edge = self.shape.insert(Edge {
            curve,
            vertices: Some(vertices),
        })?;

        Ok(edge)
    }
}
