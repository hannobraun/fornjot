//! Convenient API to build objects

use fj_math::{Circle, Line, Point, Scalar, Vector};

use crate::{
    objects::{Curve, Cycle, Edge, Face, Surface, Vertex, VerticesOfEdge},
    shape::{Handle, LocalForm, Shape},
};

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
    ) -> Handle<Vertex> {
        let point = point.into();
        self.shape.get_handle_or_insert(Vertex { point })
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
    pub fn build_circle(self, radius: Scalar) -> LocalForm<Edge<2>, Edge<3>> {
        let curve_local = Curve::Circle(Circle {
            center: Point::origin(),
            a: Vector::from([radius, Scalar::ZERO]),
            b: Vector::from([Scalar::ZERO, radius]),
        });
        let curve_canonical = Curve::Circle(Circle {
            center: Point::origin(),
            a: Vector::from([radius, Scalar::ZERO, Scalar::ZERO]),
            b: Vector::from([Scalar::ZERO, radius, Scalar::ZERO]),
        });

        let edge_local = Edge {
            curve: LocalForm::new(curve_local, curve_canonical),
            vertices: VerticesOfEdge::none(),
        };
        let edge_canonical = Edge {
            curve: LocalForm::canonical_only(curve_canonical),
            vertices: VerticesOfEdge::none(),
        };

        LocalForm::new(edge_local, edge_canonical)
    }

    /// Build a line segment from two points
    pub fn build_line_segment_from_points(
        self,
        vertices: [impl Into<Point<3>>; 2],
    ) -> Handle<Edge<3>> {
        let vertices = vertices.map(|point| {
            let point = point.into();
            Vertex { point }
        });

        self.build_line_segment_from_vertices(vertices)
    }

    /// Build a line segment from two vertices
    pub fn build_line_segment_from_vertices(
        self,
        [a, b]: [Vertex; 2],
    ) -> Handle<Edge<3>> {
        let curve = {
            let points = [a, b].map(|vertex| vertex.point);
            Curve::Line(Line::from_points(points))
        };

        let vertices = [
            LocalForm::new(Point::from([0.]), a),
            LocalForm::new(Point::from([1.]), b),
        ];

        self.shape.get_handle_or_insert(Edge {
            curve: LocalForm::canonical_only(curve),
            vertices: VerticesOfEdge::from_vertices(vertices),
        })
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
    ) -> LocalForm<Cycle<2>, Cycle<3>> {
        let mut points: Vec<_> = points.into_iter().map(Into::into).collect();

        // A polygon is closed, so we need to add the first point at the end
        // again, for the next step.
        if let Some(point) = points.first().cloned() {
            points.push(point);
        }

        let mut edges = Vec::new();
        for points in points.windows(2) {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable.
            let points = [points[0], points[1]];

            let points_canonical = points
                .map(|point| self.surface.point_from_surface_coords(point));
            let edge_canonical = Edge::builder(self.shape)
                .build_line_segment_from_points(points_canonical)
                .get();

            let edge_local = Edge {
                curve: LocalForm::new(
                    Curve::Line(Line::from_points(points)),
                    edge_canonical.curve.canonical(),
                ),
                vertices: edge_canonical.vertices.clone(),
            };

            edges.push(LocalForm::new(edge_local, edge_canonical));
        }

        let local = Cycle {
            edges: edges.clone(),
        };

        let edges_canonical = edges.into_iter().map(|edge| edge.canonical());
        let canonical = Cycle::new(edges_canonical);

        LocalForm::new(local, canonical)
    }
}

/// API for building a [`Face`]
#[must_use]
pub struct FaceBuilder<'r> {
    surface: Surface,
    exterior: Option<Vec<Point<2>>>,
    interiors: Vec<Vec<Point<2>>>,
    color: Option<[u8; 4]>,

    shape: &'r mut Shape,
}

impl<'r> FaceBuilder<'r> {
    /// Construct a new instance of `FaceBuilder`
    pub fn new(surface: Surface, shape: &'r mut Shape) -> Self {
        Self {
            surface,
            exterior: None,
            interiors: Vec::new(),
            color: None,

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

    /// Define the color of the face
    pub fn with_color(mut self, color: [u8; 4]) -> Self {
        self.color = Some(color);
        self
    }

    /// Build the face
    pub fn build(self) -> Handle<Face> {
        let surface = self.shape.get_handle_or_insert(self.surface);

        let mut exteriors = Vec::new();
        if let Some(points) = self.exterior {
            let cycle =
                Cycle::builder(self.surface, self.shape).build_polygon(points);
            exteriors.push(cycle);
        }

        let mut interiors = Vec::new();
        for points in self.interiors {
            let cycle =
                Cycle::builder(self.surface, self.shape).build_polygon(points);
            interiors.push(cycle);
        }

        let color = self.color.unwrap_or([255, 0, 0, 255]);

        self.shape.get_handle_or_insert(Face::new(
            surface, exteriors, interiors, color,
        ))
    }
}
