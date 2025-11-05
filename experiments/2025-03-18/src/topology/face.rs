use fj_interop::{Tolerance, TriMesh};
use itertools::Itertools;

use crate::{extra::triangulate::triangulate_face, handle::Handle};

use super::{half_edge::HalfEdge, surface::Surface, vertex::Vertex};

#[derive(Debug)]
pub struct Face {
    pub surface: Handle<Surface>,

    /// # The half-edges that bound the face on the surface
    ///
    /// Half-edges are specific to one face. They are never shared with another
    /// one. Hence, we don't need a [`Handle`] here. We could instead own the
    /// `HalfEdge`s here directly.
    ///
    /// This is probably a good idea, as it would simplify the object graph.
    /// Though there's also another consideration to make here: while half-edges
    /// are unique to a face, there is a (so far) implicit concept of "edges"
    /// which are shared between faces.
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
    pub half_edges: Vec<Handle<HalfEdge>>,

    /// # Indicate whether the face is internal to the solid it bounds
    ///
    /// Solids are constructed such that they have a single boundary of properly
    /// connected faces. That means that a solid with internal cavities needs
    /// faces that connect its outer boundary with these inner cavities.
    ///
    /// A different way to view that is that such a solid does not have an outer
    /// shell, and inner cavities with separate shells. Instead, it has just one
    /// single shell that happens to touch itself in certain locations.
    ///
    /// This simplifies the representation of solids. They don't need multiple
    /// boundaries, just one. And the rules that need to be enforced for that
    /// one boundary are simpler. That comes at the cost at making the
    /// construction more complicated, but so far this seems like a good trade.
    ///
    /// Where the solid touches itself, the touching faces must be congruent.
    /// Such congruent faces are called internal. This flag tracks whether a
    /// face is internal or not.
    ///
    /// ## Alternative approach: faces and half-faces
    ///
    /// This approach is actually similar to how faces work. They too have only
    /// one single boundary, and have half-edges connecting the outer part of
    /// their boundary to any holes. In that case, I have the idea of explicitly
    /// distinguishing between owned half-edges that reference shared edges.
    ///
    /// The same concept would apply to faces. A solid would no longer be bound
    /// by faces, but by half-faces.
    ///
    /// Like half-edges, half-faces would be oriented (with defined front and
    /// back sides), they would reference an non-oriented full face, and they
    /// might or might not have a sibling of opposite orientation that
    /// references the same face.
    ///
    /// This would confer a number a number of advantages:
    ///
    /// - The structure of the topological graph would clearly communicate the
    ///   intent of its construction.
    /// - That would allow for strict validation of this aspect of the graph,
    ///   making sure that mistakes, like unintended coincidence of half-faces,
    ///   would be caught.
    /// - Unambiguously identifying faces would enable caching of their
    ///   approximations.
    ///
    /// There is one open question though: How would the coordinate systems of
    /// sibling half-faces relate to each other?
    ///
    /// - They could be set up so that a face has a single coordinate system.
    ///   That would simplify the sharing of approximations and possibly other
    ///   aspects.
    /// - The half-faces could have separate coordinate systems, so that the
    ///   x-axis always goes to the right and the y-axis to the top when viewing
    ///   the face from the front. That would simplify defining what the front
    ///   is for each half-face.
    pub is_internal: bool,
}

impl Face {
    pub fn new(
        surface: Handle<Surface>,
        half_edges: impl IntoIterator<Item = Handle<HalfEdge>>,
        is_internal: bool,
    ) -> Self {
        Self {
            surface,
            half_edges: half_edges.into_iter().collect(),
            is_internal,
        }
    }

    pub fn half_edges_with_end_vertex(
        &self,
    ) -> impl Iterator<Item = HalfEdgeWithEndVertex<'_>> {
        self.half_edges
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| HalfEdgeWithEndVertex {
                half_edge: a,
                end_vertex: &b.start,
            })
    }

    pub fn to_tri_mesh(&self, tolerance: impl Into<Tolerance>) -> TriMesh {
        triangulate_face(self, tolerance)
    }
}

pub struct HalfEdgeWithEndVertex<'r> {
    pub half_edge: &'r Handle<HalfEdge>,
    pub end_vertex: &'r Handle<Vertex>,
}
