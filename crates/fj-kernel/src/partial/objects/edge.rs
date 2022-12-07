use crate::{
    builder::GlobalEdgeBuilder,
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects, Vertex,
    },
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
    pub global_form: MaybePartial<GlobalEdge>,
}

impl PartialHalfEdge {
    /// Access the partial half-edge's curve
    pub fn curve(&self) -> Partial<Curve> {
        let [a, _] = &self.vertices;
        a.read().curve.clone()
    }

    /// Build a full [`HalfEdge`] from the partial half-edge
    pub fn build(self, objects: &mut Service<Objects>) -> HalfEdge {
        let curve = self.curve().build(objects);
        let vertices = self.vertices.map(|vertex| vertex.build(objects));

        let global_form = self
            .global_form
            .update_partial(|partial| {
                partial.update_from_curve_and_vertices(&curve, &vertices)
            })
            .into_full(objects);

        HalfEdge::new(vertices, global_form)
    }
}

impl MergeWith for PartialHalfEdge {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            vertices: self.vertices,
            global_form: self.global_form.merge_with(other.global_form),
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
            global_form: half_edge.global_form().clone().into(),
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

/// A partial [`GlobalEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialGlobalEdge {
    /// The curve that the [`GlobalEdge`] is defined in
    pub curve: Partial<GlobalCurve>,

    /// The vertices that bound the [`GlobalEdge`] in the curve
    pub vertices: [Partial<GlobalVertex>; 2],
}

impl PartialGlobalEdge {
    /// Build a full [`GlobalEdge`] from the partial global edge
    pub fn build(self, objects: &mut Service<Objects>) -> GlobalEdge {
        let curve = self.curve.build(objects);
        let vertices = self
            .vertices
            .map(|global_vertex| global_vertex.build(objects));

        GlobalEdge::new(curve, vertices)
    }
}

impl MergeWith for PartialGlobalEdge {
    fn merge_with(self, _: impl Into<Self>) -> Self {
        Self {
            curve: self.curve,
            vertices: self.vertices,
        }
    }
}

impl From<&GlobalEdge> for PartialGlobalEdge {
    fn from(global_edge: &GlobalEdge) -> Self {
        Self {
            curve: Partial::from_full_entry_point(global_edge.curve().clone()),
            vertices: global_edge
                .vertices()
                .access_in_normalized_order()
                .map(Partial::from_full_entry_point),
        }
    }
}

impl MaybePartial<GlobalEdge> {
    /// Access the curve
    pub fn curve(&self) -> Partial<GlobalCurve> {
        match self {
            Self::Full(full) => {
                Partial::from_full_entry_point(full.curve().clone())
            }
            Self::Partial(partial) => partial.curve.clone(),
        }
    }

    /// Access the vertices
    pub fn vertices(&self) -> [Partial<GlobalVertex>; 2] {
        match self {
            Self::Full(full) => full
                .vertices()
                .access_in_normalized_order()
                .map(Partial::from_full_entry_point),
            Self::Partial(partial) => partial.vertices.clone(),
        }
    }
}
