use std::array;

use fj_interop::ext::ArrayExt;

use crate::{
    objects::{GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Vertex},
    partial2::{Partial, PartialObject},
};

/// A partial [`HalfEdge`]
#[derive(Clone)]
pub struct PartialHalfEdge {
    /// The vertices that bound the half-edge on the curve
    pub vertices: [Partial<Vertex>; 2],

    /// The global form of the half-edge
    pub global_form: Partial<GlobalEdge>,
}

impl PartialObject for PartialHalfEdge {
    type Full = HalfEdge;
}

impl Default for PartialHalfEdge {
    fn default() -> Self {
        let mut vertices = array::from_fn(|_| Partial::<Vertex>::new());
        let mut global_form = Partial::<GlobalEdge>::new();

        let curve = Partial::new();
        for vertex in &mut vertices {
            vertex.write().curve = curve.clone();
        }

        let global_curve = curve.read().global_form.clone();
        let global_vertices =
            vertices.each_ref_ext().map(|vertex: &Partial<Vertex>| {
                let surface_vertex = vertex.read().surface_form.clone();
                let global_vertex = surface_vertex.read().global_form.clone();
                global_vertex
            });

        {
            let mut global_form = global_form.write();
            global_form.curve = global_curve;
            global_form.vertices = global_vertices;
        }

        Self {
            vertices,
            global_form,
        }
    }
}

/// A partial [`GlobalEdge`]
#[derive(Clone, Default)]
pub struct PartialGlobalEdge {
    /// The curve that defines the edge's geometry
    pub curve: Partial<GlobalCurve>,

    /// The vertices that bound the edge on the curve
    pub vertices: [Partial<GlobalVertex>; 2],
}

impl PartialObject for PartialGlobalEdge {
    type Full = GlobalEdge;
}
