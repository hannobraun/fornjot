use crate::{
    math::{Point, Scalar, Triangle},
    new::topology::Handle,
};

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

impl HalfEdge {
    /// # Access the half-edge's boundary
    pub fn boundary(&self) -> [Handle<Vertex>; 2] {
        self.boundary
    }

    /// # Access the half-edge's approximation
    pub fn approx(&self) -> Vec<Point<3>> {
        self.approx.clone()
    }
}

/// # An edge
///
/// Edges are one-dimensional structures that exist where multiple [`Face`]s
/// meet.
///
/// Edges are distinct from, but closely related to [`HalfEdge`]s. While a
/// half-edge forms part of the boundary of a specific face, belonging only to
/// that face, and edge is shared by multiple faces.
///
/// Half-edges reference edges, which define their boundary and approximation.
/// Coincident half-edges must reference the same edge.
///
/// In principle, edges are undirected, in contrast to half-edges. In practice,
/// they have a nominal direction, since the data they contain is directed.
/// Half-edges define their own direction as the same or the opposite of the
/// edge they reference.
#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Edge {
    /// # The two vertices that bound the edge
    pub boundary: [Handle<Vertex>; 2],

    /// # The points that approximate the edge
    ///
    /// These points approximate the edge _between_ the boundary vertices. So
    /// this might be empty, if the edge is a line segment.
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

/// # A solid body
#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Solid {
    /// # The faces that bound the solid
    pub boundary: Vec<Handle<Face>>,
}

/// # An orientation, in terms of a context-dependent nominal orientation
#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum Orientation {
    /// # The orientation is the opposite of the nominal orientation
    AntiNominal,

    /// # The orientation is the same as the nominal orientation
    Nominal,
}
