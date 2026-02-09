use fj_math::{Point, Scalar, Triangle};

use crate::new::topology::Handle;

/// # A vertex
///
/// Vertices are the lowest-level topological primitive. The only piece of data
/// they contain is the point that defines their geometrical representation.
///
/// Even though each vertex corresponds to a point, not every point corresponds
/// to a vertex. There are many points that approximate the curvature of
/// [`HalfEdge`]s and [`Face`]s, which do not correspond to a vertex.
///
/// Vertices form the boundary of [`HalfEdge`]s.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Vertex {
    /// # The vertex's geometrical representation
    pub point: Point<3>,
}

impl<S> From<[S; 3]> for Vertex
where
    S: Into<Scalar>,
{
    fn from(point: [S; 3]) -> Self {
        let point = Point::from(point.map(|s| s.into()));
        Self { point }
    }
}

/// # A half-edge
///
/// Half-edges make up the boundary of [`Face`]s.
#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HalfEdge {
    /// # The two vertices that bound the half-edge
    pub boundary: [Handle<Vertex>; 2],

    /// # The points that approximate the half-edge
    ///
    /// These points approximate the half-edge _between_ the boundary vertices.
    /// So this might be empty, if the half-edge is a line segment.
    pub approx: Vec<Point<3>>,
}

/// # A face
///
/// Faces make up the boundary of [`Solid`]s.
#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Face {
    /// # The half-edges that bound the face
    pub boundary: Vec<Handle<HalfEdge>>,

    /// # The triangles that approximate the face
    pub approx: Vec<Triangle<3>>,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Solid {
    pub boundary: Vec<Handle<Face>>,
}
