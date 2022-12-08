use crate::{
    objects::{Curve, GlobalEdge, HalfEdge, Objects, Vertex},
    partial::{MaybePartial, MergeWith},
    partial2::Partial,
    services::Service,
};

/// A partial [`HalfEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialHalfEdge {
    /// The vertices that bound the [`HalfEdge`] in the curve
    pub vertices: [Partial<Vertex>; 2],

    /// The global form of the [`HalfEdge`]
    pub global_form: Partial<GlobalEdge>,
}

impl PartialHalfEdge {
    /// Access the partial half-edge's curve
    pub fn curve(&self) -> Partial<Curve> {
        let [a, _] = &self.vertices;
        a.read().curve.clone()
    }

    /// Build a full [`HalfEdge`] from the partial half-edge
    pub fn build(self, objects: &mut Service<Objects>) -> HalfEdge {
        let vertices = self.vertices.map(|vertex| vertex.build(objects));
        let global_form = self.global_form.build(objects);

        HalfEdge::new(vertices, global_form)
    }
}

impl MergeWith for PartialHalfEdge {
    fn merge_with(self, _: impl Into<Self>) -> Self {
        Self {
            vertices: self.vertices,
            global_form: self.global_form,
        }
    }
}

impl From<&HalfEdge> for PartialHalfEdge {
    fn from(half_edge: &HalfEdge) -> Self {
        let [back_vertex, front_vertex] = half_edge
            .vertices()
            .clone()
            .map(Partial::from_full_entry_point);

        Self {
            vertices: [back_vertex, front_vertex],
            global_form: Partial::from_full_entry_point(
                half_edge.global_form().clone(),
            ),
        }
    }
}

impl MaybePartial<HalfEdge> {
    /// Access the curve
    pub fn curve(&self) -> Partial<Curve> {
        match self {
            Self::Full(full) => {
                Partial::from_full_entry_point(full.curve().clone())
            }
            Self::Partial(partial) => partial.curve(),
        }
    }

    /// Access the front vertex
    pub fn front(&self) -> Partial<Vertex> {
        match self {
            Self::Full(full) => {
                Partial::from_full_entry_point(full.front().clone())
            }
            Self::Partial(partial) => {
                let [_, front] = &partial.vertices;
                front.clone()
            }
        }
    }

    /// Access the vertices
    pub fn vertices(&self) -> [Partial<Vertex>; 2] {
        match self {
            Self::Full(full) => {
                full.vertices().clone().map(Partial::from_full_entry_point)
            }
            Self::Partial(partial) => partial.vertices.clone(),
        }
    }
}
