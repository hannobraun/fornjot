use crate::{
    math::{Point, Scalar, Triangle},
    new::topology::{Handle, Store},
};

/// # A vertex
///
/// Vertices are the lowest-level topological primitive. The only piece of data
/// they contain is the point that defines their geometrical representation.
///
/// Even though each vertex corresponds to a point, not every point corresponds
/// to a vertex. There are many points that approximate the curvature of
/// [`HalfEdge`]s and [`HalfFace`]s, which do not correspond to a vertex.
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
/// Half-edges make up the boundary of [`HalfFace`]s.
#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HalfEdge {
    /// # The edge that defines this half-edge's boundary and approximation
    ///
    /// This edge must be shared with all other coincident half-edges.
    pub edge: Handle<Edge>,

    /// # The orientation of the half-edge
    ///
    /// This orientation is defined in terms of the nominal orientation of the
    /// half-edge's edge.
    pub orientation: Orientation,
}

impl HalfEdge {
    /// # Access the half-edge's boundary
    pub fn boundary(&self, edges: &Store<Edge>) -> [Handle<Vertex>; 2] {
        let [a, b] = edges[self.edge].boundary;

        match self.orientation {
            Orientation::Nominal => [a, b],
            Orientation::AntiNominal => [b, a],
        }
    }

    /// # Access the half-edge's approximation
    pub fn approx(&self, edges: &Store<Edge>) -> Vec<Point<3>> {
        let mut approx = edges[self.edge].approx.clone();

        if let Orientation::AntiNominal = self.orientation {
            approx.reverse();
        }

        approx
    }
}

/// # An edge
///
/// Edges are one-dimensional structures that exist where multiple [`HalfFace`]s
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
/// they have a nominal orientation, since the data they contain is directed.
/// Half-edges define their own orientation as the same or the opposite of the
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

/// # A half-face
///
/// Half-faces make up the boundary of [`Solid`]s. Directed half-faces relate to
/// undirected faces, like directed half-edges relate to undirected edges: Where
/// two half-faces coincide, they must reference the same face.
#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HalfFace {
    /// # The half-edges that bound the half-face
    pub boundary: Vec<Handle<HalfEdge>>,

    /// # The face that defines this half-face's approximation
    pub face: Handle<Face>,

    /// # The orientation of the half-face
    ///
    /// This orientation is defined in terms of the nominal orientation of the
    /// half-face's face.
    pub orientation: Orientation,
}

impl HalfFace {
    /// # Access the half-face's approximation
    pub fn approx(&self, faces: &Store<Face>) -> Vec<Triangle<3>> {
        let approx = &faces[self.face].approx;

        if let Orientation::Nominal = self.orientation {
            approx.clone()
        } else {
            approx.iter().map(|triangle| triangle.reverse()).collect()
        }
    }
}

/// # A face
///
/// Faces are a two-dimensional structure that exist where multiple
/// [`HalfFace`]s meet.
///
/// Faces are closely related to half-faces, in a similar way that [`Edge`]s are
/// closely related to [`HalfEdge`]s. The boundary of a [`Solid`] is made up of
/// directed half-faces, while faces are shared by multiple coincident
/// half-faces.
///
/// Faces may be shared by half-faces from different solids, where those solids
/// touch, or by half-faces from the same solid, where that solid touches
/// itself.
///
/// In principle, faces are undirected, in contrast to half-faces. In practice,
/// they have a nominal orientation, since their approximation is directed.
/// Half-faces define their own orientation as the same or the opposite of the
/// face they reference.
#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Face {
    /// # The triangles that approximate the face
    pub approx: Vec<Triangle<3>>,
}

/// # A solid body
#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Solid {
    /// # The half-faces that bound the solid
    pub boundary: Vec<Handle<HalfFace>>,
}

/// # An orientation, in terms of a context-dependent nominal orientation
#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum Orientation {
    /// # The orientation is the same as the nominal orientation
    Nominal,

    /// # The orientation is the opposite of the nominal orientation
    AntiNominal,
}

impl Orientation {
    /// # Return the reverse orientation
    pub fn reverse(&self) -> Self {
        match self {
            Self::Nominal => Self::AntiNominal,
            Self::AntiNominal => Self::Nominal,
        }
    }
}
