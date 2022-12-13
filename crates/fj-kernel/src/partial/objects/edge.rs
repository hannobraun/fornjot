use std::array;

use fj_interop::ext::ArrayExt;

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects, Vertex,
    },
    partial::{FullToPartialCache, Partial, PartialObject, PartialVertex},
    services::Service,
};

/// A partial [`HalfEdge`]
#[derive(Clone, Debug)]
pub struct PartialHalfEdge {
    /// The vertices that bound the half-edge on the curve
    pub vertices: [Partial<Vertex>; 2],

    /// The global form of the half-edge
    pub global_form: Partial<GlobalEdge>,
}

impl PartialHalfEdge {
    /// Access the curve the partial edge is defined on
    pub fn curve(&self) -> Partial<Curve> {
        let [vertex, _] = &self.vertices;
        vertex.read().curve.clone()
    }
}

impl PartialObject for PartialHalfEdge {
    type Full = HalfEdge;

    fn from_full(
        half_edge: &Self::Full,
        cache: &mut FullToPartialCache,
    ) -> Self {
        Self {
            vertices: half_edge
                .vertices()
                .clone()
                .map(|vertex| Partial::from_full(vertex, cache)),
            global_form: Partial::from_full(
                half_edge.global_form().clone(),
                cache,
            ),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let vertices = self.vertices.map(|vertex| vertex.build(objects));
        let global_form = self.global_form.build(objects);

        HalfEdge::new(vertices, global_form)
    }
}

impl Default for PartialHalfEdge {
    fn default() -> Self {
        let curve = Partial::<Curve>::new();

        let vertices = array::from_fn(|_| {
            Partial::from_partial(PartialVertex {
                curve: curve.clone(),
                ..Default::default()
            })
        });

        let global_curve = curve.read().global_form.clone();
        let global_vertices =
            vertices.each_ref_ext().map(|vertex: &Partial<Vertex>| {
                let surface_vertex = vertex.read().surface_form.clone();
                let global_vertex = surface_vertex.read().global_form.clone();
                global_vertex
            });

        let global_form = Partial::from_partial(PartialGlobalEdge {
            curve: global_curve,
            vertices: global_vertices,
        });

        Self {
            vertices,
            global_form,
        }
    }
}

/// A partial [`GlobalEdge`]
#[derive(Clone, Debug, Default)]
pub struct PartialGlobalEdge {
    /// The curve that defines the edge's geometry
    pub curve: Partial<GlobalCurve>,

    /// The vertices that bound the edge on the curve
    pub vertices: [Partial<GlobalVertex>; 2],
}

impl PartialObject for PartialGlobalEdge {
    type Full = GlobalEdge;

    fn from_full(
        global_edge: &Self::Full,
        cache: &mut FullToPartialCache,
    ) -> Self {
        Self {
            curve: Partial::from_full(global_edge.curve().clone(), cache),
            vertices: global_edge
                .vertices()
                .access_in_normalized_order()
                .map(|vertex| Partial::from_full(vertex, cache)),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let curve = self.curve.build(objects);
        let vertices = self.vertices.map(|vertex| vertex.build(objects));

        GlobalEdge::new(curve, vertices)
    }
}
