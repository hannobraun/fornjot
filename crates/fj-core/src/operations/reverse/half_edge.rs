use crate::{
    operations::{derive::DeriveFrom, insert::Insert},
    storage::Handle,
    topology::{HalfEdge, Surface, Vertex},
    Core,
};

use super::ReverseCurveCoordinateSystems;

impl ReverseCurveCoordinateSystems
    for (&Handle<HalfEdge>, &Handle<Vertex>, &Handle<Surface>)
{
    type Reversed = Handle<HalfEdge>;

    fn reverse_curve_coordinate_systems(
        self,
        core: &mut Core,
    ) -> Self::Reversed {
        let (half_edge, end_vertex, surface) = self;

        let vertex_geom_start = core
            .layers
            .geometry
            .of_vertex(half_edge.start_vertex())
            .unwrap()
            .local_on(half_edge.curve())
            .unwrap()
            .clone();
        let vertex_geom_end = core
            .layers
            .geometry
            .of_vertex(end_vertex)
            .unwrap()
            .local_on(half_edge.curve())
            .unwrap()
            .clone();

        let mut half_edge_geom = *core.layers.geometry.of_half_edge(half_edge);
        half_edge_geom.boundary = half_edge_geom.boundary.reverse();

        let curve =
            (half_edge.curve(), surface).reverse_curve_coordinate_systems(core);

        let half_edge = HalfEdge::new(curve, half_edge.start_vertex().clone())
            .insert(core)
            .derive_from(half_edge, core);

        core.layers.geometry.define_vertex(
            half_edge.start_vertex().clone(),
            half_edge.curve().clone(),
            vertex_geom_end,
        );
        core.layers.geometry.define_vertex(
            end_vertex.clone(),
            half_edge.curve().clone(),
            vertex_geom_start,
        );

        core.layers
            .geometry
            .define_half_edge(half_edge.clone(), half_edge_geom);

        half_edge
    }
}
