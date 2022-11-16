use fj_interop::ext::ArrayExt;

use crate::{
    builder::GlobalEdgeBuilder,
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects,
        Surface, Vertex,
    },
    partial::{MaybePartial, MergeWith, PartialCurve, PartialVertex, Replace},
    storage::Handle,
    validate::ValidationError,
};

/// A partial [`HalfEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialHalfEdge {
    /// The curve that the [`HalfEdge`] is defined in
    pub curve: MaybePartial<Curve>,

    /// The vertices that bound the [`HalfEdge`] in the curve
    pub vertices: [MaybePartial<Vertex>; 2],

    /// The global form of the [`HalfEdge`]
    pub global_form: MaybePartial<GlobalEdge>,
}

impl PartialHalfEdge {
    /// Build a full [`HalfEdge`] from the partial half-edge
    pub fn build(
        mut self,
        objects: &Objects,
    ) -> Result<HalfEdge, ValidationError> {
        let global_curve = self
            .curve
            .global_form()
            .merge_with(self.global_form.curve());

        let curve = {
            self.curve = self.curve.merge_with(PartialCurve {
                global_form: global_curve,
                ..Default::default()
            });

            self.curve.into_full(objects)?
        };
        let vertices = self.vertices.try_map_ext(|vertex| {
            vertex
                .merge_with(PartialVertex {
                    curve: curve.clone().into(),
                    ..Default::default()
                })
                .into_full(objects)
        })?;

        let global_form = self
            .global_form
            .update_partial(|partial| {
                partial.update_from_curve_and_vertices(&curve, &vertices)
            })
            .into_full(objects)?;

        Ok(HalfEdge::new(vertices, global_form))
    }
}

impl MergeWith for PartialHalfEdge {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            curve: self.curve.merge_with(other.curve),
            vertices: self.vertices.merge_with(other.vertices),
            global_form: self.global_form.merge_with(other.global_form),
        }
    }
}

impl Replace<Surface> for PartialHalfEdge {
    fn replace(&mut self, surface: Handle<Surface>) -> &mut Self {
        self.curve.replace(surface.clone());

        for vertex in &mut self.vertices {
            vertex.replace(surface.clone());
        }

        self
    }
}

impl From<&HalfEdge> for PartialHalfEdge {
    fn from(half_edge: &HalfEdge) -> Self {
        let [back_vertex, front_vertex] =
            half_edge.vertices().clone().map(Into::into);

        Self {
            curve: half_edge.curve().clone().into(),
            vertices: [back_vertex, front_vertex],
            global_form: half_edge.global_form().clone().into(),
        }
    }
}

/// A partial [`GlobalEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialGlobalEdge {
    /// The curve that the [`GlobalEdge`] is defined in
    pub curve: MaybePartial<GlobalCurve>,

    /// The vertices that bound the [`GlobalEdge`] in the curve
    pub vertices: [MaybePartial<GlobalVertex>; 2],
}

impl PartialGlobalEdge {
    /// Build a full [`GlobalEdge`] from the partial global edge
    pub fn build(
        self,
        objects: &Objects,
    ) -> Result<GlobalEdge, ValidationError> {
        let curve = self.curve.into_full(objects)?;
        let vertices = self
            .vertices
            .try_map_ext(|global_vertex| global_vertex.into_full(objects))?;

        Ok(GlobalEdge::new(curve, vertices))
    }
}

impl MergeWith for PartialGlobalEdge {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            curve: self.curve.merge_with(other.curve),
            vertices: self.vertices.merge_with(other.vertices),
        }
    }
}

impl From<&GlobalEdge> for PartialGlobalEdge {
    fn from(global_edge: &GlobalEdge) -> Self {
        Self {
            curve: global_edge.curve().clone().into(),
            vertices: global_edge
                .vertices()
                .access_in_normalized_order()
                .map(Into::into),
        }
    }
}
