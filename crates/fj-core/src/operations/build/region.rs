use fj_math::{Point, Scalar};

use crate::{
    objects::{Cycle, Region},
    operations::{build::BuildCycle, insert::Insert},
    Instance,
};

/// Build a [`Region`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildRegion {
    /// Build an empty region
    fn empty(core: &mut Instance) -> Region {
        let exterior = Cycle::empty().insert(core);
        let interiors = [];
        let color = None;

        Region::new(exterior, interiors, color)
    }

    /// Build a circle
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        core: &mut Instance,
    ) -> Region {
        let exterior = Cycle::circle(center, radius, core).insert(core);
        Region::new(exterior, [], None)
    }

    /// Build a polygon
    fn polygon<P, Ps>(points: Ps, core: &mut Instance) -> Region
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let exterior = Cycle::polygon(points, core).insert(core);
        Region::new(exterior, [], None)
    }
}

impl BuildRegion for Region {}
