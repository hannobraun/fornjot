use fj_math::{Point, Scalar};

use crate::{
    Core,
    operations::{build::BuildCycle, insert::Insert},
    storage::Handle,
    topology::{Cycle, Region, Surface},
};

/// Build a [`Region`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildRegion {
    /// Build an empty region
    fn empty(core: &mut Core) -> Region {
        let exterior = Cycle::empty().insert(core);
        let interiors = [];

        Region::new(exterior, interiors)
    }

    /// Build a circle
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        surface: Handle<Surface>,
        core: &mut Core,
    ) -> Region {
        let exterior =
            Cycle::circle(center, radius, surface, core).insert(core);
        Region::new(exterior, [])
    }

    /// Build a polygon
    fn polygon<P, Ps>(
        points: Ps,
        surface: Handle<Surface>,
        core: &mut Core,
    ) -> Region
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let exterior = Cycle::polygon(points, surface, core).insert(core);
        Region::new(exterior, [])
    }
}

impl BuildRegion for Region {}
