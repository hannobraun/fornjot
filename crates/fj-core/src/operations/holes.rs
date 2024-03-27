//! Add holes to shapes

use fj_math::{Point, Scalar, Vector};

use crate::{
    storage::Handle,
    topology::{Cycle, Face, HalfEdge, Region, Shell},
    Core,
};

use super::{
    build::{BuildCycle, BuildHalfEdge, BuildRegion},
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
        core: &mut Core,
    ) -> Self;

    /// Add a through hole between the provided locations
    fn add_through_hole(
        &self,
        locations: [HoleLocation; 2],
        radius: impl Into<Scalar>,
        core: &mut Core,
    ) -> Self;
}

impl AddHole for Shell {
    fn add_blind_hole(
        &self,
        location: HoleLocation,
        radius: impl Into<Scalar>,
        path: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> Self {
        let entry = HalfEdge::circle(location.position, radius, core);
        let hole = Region::empty(core)
            .update_exterior(
                |_, core| Cycle::empty().add_half_edges([entry.clone()], core),
                core,
            )
            .sweep_region(
                location.face.surface().clone(),
                None,
                path,
                &mut SweepCache::default(),
                core,
            )
            .all_faces()
            .collect::<Vec<_>>();

        self.update_face(
            location.face,
            |face, core| {
                [face.update_region(
                    |region, core| {
                        region.add_interiors(
                            [Cycle::empty().add_joined_edges(
                                [(
                                    entry.clone(),
                                    *core.layers.geometry.of_half_edge(&entry),
                                )],
                                core,
                            )],
                            core,
                        )
                    },
                    core,
                )]
            },
            core,
        )
        .add_faces(hole, core)
    }

    fn add_through_hole(
        &self,
        [entry_location, exit_location]: [HoleLocation; 2],
        radius: impl Into<Scalar>,
        core: &mut Core,
    ) -> Self {
        let radius = radius.into();

        let entry = HalfEdge::circle(entry_location.position, radius, core);

        let path = {
            let point = |location: &HoleLocation| {
                core.layers
                    .geometry
                    .of_surface(location.face.surface())
                    .point_from_surface_coords(location.position)
            };

            let entry_point = point(&entry_location);
            let exit_point = point(&exit_location);

            exit_point - entry_point
        };

        let swept_region = Region::empty(core)
            .update_exterior(
                |_, core| Cycle::empty().add_half_edges([entry.clone()], core),
                core,
            )
            .sweep_region(
                entry_location.face.surface().clone(),
                None,
                path,
                &mut SweepCache::default(),
                core,
            );

        let hole = swept_region.side_faces.into_iter().collect::<Vec<_>>();

        let exit = swept_region
            .top_face
            .region()
            .exterior()
            .half_edges()
            .only();

        self.update_face(
            entry_location.face,
            |face, core| {
                [face.update_region(
                    |region, core| {
                        region.add_interiors(
                            [Cycle::empty().add_joined_edges(
                                [(
                                    entry.clone(),
                                    *core.layers.geometry.of_half_edge(&entry),
                                )],
                                core,
                            )],
                            core,
                        )
                    },
                    core,
                )]
            },
            core,
        )
        .update_face(
            exit_location.face,
            |face, core| {
                [face.update_region(
                    |region, core| {
                        region.add_interiors(
                            [Cycle::empty().add_joined_edges(
                                [(
                                    exit.clone(),
                                    *core.layers.geometry.of_half_edge(exit),
                                )],
                                core,
                            )],
                            core,
                        )
                    },
                    core,
                )]
            },
            core,
        )
        .add_faces(hole, core)
    }
}

/// Defines the location of a hole
pub struct HoleLocation<'r> {
    /// The face that the hole is in
    pub face: &'r Handle<Face>,

    /// The position of the hole within the face, in surface coordinates
    pub position: Point<2>,
}
