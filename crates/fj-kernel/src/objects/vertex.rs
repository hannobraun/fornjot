use fj_math::Point;

/// A vertex
///
/// `Vertex` is defined in terms of a 1-dimensional position on a curve. If you
/// need the 3D position of a vertex, you can use [`Vertex::global`], to get
/// access of the global form of a vertex ([`GlobalVertex`]).
///
/// # Implementation Note
///
/// Since `Vertex` is defined in terms of the curve it lies on, a reference to
/// that curve should be available here. As of this writing, this reference
/// still lives in [`Edge`].
///
/// [`Edge`]: super::Edge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vertex {
    position: Point<1>,
    global: GlobalVertex,
}

impl Vertex {
    /// Construct an instance of `Vertex`
    pub fn new(position: impl Into<Point<1>>, global: GlobalVertex) -> Self {
        let position = position.into();
        Self { position, global }
    }

    /// The position of the vertex on the curve
    pub fn position(&self) -> Point<1> {
        self.position
    }

    /// The global form of this vertex
    pub fn global(&self) -> &GlobalVertex {
        &self.global
    }
}

/// A vertex, defined in global (3D) coordinates
///
/// This struct exists to distinguish between vertices and points at the type
/// level. This is a relevant distinction, as vertices are part of a shape that
/// help define its topology.
///
/// Points, on the other hand, might be used to approximate a shape for various
/// purposes, without presenting any deeper truth about the shape's structure.
///
/// # Validation
///
/// Vertices must be unique within a shape, meaning an identical vertex must not
/// exist in the same shape. In the context of vertex uniqueness, points that
/// are close to each other are considered identical. The minimum distance
/// between distinct vertices can be configured using the respective field in
/// [`ValidationConfig`].
///
/// [`ValidationConfig`]: crate::validation::ValidationConfig
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalVertex {
    position: Point<3>,
}

impl GlobalVertex {
    /// Construct a `Vertex` from a point
    pub fn from_position(position: impl Into<Point<3>>) -> Self {
        let position = position.into();
        Self { position }
    }

    /// The position of the vertex
    pub fn position(&self) -> Point<3> {
        self.position
    }
}
