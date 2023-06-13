use fj_math::{Point, Scalar};

use crate::{
    objects::{Cycle, Region},
    operations::{BuildCycle, Insert},
    services::Services,
};

/// Build a [`Region`]
pub trait BuildRegion {
    /// Build a circle
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        services: &mut Services,
    ) -> Region {
        let exterior = Cycle::circle(center, radius, services).insert(services);
        Region::new(exterior, [], None)
    }
}

impl BuildRegion for Region {}
