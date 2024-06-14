use fj_interop::Color;
use fj_math::Vector;

use crate::{
    operations::{
        insert::Insert, reverse::Reverse, transform::TransformObject,
    },
    storage::Handle,
    topology::{Cycle, Face, Region, Surface},
    Core,
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
        bottom_surface: Handle<Surface>,
        color: Option<Color>,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        core: &mut Core,
    ) -> SweptRegion;
}

impl SweepRegion for Region {
    fn sweep_region(
        &self,
        bottom_surface: Handle<Surface>,
        color: Option<Color>,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        core: &mut Core,
    ) -> SweptRegion {
        let path = path.into();

        let mut faces = Vec::new();

        let top_surface =
            bottom_surface.clone().translate(path, core).insert(core);

        let top_exterior = sweep_cycle(
            self.exterior(),
            bottom_surface.clone(),
            top_surface.clone(),
            color,
            &mut faces,
            path,
            cache,
            core,
        );

        let top_interiors = self
            .interiors()
            .iter()
            .map(|bottom_cycle| {
                sweep_cycle(
                    bottom_cycle,
                    bottom_surface.clone(),
                    top_surface.clone(),
                    color,
                    &mut faces,
                    path,
                    cache,
                    core,
                )
            })
            .collect::<Vec<_>>();

        let top_face = {
            let top_region =
                Region::new(top_exterior, top_interiors).insert(core);

            Face::new(top_surface, top_region)
        };

        SweptRegion {
            top_face,
            side_faces: faces,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn sweep_cycle(
    bottom_cycle: &Cycle,
    bottom_surface: Handle<Surface>,
    top_surface: Handle<Surface>,
    color: Option<Color>,
    faces: &mut Vec<Face>,
    path: Vector<3>,
    cache: &mut SweepCache,
    core: &mut Core,
) -> Handle<Cycle> {
    let swept_cycle = bottom_cycle.reverse(core).sweep_cycle(
        bottom_surface,
        top_surface,
        color,
        path,
        cache,
        core,
    );

    faces.extend(swept_cycle.faces);

    swept_cycle.top_cycle.insert(core)
}

/// The result of sweeping a [`Region`]
///
/// See [`SweepRegion`].
#[derive(Clone)]
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
