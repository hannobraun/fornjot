use std::fmt;

use crate::storage::{Handle, HandleWrapper};

use super::{Curve, GlobalCurve, GlobalVertex, Surface, Vertex};

/// A half-edge
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdge {
    vertices: [Handle<Vertex>; 2],
    global_form: Handle<GlobalEdge>,
}

impl HalfEdge {
    /// Create a new instance of `HalfEdge`
    pub fn new(
        vertices: [Handle<Vertex>; 2],
        global_form: Handle<GlobalEdge>,
    ) -> Self {
        Self {
            vertices,
            global_form,
        }
    }

    /// Access the curve that defines the half-edge's geometry
    pub fn curve(&self) -> &Handle<Curve> {
        let [vertex, _] = self.vertices();
        vertex.curve()
    }

    /// Access the vertices that bound the half-edge on the curve
    pub fn vertices(&self) -> &[Handle<Vertex>; 2] {
        &self.vertices
    }

    /// Access the vertex at the back of the half-edge
    pub fn back(&self) -> &Handle<Vertex> {
        let [back, _] = self.vertices();
        back
    }

    /// Access the vertex at the front of the half-edge
    pub fn front(&self) -> &Handle<Vertex> {
        let [_, front] = self.vertices();
        front
    }

    /// Access the surface that the half-edge's curve is defined in
    pub fn surface(&self) -> &Handle<Surface> {
        self.curve().surface()
    }

    /// Access the global form of this half-edge
    pub fn global_form(&self) -> &Handle<GlobalEdge> {
        &self.global_form
    }
}

impl fmt::Display for HalfEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b] = self.vertices().clone().map(|vertex| vertex.position());
        write!(f, "edge from {:?} to {:?}", a, b)?;
        write!(f, " on {:?}", self.curve().global_form())?;

        Ok(())
    }
}

/// An edge, defined in global (3D) coordinates
///
/// In contract to [`HalfEdge`], `GlobalEdge` is undirected, meaning it has no
/// defined direction, and its vertices have no defined order. This means it can
/// be used to determine whether two [`HalfEdge`]s map to the same `GlobalEdge`,
/// regardless of their direction.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalEdge {
    curve: HandleWrapper<GlobalCurve>,
    vertices: VerticesInNormalizedOrder,
}

impl GlobalEdge {
    /// Create a new instance
    ///
    /// The order of `vertices` is irrelevant. Two `GlobalEdge`s with the same
    /// `curve` and `vertices` will end up being equal, regardless of the order
    /// of `vertices` here.
    pub fn new(
        curve: impl Into<HandleWrapper<GlobalCurve>>,
        vertices: [Handle<GlobalVertex>; 2],
    ) -> Self {
        let curve = curve.into();
        let (vertices, _) = VerticesInNormalizedOrder::new(vertices);

        Self { curve, vertices }
    }

    /// Access the curve that defines the edge's geometry
    ///
    /// The edge can be a segment of the curve that is bounded by two vertices,
    /// or if the curve is continuous (i.e. connects to itself), the edge could
    /// be defined by the whole curve, and have no bounding vertices.
    pub fn curve(&self) -> &Handle<GlobalCurve> {
        &self.curve
    }

    /// Access the vertices that bound the edge on the curve
    ///
    /// As the name indicates, the order of the returned vertices is normalized
    /// and might not match the order of the vertices that were passed to
    /// [`GlobalEdge::new`]. You must not rely on the vertices being in any
    /// specific order.
    pub fn vertices(&self) -> &VerticesInNormalizedOrder {
        &self.vertices
    }
}

/// The vertices of a [`GlobalEdge`]
///
/// Since [`GlobalEdge`] is the single global representation of an edge in
/// global space, it must normalize the order of its vertices. Otherwise, it is
/// possible to construct two [`GlobalEdge`] instances that are meant to
/// represent the same edge, but aren't equal.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct VerticesInNormalizedOrder([Handle<GlobalVertex>; 2]);

impl VerticesInNormalizedOrder {
    /// Construct a new instance of `VerticesInNormalizedOrder`
    ///
    /// The provided vertices can be in any order. The returned `bool` value
    /// indicates whether the normalization changed the order of the vertices.
    pub fn new([a, b]: [Handle<GlobalVertex>; 2]) -> (Self, bool) {
        if a < b {
            (Self([a, b]), false)
        } else {
            (Self([b, a]), true)
        }
    }

    /// Access the vertices
    ///
    /// The vertices in the returned array will be in normalized order.
    pub fn access_in_normalized_order(&self) -> [Handle<GlobalVertex>; 2] {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        builder::HalfEdgeBuilder, objects::Objects, partial::HasPartial,
    };

    use super::HalfEdge;

    #[test]
    fn global_edge_equality() -> anyhow::Result<()> {
        let objects = Objects::new();

        let surface = objects.surfaces.xy_plane();

        let a = [0., 0.];
        let b = [1., 0.];

        let a_to_b = HalfEdge::partial()
            .update_as_line_segment_from_points(surface.clone(), [a, b])
            .build(&objects)?;
        let b_to_a = HalfEdge::partial()
            .update_as_line_segment_from_points(surface, [b, a])
            .build(&objects)?;

        assert_eq!(a_to_b.global_form(), b_to_a.global_form());
        Ok(())
    }
}
