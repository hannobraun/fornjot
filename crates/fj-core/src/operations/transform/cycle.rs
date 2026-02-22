use itertools::Itertools;

use crate::{
    Core,
    math::Transform,
    operations::insert::Insert,
    storage::Handle,
    topology::{Cycle, Surface},
};

use super::{TransformCache, TransformObject};

impl TransformObject for (&Handle<Cycle>, &Handle<Surface>) {
    type Transformed = Handle<Cycle>;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        let (cycle, surface) = self;

        let half_edges_and_old_vertex_geometries = cycle
            .half_edges()
            .pairs()
            .map(|(half_edge, next_half_edge)| {
                let vertex_a_geom = core
                    .layers
                    .geometry
                    .of_vertex(half_edge.start_vertex())
                    .unwrap()
                    .local_on(half_edge.curve())
                    .unwrap()
                    .clone();
                let vertex_b_geom = core
                    .layers
                    .geometry
                    .of_vertex(next_half_edge.start_vertex())
                    .unwrap()
                    .local_on(half_edge.curve())
                    .unwrap()
                    .clone();

                let half_edge = (half_edge, surface)
                    .transform_with_cache(transform, core, cache);

                (half_edge, vertex_a_geom, vertex_b_geom)
            })
            .collect::<Vec<_>>();

        // That we're transforming the vertex geometry here, instead of down in
        // the vertex transform implementation, presents an inconsistency in the
        // transform architecture. But if we were to transform it down there,
        // we'd have to pass all kids of information down there (the half-edge
        // transform code would need the half-edge's end vertex, and we would
        // need to pass the curve into the vertex transform code).
        //
        // Doing it here is easier. It might be a hack, but so be it. I have two
        // reasons for accepting this, instead of doing it "the right way" (if
        // what I've described in the previous paragraph actually is the right
        // thing to do):
        //
        // - At some point, it will no longer be necessary to define vertex
        //   geometry redundantly in every local curve space. Then this whole
        //   situation will be easier to handle.
        // - With geometry living in a separate layer now, it's unclear if the
        //   current approach to transforms is the right one anyway. So far,
        //   we're still creating new topological objects with each
        //   transformation, but whether this is desirable is not obvious.
        //
        // When we have progress on those two points, whatever the "right way"
        // is will have changed anyway, so I'm not too worried about a hack here
        // or there.
        let half_edges = half_edges_and_old_vertex_geometries
            .into_iter()
            .circular_tuple_windows()
            .map(
                |(
                    (half_edge, vertex_a_geom, vertex_b_geom),
                    (next_half_edge, _, _),
                )| {
                    // We have only transformed the vertices in 3D space. They
                    // still have the same positions in local curve space. We
                    // just have to copy those over to the new vertices.
                    core.layers.geometry.define_vertex(
                        half_edge.start_vertex().clone(),
                        half_edge.curve().clone(),
                        vertex_a_geom,
                    );
                    core.layers.geometry.define_vertex(
                        next_half_edge.start_vertex().clone(),
                        half_edge.curve().clone(),
                        vertex_b_geom,
                    );

                    half_edge
                },
            );

        Cycle::new(half_edges).insert(core)
    }
}
