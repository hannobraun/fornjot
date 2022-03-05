use kiddo::KdTree;

use crate::math::{Point, Scalar};

use super::topology::{edges::Edges, faces::Faces, vertices::Vertex};

/// The boundary representation of a shape
///
/// # Implementation note
///
/// The goal for `Shape` is to enforce full self-consistency, through the API it
/// provides. Steps have been made in that direction, but right now, the API is
/// still full of holes, forcing callers to just be careful for the time being.
#[derive(Clone, Debug)]
pub struct Shape {
    vertices: VerticesInner,

    pub edges: Edges,
    pub faces: Faces,
}

impl Shape {
    /// Construct a new shape
    pub fn new() -> Self {
        Self {
            vertices: VerticesInner::new(),
            edges: Edges { cycles: Vec::new() },
            faces: Faces(Vec::new()),
        }
    }

    /// Access and modify the shape's vertices
    pub fn vertices(&mut self) -> Vertices {
        Vertices {
            vertices: &mut self.vertices,
        }
    }
}

/// The vertices of a shape
pub struct Vertices<'r> {
    vertices: &'r mut VerticesInner,
}

impl Vertices<'_> {
    /// Create a vertex
    ///
    /// The caller must make sure to uphold all rules regarding vertex
    /// uniqueness.
    ///
    /// # Implementation note
    ///
    /// This method is the only means to create `Vertex` instances, outside of
    /// unit tests. That puts this method is in a great position to enforce
    /// vertex uniqueness rules, instead of requiring the user to uphold those.
    pub fn create(&mut self, point: Point<3>) -> Vertex {
        self.vertices
            .add(&point.into(), point)
            .expect("Error adding vertex");
        Vertex(point)
    }
}

type VerticesInner = KdTree<Scalar, Point<3>, 3>;
