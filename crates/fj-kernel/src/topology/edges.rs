use std::hash::{Hash, Hasher};

use crate::{
    geometry::{self, Curve},
    shape::{Handle, LocalForm, Shape},
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
    pub curve: Handle<Curve<3>>,

    /// Access the vertices that bound the edge on the curve
    ///
    /// If there are no such vertices, that means that both the curve and the
    /// edge are continuous (i.e. connected to themselves).
    pub vertices: Option<[LocalForm<geometry::Point<1, 3>, Vertex<3>>; 2]>,
}

impl Edge {
    /// Construct an instance of `Edge`
    pub fn new(
        curve: Handle<Curve<3>>,
        vertices: Option<[Handle<Vertex<3>>; 2]>,
    ) -> Self {
        let vertices = vertices.map(|vertices| {
            vertices.map(|canonical| {
                let local =
                    curve.get().point_to_curve_coords(canonical.get().point());
                LocalForm { local, canonical }
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
    pub fn curve(&self) -> Curve<3> {
        self.curve.get()
    }

    /// Access the vertices that the edge refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn vertices(&self) -> Option<[Vertex<3>; 2]> {
        self.vertices
            .as_ref()
            .map(|[a, b]| [a.canonical.get(), b.canonical.get()])
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
