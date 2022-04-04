use fj_math::{Point, Scalar, Vector};

use crate::{
    geometry::{Circle, Curve, Line, Surface},
    shape::{Handle, Shape, ValidationResult},
};

use super::{Cycle, Edge, Face, Vertex};

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
    pub fn build_from_point(
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
    pub fn build_circle(self, radius: Scalar) -> ValidationResult<Edge> {
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
    pub fn build_line_segment_from_points(
        self,
        vertices: [impl Into<Point<3>>; 2],
    ) -> ValidationResult<Edge> {
        // Can be cleaned up with `try_map`, once that is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.try_map
        let vertices = vertices
            .map(|point| Vertex::builder(self.shape).build_from_point(point));
        let vertices = match vertices {
            [Ok(a), Ok(b)] => Ok([a, b]),
            [Err(err), _] | [_, Err(err)] => Err(err),
        }?;

        let edge = self.build_line_segment_from_vertices(vertices)?;

        Ok(edge)
    }

    /// Build a line segment from two vertices
    pub fn build_line_segment_from_vertices(
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

/// API for building a [`Cycle`]
pub struct CycleBuilder<'r> {
    shape: &'r mut Shape,
}

impl<'r> CycleBuilder<'r> {
    /// Construct a new instance of `CycleBuilder`
    pub fn new(shape: &'r mut Shape) -> Self {
        Self { shape }
    }

    /// Build a polygon from a list of points
    pub fn build_polygon(
        self,
        points: impl IntoIterator<Item = impl Into<Point<3>>>,
    ) -> ValidationResult<Cycle> {
        // A polygon is closed, so we need to add the first point at the end
        // again, for the next step.
        let mut points: Vec<_> = points.into_iter().map(Into::into).collect();
        if let Some(point) = points.first().cloned() {
            points.push(point);
        }

        let mut edges = Vec::new();
        for ab in points.windows(2) {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable.
            let points = [ab[0], ab[1]];

            let edge = Edge::builder(self.shape)
                .build_line_segment_from_points(points)?;
            edges.push(edge);
        }

        self.shape.insert(Cycle { edges })
    }
}

/// API for building a [`Face`]
pub struct FaceBuilder<'r> {
    surface: Surface,
    exterior: Option<Vec<Point<3>>>,

    shape: &'r mut Shape,
}

impl<'r> FaceBuilder<'r> {
    /// Construct a new instance of `FaceBuilder`
    pub fn new(surface: Surface, shape: &'r mut Shape) -> Self {
        Self {
            surface,
            exterior: None,

            shape,
        }
    }

    /// Make the exterior or the face a polygon
    pub fn with_exterior_polygon(
        self,
        points: impl IntoIterator<Item = impl Into<Point<3>>>,
    ) -> Self {
        Self {
            surface: self.surface,
            exterior: Some(points.into_iter().map(Into::into).collect()),

            shape: self.shape,
        }
    }

    /// Build the face
    pub fn build(self) -> ValidationResult<Face> {
        let surface = self.shape.insert(self.surface)?;

        let exteriors = match self.exterior {
            Some(points) => {
                let cycle = Cycle::builder(self.shape).build_polygon(points)?;
                vec![cycle]
            }
            None => Vec::new(),
        };

        self.shape.insert(Face::Face {
            surface,
            exteriors,
            interiors: Vec::new(),
            color: [255, 0, 0, 255],
        })
    }
}
