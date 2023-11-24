use fj_interop::mesh::Color;
use fj_math::Vector;

use crate::{
    objects::{Cycle, Face, Surface},
    operations::{
        build::BuildCycle, join::JoinCycle, sweep::half_edge::SweepHalfEdge,
    },
    services::Services,
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
        surface: &Surface,
        color: Option<Color>,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> SweptCycle;
}

impl SweepCycle for Cycle {
    fn sweep_cycle(
        &self,
        surface: &Surface,
        color: Option<Color>,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> SweptCycle {
        let path = path.into();

        let mut faces = Vec::new();
        let mut top_edges = Vec::new();

        for bottom_half_edge_pair in self.half_edges().pairs() {
            let (bottom_half_edge, bottom_half_edge_next) =
                bottom_half_edge_pair;

            let (side_face, top_edge) = bottom_half_edge.sweep_half_edge(
                bottom_half_edge_next.start_vertex().clone(),
                surface,
                color,
                path,
                cache,
                services,
            );

            faces.push(side_face);

            top_edges.push((
                top_edge,
                bottom_half_edge.path(),
                bottom_half_edge.boundary(),
            ));
        }

        let top_cycle = Cycle::empty().add_joined_edges(top_edges, services);

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
