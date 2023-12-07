//! Add holes to shapes

use fj_math::{Point, Vector};

use crate::{
    objects::{Cycle, Face, HalfEdge, Region, Shell},
    services::Services,
    storage::Handle,
};

use super::{
    build::{BuildCycle, BuildHalfEdge, BuildRegion},
    insert::Insert,
    join::JoinCycle,
    sweep::{SweepCache, SweepRegion},
    update::{UpdateCycle, UpdateFace, UpdateRegion, UpdateShell},
};

/// Add a hole to a [`Shell`]
pub trait AddHole {
    /// Add a blind hole to the provided face of the shell
    fn add_blind_hole(
        &self,
        face: &Handle<Face>,
        position: impl Into<Point<2>>,
        path: impl Into<Vector<3>>,
        services: &mut Services,
    ) -> Self;
}

impl AddHole for Shell {
    fn add_blind_hole(
        &self,
        face: &Handle<Face>,
        position: impl Into<Point<2>>,
        path: impl Into<Vector<3>>,
        services: &mut Services,
    ) -> Self {
        let half_edge =
            HalfEdge::circle(position, 0.25, services).insert(services);
        let hole = Region::empty(services)
            .update_exterior(|_| {
                Cycle::empty()
                    .add_half_edges([half_edge.clone()])
                    .insert(services)
            })
            .sweep_region(
                face.surface(),
                path,
                &mut SweepCache::default(),
                services,
            )
            .into_iter()
            .map(|face| face.insert(services))
            .collect::<Vec<_>>();

        self.update_face(face, |face| {
            face.update_region(|region| {
                region
                    .add_interiors([Cycle::empty()
                        .add_joined_edges(
                            [(
                                half_edge.clone(),
                                half_edge.path(),
                                half_edge.boundary(),
                            )],
                            services,
                        )
                        .insert(services)])
                    .insert(services)
            })
            .insert(services)
        })
        .add_faces(hole)
    }
}
