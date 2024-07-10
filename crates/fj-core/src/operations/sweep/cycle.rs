use fj_interop::Color;
use fj_math::Vector;
use itertools::Itertools;

use crate::{
    geometry::LocalVertexGeom,
    operations::{
        build::BuildCycle, join::JoinCycle, sweep::half_edge::SweepHalfEdge,
    },
    storage::Handle,
    topology::{Cycle, Face, Surface},
    Core,
};

use super::SweepCache;

/// # Sweep a [`Cycle`]
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait SweepCycle {
    /// # Sweep the [`Cycle`]
    ///
    /// Sweep the cycle into a set of connected faces. Each half-edge in the
    /// cycle is swept into a face, meaning all resulting faces form a connected
    /// set of side walls.
    ///
    /// Requires the surface that the half-edges of the cycle are defined in,
    /// and optionally the color of the created faces.
    ///
    /// There is no face at the "top" (the end of the sweep path), as we don't
    /// have enough information here to create that. We just have a single
    /// cycle, and don't know whether that is supposed to be the only cycle
    /// within a potential top face, or whether it's an exterior or interior
    /// one.
    ///
    /// For the same reason, there also is no "bottom" face. Additionally,
    /// whether a bottom face is even desirable depends on the context this
    /// operation is called in, and therefore falls outside of its scope.
    fn sweep_cycle(
        &self,
        bottom_surface: Handle<Surface>,
        top_surface: Handle<Surface>,
        color: Option<Color>,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        core: &mut Core,
    ) -> SweptCycle;
}

impl SweepCycle for Cycle {
    fn sweep_cycle(
        &self,
        bottom_surface: Handle<Surface>,
        top_surface: Handle<Surface>,
        color: Option<Color>,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        core: &mut Core,
    ) -> SweptCycle {
        let path = path.into();

        let mut faces = Vec::new();
        let mut top_half_edges = Vec::new();

        for bottom_half_edge_pair in self.half_edges().pairs() {
            let (bottom_half_edge, bottom_half_edge_next) =
                bottom_half_edge_pair;

            let swept_half_edge = bottom_half_edge.sweep_half_edge(
                bottom_half_edge_next.start_vertex().clone(),
                bottom_surface.clone(),
                color,
                path,
                cache,
                core,
            );

            faces.push(swept_half_edge.face);

            top_half_edges.push((
                swept_half_edge.top_half_edge,
                swept_half_edge.top_boundary,
                core.layers
                    .geometry
                    .of_curve(bottom_half_edge.curve())
                    .unwrap()
                    .local_on(&bottom_surface)
                    .unwrap()
                    .clone(),
            ));
        }

        let top_half_edges = top_half_edges
            .into_iter()
            .circular_tuple_windows()
            .map(
                |(
                    (half_edge, boundary, curve_geom),
                    (next_half_edge, _, _),
                )| {
                    let [start, end] = boundary.inner;

                    for (point, vertex) in [
                        (start, half_edge.start_vertex()),
                        (end, next_half_edge.start_vertex()),
                    ] {
                        core.layers.geometry.define_vertex(
                            vertex.clone(),
                            half_edge.curve().clone(),
                            LocalVertexGeom { position: point },
                        );
                    }

                    (half_edge, curve_geom)
                },
            )
            .collect::<Vec<_>>();

        let top_cycle =
            Cycle::empty().add_joined_edges(top_half_edges, top_surface, core);

        SweptCycle { faces, top_cycle }
    }
}

/// The result of sweeping a [`Cycle`]
///
/// See [`SweepCycle`].
pub struct SweptCycle {
    /// The faces created by sweeping each half-edge of the cycle
    ///
    /// See [`SweepCycle::sweep_cycle`] for more information.
    pub faces: Vec<Face>,

    /// A cycle made up of the "top" half-edges of the resulting faces
    ///
    /// "Top" here refers to the place that the sweep path points to from the
    /// original cycle. Essentially, this is a translated (along the sweep path)
    /// and reversed version of the original cycle.
    pub top_cycle: Cycle,
}
