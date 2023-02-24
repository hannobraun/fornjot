use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    objects::{Curve, GlobalVertex, SurfaceVertex},
    storage::Handle,
};

/// A directed edge, defined in a surface's 2D space
///
/// The concept of an "edge" in Fornjot is represented by two structs,
/// `HalfEdge` and [`GlobalEdge`]. `HalfEdge` has two attributes that make it
/// distinct from [`GlobalEdge`]:
///
/// - `HalfEdge` is directed, meaning it has a defined start and end vertex.
/// - `HalfEdge` is defined in the 2-dimensional space of a surface.
///
/// When multiple faces, which are bound by edges, are combined to form a solid,
/// the `HalfEdge`s that bound the face on the surface are then coincident with
/// the `HalfEdge`s of other faces, where those faces touch. Those coincident
/// `HalfEdge`s are different representations of the same edge. This edge is
/// represented by an instance of [`GlobalEdge`].
///
/// There are some requirements that a `HalfEdge` needs to uphold to be valid:
///
/// 1. Coincident `HalfEdge`s *must* refer to the same [`GlobalEdge`].
/// 2. `HalfEdge`s that are coincident, i.e. located in the same space, must
///    always be congruent. This means they must coincide *exactly*. The overlap
///    must be complete. None of the coincident `HalfEdge`s must overlap with
///    just a section of another.
///
/// That second requirement means that a `HalfEdge` might need to be split into
/// multiple smaller `HalfEdge`s that are each coincident with a `HalfEdge` in
/// another face.
///
/// # Implementation Note
///
/// There is no validation code that verifies whether coincident `HalfEdge`s
/// refer to the same [`GlobalEdge`] or not:
/// <https://github.com/hannobraun/Fornjot/issues/1594>
///
/// Conversely, there is no validation code to verify that coincident
/// `HalfEdge`s are congruent:
/// <https://github.com/hannobraun/Fornjot/issues/1608>
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdge {
    curve: Curve,
    boundary: [(Point<1>, Handle<SurfaceVertex>); 2],
    global_form: Handle<GlobalEdge>,
}

impl HalfEdge {
    /// Create an instance of `HalfEdge`
    pub fn new(
        curve: Curve,
        boundary: [(Point<1>, Handle<SurfaceVertex>); 2],
        global_form: Handle<GlobalEdge>,
    ) -> Self {
        Self {
            curve,
            boundary,
            global_form,
        }
    }

    /// Access the curve that defines the half-edge's geometry
    pub fn curve(&self) -> Curve {
        self.curve
    }

    /// Access the boundary points of the half-edge on the curve
    pub fn boundary(&self) -> [Point<1>; 2] {
        self.boundary.each_ref_ext().map(|&(point, _)| point)
    }

    /// Access the vertex from where this half-edge starts
    pub fn start_vertex(&self) -> &Handle<SurfaceVertex> {
        let [vertex, _] =
            self.boundary.each_ref_ext().map(|(_, vertex)| vertex);
        vertex
    }

    /// Access the surface vertices that bound the half-edge
    pub fn surface_vertices(&self) -> [&Handle<SurfaceVertex>; 2] {
        self.boundary
            .each_ref_ext()
            .map(|(_, surface_form)| surface_form)
    }

    /// Access the global form of the half-edge
    pub fn global_form(&self) -> &Handle<GlobalEdge> {
        &self.global_form
    }
}

/// An undirected edge, defined in global (3D) coordinates
///
/// In contrast to [`HalfEdge`], `GlobalEdge` is undirected, meaning it has no
/// defined direction, and its vertices have no defined order. This means it can
/// be used to determine whether two [`HalfEdge`]s map to the same `GlobalEdge`,
/// regardless of their direction.
///
/// See [`HalfEdge`]'s documentation for more information on the relationship
/// between [`HalfEdge`] and `GlobalEdge`.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalEdge {
    vertices: VerticesInNormalizedOrder,
}

impl GlobalEdge {
    /// Create a new instance
    ///
    /// The order of `vertices` is irrelevant. Two `GlobalEdge`s with the same
    /// `curve` and `vertices` will end up being equal, regardless of the order
    /// of `vertices` here.
    pub fn new(vertices: [Handle<GlobalVertex>; 2]) -> Self {
        let (vertices, _) = VerticesInNormalizedOrder::new(vertices);

        Self { vertices }
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
        builder::HalfEdgeBuilder,
        partial::{PartialHalfEdge, PartialObject},
        services::Services,
    };

    #[test]
    fn global_edge_equality() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();

        let a = [0., 0.];
        let b = [1., 0.];

        let a_to_b = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points([a, b]);
            half_edge.infer_vertex_positions_if_necessary(&surface.geometry());

            half_edge.build(&mut services.objects)
        };
        let b_to_a = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points([b, a]);
            half_edge.infer_vertex_positions_if_necessary(&surface.geometry());

            half_edge.build(&mut services.objects)
        };

        assert_eq!(a_to_b.global_form(), b_to_a.global_form());
    }
}
