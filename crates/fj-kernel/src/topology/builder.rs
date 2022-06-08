use fj_math::{Circle, Line, Point, Scalar, Vector};

use crate::{
    geometry::{Curve, Surface},
    shape::{Handle, LocalForm, Shape, ValidationResult},
};

use super::{Cycle, Edge, Face, Vertex, VerticesOfEdge};

/// API for building a [`Vertex`]
#[must_use]
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
        let point = point.into();
        let vertex = self.shape.get_handle_or_insert(Vertex { point })?;

        Ok(vertex)
    }
}

/// API for building an [`Edge`]
#[must_use]
pub struct EdgeBuilder<'r> {
    shape: &'r mut Shape,
}

impl<'r> EdgeBuilder<'r> {
    /// Construct a new instance of `EdgeBuilder`
    pub fn new(shape: &'r mut Shape) -> Self {
        Self { shape }
    }

    /// Build a circle from a radius
    pub fn build_circle(self, radius: Scalar) -> ValidationResult<Edge<3>> {
        let curve = self.shape.insert(Curve::Circle(Circle {
            center: Point::origin(),
            a: Vector::from([radius, Scalar::ZERO, Scalar::ZERO]),
            b: Vector::from([Scalar::ZERO, radius, Scalar::ZERO]),
        }))?;
        let edge = self
            .shape
            .insert(Edge::new(curve, VerticesOfEdge::none()))?;

        Ok(edge)
    }

    /// Build a line segment from two points
    pub fn build_line_segment_from_points(
        self,
        vertices: [impl Into<Point<3>>; 2],
    ) -> ValidationResult<Edge<3>> {
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
        [a, b]: [Handle<Vertex>; 2],
    ) -> ValidationResult<Edge<3>> {
        let curve = {
            let points = [&a, &b].map(|vertex| vertex.get().point());
            let curve = Curve::Line(Line::from_points(points));
            self.shape.insert(curve)?
        };

        let vertices = [
            LocalForm::new(Point::from([0.]), a),
            LocalForm::new(Point::from([1.]), b),
        ];

        let edge = self.shape.insert(Edge::new(
            curve,
            VerticesOfEdge::from_vertices(vertices),
        ))?;

        Ok(edge)
    }
}

/// API for building a [`Cycle`]
#[must_use]
pub struct CycleBuilder<'r> {
    surface: Surface,
    shape: &'r mut Shape,
}

impl<'r> CycleBuilder<'r> {
    /// Construct a new instance of `CycleBuilder`
    pub fn new(surface: Surface, shape: &'r mut Shape) -> Self {
        Self { surface, shape }
    }

    /// Build a polygon from a list of points
    pub fn build_polygon(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> ValidationResult<Cycle<3>> {
        let mut points: Vec<_> = points
            .into_iter()
            .map(|point| self.surface.point_from_surface_coords(point))
            .collect();

        // A polygon is closed, so we need to add the first point at the end
        // again, for the next step.
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

        self.shape.insert(Cycle::new(edges))
    }
}

/// API for building a [`Face`]
#[must_use]
pub struct FaceBuilder<'r> {
    surface: Surface,
    exterior: Option<Vec<Point<2>>>,
    interiors: Vec<Vec<Point<2>>>,

    shape: &'r mut Shape,
}

impl<'r> FaceBuilder<'r> {
    /// Construct a new instance of `FaceBuilder`
    pub fn new(surface: Surface, shape: &'r mut Shape) -> Self {
        Self {
            surface,
            exterior: None,
            interiors: Vec::new(),

            shape,
        }
    }

    /// Make the exterior or the face a polygon
    pub fn with_exterior_polygon(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let points = points.into_iter().map(Into::into).collect();

        Self {
            exterior: Some(points),
            ..self
        }
    }

    /// Add an interior polygon to the face
    pub fn with_interior_polygon(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let points = points.into_iter().map(Into::into).collect();

        let mut interiors = self.interiors;
        interiors.push(points);

        Self { interiors, ..self }
    }

    /// Build the face
    pub fn build(self) -> ValidationResult<Face> {
        let surface = self.shape.insert(self.surface)?;

        let mut exteriors = Vec::new();
        if let Some(points) = self.exterior {
            let cycle = Cycle::builder(self.surface, self.shape)
                .build_polygon(points)?;
            exteriors.push(cycle);
        }

        let mut interiors = Vec::new();
        for points in self.interiors {
            let cycle = Cycle::builder(self.surface, self.shape)
                .build_polygon(points)?;
            interiors.push(cycle);
        }

        self.shape.insert(Face::new(
            surface,
            exteriors,
            interiors,
            [255, 0, 0, 255],
        ))
    }
}
