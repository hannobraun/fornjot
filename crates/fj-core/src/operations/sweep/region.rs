use fj_interop::mesh::Color;
use fj_math::Vector;

use crate::{
    objects::{Cycle, Face, Region, Surface},
    operations::{insert::Insert, reverse::Reverse},
    services::Services,
    storage::Handle,
};

use super::{SweepCache, SweepCycle};

/// # Sweep a [`Region`]
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait SweepRegion {
    /// # Sweep the [`Region`]
    ///
    /// Sweep the region into multiple sets of faces. Each set of faces is
    /// formed by sweeping one of the region's cycles
    ///
    /// Requires the surface that the face that the region belongs to is defined
    /// in.
    ///
    /// There are no faces at the "top" (the end of the sweep path) or "bottom".
    ///
    /// There is no face at the "top" (the end of the sweep path). We *would*
    /// have enough information to create that, as we have access to the surface
    /// too and could translate that here. However, that we have access to that
    /// surface is a bit incidental, and a weird artifact of how the object
    /// graph currently works. For this reason, the creating the top face is
    /// considered out of scope for this operation, and left to the caller.
    ///
    /// There also is no "bottom" face. Whether having one is desirable, depends
    /// on the context of the caller of this operation, and there also falls
    /// outside of its scope.
    fn sweep_region(
        &self,
        surface: &Surface,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> SweptRegion;
}

impl SweepRegion for Region {
    fn sweep_region(
        &self,
        surface: &Surface,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> SweptRegion {
        let path = path.into();

        let mut faces = Vec::new();

        let top_exterior = sweep_cycle(
            self.exterior(),
            surface,
            self.color(),
            &mut faces,
            path,
            cache,
            services,
        );

        let mut top_interiors = Vec::new();

        for bottom_cycle in self.interiors() {
            let top_cycle = sweep_cycle(
                bottom_cycle,
                surface,
                self.color(),
                &mut faces,
                path,
                cache,
                services,
            );

            top_interiors.push(top_cycle);
        }

        let top_region = Region::new(top_exterior, top_interiors, self.color());

        SweptRegion { faces, top_region }
    }
}

/// The result of sweeping a [`Region`]
///
/// See [`SweepRegion`].
pub struct SweptRegion {
    /// The faces created by sweeping each cycle of the region
    pub faces: Vec<Face>,

    /// A region made up of the "top" cycles
    ///
    /// This is essentially a version of the original region, translated by the
    /// sweep path.
    pub top_region: Region,
}

fn sweep_cycle(
    bottom_cycle: &Cycle,
    bottom_surface: &Surface,
    color: Option<Color>,
    faces: &mut Vec<Face>,
    path: Vector<3>,
    cache: &mut SweepCache,
    services: &mut Services,
) -> Handle<Cycle> {
    let swept_cycle = bottom_cycle.reverse(services).sweep_cycle(
        bottom_surface,
        color,
        path,
        cache,
        services,
    );

    faces.extend(swept_cycle.faces);

    swept_cycle.top_cycle.insert(services)
}
