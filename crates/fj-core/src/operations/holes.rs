//! Add holes to shapes

use fj_math::{Point, Scalar, Vector};

use crate::{
    geometry::util::tri_mesh::convert_point_surface_to_global,
    storage::Handle,
    topology::{Cycle, Face, Region, Shell},
    Core,
};

use super::{
    build::{BuildCycle, BuildRegion},
    sweep::{SweepCache, SweepRegion},
    update::{UpdateFace, UpdateRegion, UpdateShell},
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
        let entry = Cycle::circle(
            location.position,
            radius,
            location.face.surface().clone(),
            core,
        );
        let hole = Region::empty(core)
            .update_exterior(|_, _| entry.clone(), core)
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
                    |region, core| region.add_interiors([entry], core),
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

        let entry = Cycle::circle(
            entry_location.position,
            radius,
            entry_location.face.surface().clone(),
            core,
        );

        let path = {
            let point = |location: &HoleLocation| {
                convert_point_surface_to_global(
                    &core
                        .layers
                        .geometry
                        .of_surface_2(location.face.surface())
                        .unwrap()
                        .geometry,
                    location.position,
                    core.tolerance(),
                    &core.layers.geometry,
                )
            };

            let entry_point = point(&entry_location);
            let exit_point = point(&exit_location);

            exit_point - entry_point
        };

        let swept_region = Region::empty(core)
            .update_exterior(|_, _| entry.clone(), core)
            .sweep_region(
                entry_location.face.surface().clone(),
                None,
                path,
                &mut SweepCache::default(),
                core,
            );

        let hole = swept_region.side_faces.into_iter().collect::<Vec<_>>();

        let exit = swept_region.top_face.region().exterior();

        self.update_face(
            entry_location.face,
            |face, core| {
                [face.update_region(
                    |region, core| region.add_interiors([entry], core),
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
                        for half_edge in exit.half_edges() {
                            let geometry = core
                                .layers
                                .geometry
                                .of_curve(half_edge.curve())
                                .unwrap()
                                .local_on(swept_region.top_face.surface())
                                .unwrap();
                            core.layers.geometry.define_curve(
                                half_edge.curve().clone(),
                                exit_location.face.surface().clone(),
                                geometry.clone(),
                            );
                        }
                        region.add_interiors([exit.clone()], core)
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
