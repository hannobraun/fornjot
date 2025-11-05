use crate::handle::Handle;

use super::{curve::Curve, vertex::Vertex};

/// # A half-edge that is part of a face's boundary
///
/// Half-edges are unique to a face, but there is also a (so far) implicit
/// concept of "edges" which are shared between faces.
///
/// It might be advantageous to make this explicit, by having an `Edge`
/// struct that is referred to as `Handle<Edge>`. Not only would this better
/// communicate the intent of the design, it would also allow for easier
/// caching of approximations, or inferring if a half-edge is internal.
///
/// `Edge` could refer to its two bounding vertices (as `Handle<Vertex>`),
/// and the curve (as `Handle<Curve>`). Then `HalfEdge` would be reduced to
/// a `Handle<Edge>`, a field to specify the direction of the half-edge, and
/// possibly its current `is_internal` field.
#[derive(Debug)]
pub struct HalfEdge {
    pub curve: Handle<Curve>,
    pub start: Handle<Vertex>,
    pub is_internal: bool,
}
