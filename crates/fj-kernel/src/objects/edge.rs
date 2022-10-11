use std::fmt;

use pretty_assertions::{assert_eq, assert_ne};

use crate::storage::{Handle, HandleWrapper};

use super::{Curve, GlobalCurve, GlobalVertex, Vertex};

/// A half-edge
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdge {
    vertices: [Vertex; 2],
    global_form: GlobalEdge,
}

impl HalfEdge {
    /// Create a new instance of `HalfEdge`
    ///
    /// # Panics
    ///
    /// Panics, if the provided `vertices` are not defined on the same curve as
    /// `curve`.
    ///
    /// Panics, if the provided [`GlobalEdge`] instance doesn't refer to the
    /// same [`GlobalCurve`] and [`GlobalVertex`] instances that the other
    /// objects that are passed refer to.
    ///
    /// Panics, if the provided vertices are coincident on the curve. If they
    /// were, the edge would have no length, and thus not be valid. (It is
    /// perfectly fine for global forms of the the vertices to be coincident.
    /// That would just mean, that ends of the edge connect to each other.)
    pub fn new([a, b]: [Vertex; 2], global_form: GlobalEdge) -> Self {
        // Make sure `curve` and `vertices` match.
        assert_eq!(
            a.curve().id(),
            b.curve().id(),
            "An edge's vertices must be defined in the same curve",
        );

        let curve = a.curve();

        // Make sure `curve` and `vertices` match `global_form`.
        assert_eq!(
            curve.global_form().id(),
            global_form.curve().id(),
            "The global form of a half-edge's curve must match the curve of \
            the half-edge's global form"
        );
        assert_eq!(
            &VerticesInNormalizedOrder::new(
                [&a, &b].map(|vertex| vertex.global_form().clone())
            ),
            global_form.vertices(),
            "The global forms of a half-edge's vertices must match the \
            vertices of the half-edge's global form"
        );

        // Make sure that the edge vertices are not coincident on the curve.
        assert_ne!(
            a.position(),
            b.position(),
            "Vertices of an edge must not be coincident on curve"
        );

        Self {
            vertices: [a, b],
            global_form,
        }
    }

    /// Access the curve that defines the half-edge's geometry
    pub fn curve(&self) -> &Handle<Curve> {
        let [vertex, _] = self.vertices();
        vertex.curve()
    }

    /// Access the vertices that bound the half-edge on the curve
    pub fn vertices(&self) -> &[Vertex; 2] {
        &self.vertices
    }

    /// Access the global form of this half-edge
    pub fn global_form(&self) -> &GlobalEdge {
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
        let vertices = VerticesInNormalizedOrder::new(vertices);
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
    /// The provided vertices can be in any order.
    pub fn new([a, b]: [Handle<GlobalVertex>; 2]) -> Self {
        let vertices = if a < b { [a, b] } else { [b, a] };
        Self(vertices)
    }

    /// Access the vertices
    ///
    /// The vertices in the returned array will be in normalized order.
    pub fn access_in_normalized_order(&self) -> &[Handle<GlobalVertex>; 2] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{objects::Objects, partial::HasPartial};

    use super::HalfEdge;

    #[test]
    fn global_edge_equality() {
        let objects = Objects::new();

        let surface = objects.surfaces.xy_plane();

        let a = [0., 0.];
        let b = [1., 0.];

        let a_to_b = HalfEdge::partial()
            .with_surface(Some(surface.clone()))
            .as_line_segment_from_points([a, b])
            .build(&objects);
        let b_to_a = HalfEdge::partial()
            .with_surface(Some(surface))
            .as_line_segment_from_points([b, a])
            .build(&objects);

        assert_eq!(a_to_b.global_form(), b_to_a.global_form());
    }
}
