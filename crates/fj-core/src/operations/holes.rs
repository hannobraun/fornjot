//! Add holes to shapes

use fj_math::{Point, Scalar, Vector};

use crate::{
    objects::{Cycle, Face, HalfEdge, Region, Shell},
    storage::Handle,
    Instance,
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
        core: &mut Instance,
    ) -> Self;

    /// Add a through hole between the provided locations
    fn add_through_hole(
        &self,
        locations: [HoleLocation; 2],
        radius: impl Into<Scalar>,
        core: &mut Instance,
    ) -> Self;
}

impl AddHole for Shell {
    fn add_blind_hole(
        &self,
        location: HoleLocation,
        radius: impl Into<Scalar>,
        path: impl Into<Vector<3>>,
        core: &mut Instance,
    ) -> Self {
        let entry =
            HalfEdge::circle(location.position, radius, &mut core.services)
                .insert(&mut core.services);
        let hole = Region::empty(&mut core.services)
            .update_exterior(|_| {
                Cycle::empty()
                    .add_half_edges([entry.clone()])
                    .insert(&mut core.services)
            })
            .sweep_region(
                location.face.surface(),
                path,
                &mut SweepCache::default(),
                &mut core.services,
            )
            .all_faces()
            .map(|face| face.insert(&mut core.services))
            .collect::<Vec<_>>();

        self.update_face(location.face, |face| {
            face.update_region(|region| {
                region
                    .add_interiors([Cycle::empty()
                        .add_joined_edges(
                            [(entry.clone(), entry.path(), entry.boundary())],
                            &mut core.services,
                        )
                        .insert(&mut core.services)])
                    .insert(&mut core.services)
            })
            .insert(&mut core.services)
        })
        .add_faces(hole)
    }

    fn add_through_hole(
        &self,
        [entry_location, exit_location]: [HoleLocation; 2],
        radius: impl Into<Scalar>,
        core: &mut Instance,
    ) -> Self {
        let radius = radius.into();

        let entry = HalfEdge::circle(
            entry_location.position,
            radius,
            &mut core.services,
        )
        .insert(&mut core.services);

        let path = {
            let point = |location: &HoleLocation| {
                location
                    .face
                    .surface()
                    .geometry()
                    .point_from_surface_coords(location.position)
            };

            let entry_point = point(&entry_location);
            let exit_point = point(&exit_location);

            exit_point - entry_point
        };

        let swept_region = Region::empty(&mut core.services)
            .update_exterior(|_| {
                Cycle::empty()
                    .add_half_edges([entry.clone()])
                    .insert(&mut core.services)
            })
            .sweep_region(
                entry_location.face.surface(),
                path,
                &mut SweepCache::default(),
                &mut core.services,
            );

        let hole = swept_region
            .side_faces
            .into_iter()
            .map(|face| face.insert(&mut core.services))
            .collect::<Vec<_>>();

        let exit = swept_region
            .top_face
            .region()
            .exterior()
            .half_edges()
            .only();

        self.update_face(entry_location.face, |face| {
            face.update_region(|region| {
                region
                    .add_interiors([Cycle::empty()
                        .add_joined_edges(
                            [(entry.clone(), entry.path(), entry.boundary())],
                            &mut core.services,
                        )
                        .insert(&mut core.services)])
                    .insert(&mut core.services)
            })
            .insert(&mut core.services)
        })
        .update_face(exit_location.face, |face| {
            face.update_region(|region| {
                region
                    .add_interiors([Cycle::empty()
                        .add_joined_edges(
                            [(exit.clone(), exit.path(), exit.boundary())],
                            &mut core.services,
                        )
                        .insert(&mut core.services)])
                    .insert(&mut core.services)
            })
            .insert(&mut core.services)
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
