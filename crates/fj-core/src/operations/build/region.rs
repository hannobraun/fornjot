use fj_math::{Point, Scalar};

use crate::{
    objects::{Cycle, Region},
    operations::{BuildCycle, Insert},
    services::Services,
};

/// Build a [`Region`]
pub trait BuildRegion {
    /// Build an empty region
    fn empty(services: &mut Services) -> Region {
        let exterior = Cycle::empty().insert(services);
        let interiors = [];
        let color = None;

        Region::new(exterior, interiors, color)
    }

    /// Build a circle
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        services: &mut Services,
    ) -> Region {
        let exterior = Cycle::circle(center, radius, services).insert(services);
        Region::new(exterior, [], None)
    }

    /// Build a polygon
    fn polygon<P, Ps>(points: Ps, services: &mut Services) -> Region
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let exterior = Cycle::polygon(points, services).insert(services);
        Region::new(exterior, [], None)
    }
}

impl BuildRegion for Region {}
