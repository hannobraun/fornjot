use std::hash::{Hash, Hasher};

use crate::{
    geometry::Curve,
    shape::{Handle, Shape},
};

use super::{vertices::Vertex, EdgeBuilder};

/// A cycle of connected edges
///
/// The end of each edge in the cycle must connect to the beginning of the next
/// edge. The end of the last edge must connect to the beginning of the first
/// one.
///
/// # Equality
///
/// Please refer to [`crate::kernel::topology`] for documentation on the
/// equality of topological objects.
///
/// # Validation
///
/// A cycle that is part of a [`Shape`] must be structurally sound. That means
/// the edges it refers to, must be part of the same shape.
#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub struct Cycle {
    /// The edges that make up the cycle
    pub edges: Vec<Handle<Edge>>,
}

impl Cycle {
    /// Access the edges that this cycle refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges.iter().map(|handle| handle.get())
    }
}

impl PartialEq for Cycle {
    fn eq(&self, other: &Self) -> bool {
        self.edges().eq(other.edges())
    }
}

impl Hash for Cycle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for edge in self.edges() {
            edge.hash(state);
        }
    }
}

/// An edge of a shape
///
/// # Equality
///
/// Please refer to [`crate::kernel::topology`] for documentation on the
/// equality of topological objects.
///
/// # Validation
///
/// An edge that is part of a [`Shape`] must be structurally sound. That means
/// the curve and any vertices that it refers to, must be part of the same
/// shape.
#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub struct Edge {
    /// Access the curve that defines the edge's geometry
    ///
    /// The edge can be a segment of the curve that is bounded by two vertices,
    /// or if the curve is continuous (i.e. connects to itself), the edge could
    /// be defined by the whole curve, and have no bounding vertices.
    pub curve: Handle<Curve>,

    /// Access the vertices that bound the edge on the curve
    ///
    /// If there are no such vertices, that means that both the curve and the
    /// edge are continuous (i.e. connected to themselves).
    ///
    /// # Implementation note
    ///
    /// Since these vertices bound the edge, they must lie on the curve. This
    /// isn't enforced at all, however. It would make sense to store 1D vertices
    /// here, and indeed, this was the case in the past.
    ///
    /// It got in the way of some work, however, so it made sense to simplify
    /// it by storing 3D vertices. It will probably make sense to revert this
    /// and store 1D vertices again, at some point.
    pub vertices: Option<[Handle<Vertex>; 2]>,
}

impl Edge {
    /// Build an edge using the [`EdgeBuilder`] API
    pub fn build(shape: &mut Shape) -> EdgeBuilder {
        EdgeBuilder::new(shape)
    }

    /// Access the curve that the edge refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`].
    pub fn curve(&self) -> Curve {
        self.curve.get()
    }

    /// Access the vertices that the edge refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn vertices(&self) -> Option<[Vertex; 2]> {
        self.vertices.as_ref().map(|[a, b]| [a.get(), b.get()])
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.curve() == other.curve() && self.vertices() == other.vertices()
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.curve().hash(state);
        self.vertices().hash(state);
    }
}
