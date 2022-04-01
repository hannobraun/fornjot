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
    pub fn from_point(
        self,
        point: impl Into<Point<3>>,
    ) -> ValidationResult<Vertex> {
        let point = self.shape.insert(point.into())?;
        let vertex = self.shape.topology().add_vertex(Vertex { point })?;

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
        let curve = self.shape.geometry().add_curve(Curve::Circle(Circle {
            center: Point::origin(),
            radius: Vector::from([radius, Scalar::ZERO]),
        }));
        let edge = self.shape.topology().add_edge(Edge {
            curve,
            vertices: None,
        })?;

        Ok(edge)
    }

    /// Build a line segment from two vertices
    pub fn line_segment_from_vertices(
        self,
        vertices: [Handle<Vertex>; 2],
    ) -> ValidationResult<Edge> {
        let curve =
            self.shape
                .geometry()
                .add_curve(Curve::Line(Line::from_points(
                    vertices.clone().map(|vertex| vertex.get().point()),
                )));
        let edge = self.shape.topology().add_edge(Edge {
            curve,
            vertices: Some(vertices),
        })?;

        Ok(edge)
    }
}
