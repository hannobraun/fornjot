use fj_interop::Color;
use fj_math::Vector;

use crate::{
    algorithms::transform::TransformObject,
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
    /// formed by sweeping one of the region's cycles, then adding a top face.
    ///
    /// Requires the surface that the face that the region belongs to is defined
    /// in.
    ///
    /// There no "bottom" face. Whether having one is desirable depends on the
    /// context of the caller of this operation, and falls outside of this
    /// operation's scope.
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

        let top_face = {
            let top_surface =
                surface.translate(path, services).insert(services);
            let top_region =
                Region::new(top_exterior, top_interiors, self.color())
                    .insert(services);

            Face::new(top_surface, top_region)
        };

        SweptRegion {
            top_face,
            side_faces: faces,
        }
    }
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

/// The result of sweeping a [`Region`]
///
/// See [`SweepRegion`].
pub struct SweptRegion {
    /// The side faces created by the sweep
    pub side_faces: Vec<Face>,

    /// The top face created by the sweep
    pub top_face: Face,
}

impl SweptRegion {
    /// Return an iterator over all of the faces
    pub fn all_faces(self) -> impl Iterator<Item = Face> {
        self.side_faces.into_iter().chain([self.top_face])
    }
}
