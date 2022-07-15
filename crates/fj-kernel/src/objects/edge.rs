use std::fmt;

use fj_math::{Circle, Line, Point, Scalar, Vector};

use crate::local::Local;

use super::{Curve, GlobalVertex, Surface, Vertex};

/// An edge of a shape
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edge {
    /// Access the curve that defines the edge's geometry
    ///
    /// The edge can be a segment of the curve that is bounded by two vertices,
    /// or if the curve is continuous (i.e. connects to itself), the edge could
    /// be defined by the whole curve, and have no bounding vertices.
    pub curve: Local<Curve<2>>,

    /// Access the vertices that bound the edge on the curve
    ///
    /// If there are no such vertices, that means that both the curve and the
    /// edge are continuous (i.e. connected to themselves).
    pub vertices: VerticesOfEdge,
}

impl Edge {
    /// Create a circle from the given radius
    pub fn circle_from_radius(radius: Scalar) -> Self {
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

        Edge {
            curve: Local::new(curve_local, curve_canonical),
            vertices: VerticesOfEdge::none(),
        }
    }

    /// Create a line segment from two points
    pub fn line_segment_from_points(
        surface: &Surface,
        points: [impl Into<Point<2>>; 2],
    ) -> Self {
        let points = points.map(Into::into);

        let global_vertices = points.map(|position| {
            let position = surface.point_from_surface_coords(position);
            GlobalVertex::from_position(position)
        });

        let curve_local = Curve::Line(Line::from_points(points));
        let curve_canonical = {
            let points =
                global_vertices.map(|global_vertex| global_vertex.position());
            Curve::Line(Line::from_points(points))
        };

        let vertices = {
            let [a, b] = global_vertices;
            [
                Vertex::new(Point::from([0.]), a),
                Vertex::new(Point::from([1.]), b),
            ]
        };

        Self {
            curve: Local::new(curve_local, curve_canonical),
            vertices: VerticesOfEdge::from_vertices(vertices),
        }
    }

    /// Access this edge's curve
    pub fn curve(&self) -> Curve<3> {
        self.curve.global()
    }

    /// Access this edge's vertices
    pub fn vertices(&self) -> Option<[Vertex; 2]> {
        self.vertices.0
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.vertices() {
            Some(vertices) => {
                let [a, b] = vertices.map(|vertex| vertex.position());
                write!(f, "edge from {:?} to {:?}", a, b)?
            }
            None => write!(f, "continuous edge")?,
        }

        write!(f, " on {}", self.curve())?;

        Ok(())
    }
}

/// The vertices that bound an edge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct VerticesOfEdge(Option<[Vertex; 2]>);

impl VerticesOfEdge {
    /// Construct an instance of `VerticesOfEdge` from zero or two vertices
    pub fn new(vertices: Option<[Vertex; 2]>) -> Self {
        Self(vertices)
    }

    /// Construct an instance of `VerticesOfEdge` from two vertices
    pub fn from_vertices(vertices: [Vertex; 2]) -> Self {
        Self(Some(vertices))
    }

    /// Construct an instance of `VerticesOfEdge` without vertices
    pub fn none() -> Self {
        Self(None)
    }

    /// Determine whether the other instance has the same vertices
    ///
    /// The order of vertices is ignored.
    pub fn are_same(&self, other: &Self) -> bool {
        if let Some([a, b]) = self.0 {
            if let Some(other) = other.0 {
                return [a, b] == other || [b, a] == other;
            }
        }

        false
    }

    /// Access the two vertices
    ///
    /// # Panics
    ///
    /// Panics, if the edge has no vertices.
    pub fn expect_vertices(self) -> [Vertex; 2] {
        self.0.expect("Expected edge to have vertices")
    }

    /// Iterate over the vertices, if any
    pub fn iter(&self) -> impl Iterator<Item = &Vertex> {
        self.0.iter().flatten()
    }

    /// Reverse the order of vertices
    ///
    /// Makes sure that the local coordinates are still correct.
    pub fn reverse(self) -> Self {
        Self(self.0.map(|[a, b]| {
            [
                Vertex::new(-b.position(), b.global()),
                Vertex::new(-a.position(), a.global()),
            ]
        }))
    }

    /// Map each vertex using the provided function
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnMut(Vertex) -> Vertex,
    {
        Self(self.convert(f))
    }

    /// Convert each vertex using the provided function
    pub fn convert<F, T>(self, f: F) -> Option<[T; 2]>
    where
        F: FnMut(Vertex) -> T,
    {
        self.0.map(|vertices| vertices.map(f))
    }

    /// Convert each vertex using the provided fallible function
    pub fn try_convert<F, T, E>(self, f: F) -> Result<Option<[T; 2]>, E>
    where
        F: FnMut(Vertex) -> Result<T, E>,
    {
        // Can be cleaned up using `try_map`, once that is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.try_map
        let vertices: Option<[Result<_, E>; 2]> = self.convert(f);
        let vertices = match vertices {
            Some([a, b]) => Some([a?, b?]),
            None => None,
        };

        Ok(vertices)
    }
}
