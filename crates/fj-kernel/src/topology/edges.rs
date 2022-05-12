use std::hash::{Hash, Hasher};

use crate::{
    geometry::{self, Curve},
    shape::{Handle, Shape},
};

use super::{vertices::Vertex, EdgeBuilder};

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
    pub vertices: Option<[EdgeVertex; 2]>,
}

impl Edge {
    /// Construct an instance of `Edge`
    pub fn new(
        curve: Handle<Curve>,
        vertices: Option<[Handle<Vertex>; 2]>,
    ) -> Self {
        let vertices = vertices.map(|vertices| {
            vertices.map(|handle| {
                let local =
                    curve.get().point_to_curve_coords(handle.get().point());
                EdgeVertex { handle, local }
            })
        });

        Self { curve, vertices }
    }

    /// Build an edge using the [`EdgeBuilder`] API
    pub fn builder(shape: &mut Shape) -> EdgeBuilder {
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
        self.vertices
            .as_ref()
            .map(|[a, b]| [a.handle.get(), b.handle.get()])
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

/// A vertex of an edge
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct EdgeVertex {
    /// The handle to the vertex
    pub handle: Handle<Vertex>,

    /// The local representation of the vertex
    ///
    /// Represents the vertex in terms of the coordinates of the edge's curve.
    pub local: geometry::Point<1>,
}
