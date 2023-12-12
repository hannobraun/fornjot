//! Add holes to shapes

use fj_math::{Point, Scalar, Vector};

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
    /// Add a blind hole at the provided location
    fn add_blind_hole(
        &self,
        location: HoleLocation,
        radius: impl Into<Scalar>,
        path: impl Into<Vector<3>>,
        services: &mut Services,
    ) -> Self;
}

impl AddHole for Shell {
    fn add_blind_hole(
        &self,
        location: HoleLocation,
        radius: impl Into<Scalar>,
        path: impl Into<Vector<3>>,
        services: &mut Services,
    ) -> Self {
        let half_edge = HalfEdge::circle(location.position, radius, services)
            .insert(services);
        let hole = Region::empty(services)
            .update_exterior(|_| {
                Cycle::empty()
                    .add_half_edges([half_edge.clone()])
                    .insert(services)
            })
            .sweep_region(
                location.face.surface(),
                path,
                &mut SweepCache::default(),
                services,
            )
            .all_faces()
            .map(|face| face.insert(services))
            .collect::<Vec<_>>();

        self.update_face(location.face, |face| {
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

/// Defines the location of a hole
pub struct HoleLocation<'r> {
    /// The face that the hole is in
    pub face: &'r Handle<Face>,

    /// The position of the hole within the face, in surface coordinates
    pub position: Point<2>,
}
