use fj_math::{Point, Scalar};

use crate::{
    objects::{Cycle, Region, Sketch},
    operations::{BuildCycle, Insert},
    services::Services,
    storage::Handle,
};

/// Update a [`Sketch`]
pub trait UpdateSketch {
    /// Add a region to the sketch
    fn add_region(&self, region: Handle<Region>) -> Self;

    /// Add a circle to the sketch
    fn add_circle(
        &self,
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        services: &mut Services,
    ) -> Self;

    /// Add a polygon to the sketch
    fn add_polygon<P, Ps>(&self, points: Ps, services: &mut Services) -> Self
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator;
}

impl UpdateSketch for Sketch {
    fn add_region(&self, region: Handle<Region>) -> Self {
        Sketch::new(self.regions().cloned().chain([region.clone_object()]))
    }

    fn add_circle(
        &self,
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        services: &mut Services,
    ) -> Self {
        let exterior = Cycle::circle(center, radius, services).insert(services);
        let region = Region::new(exterior, [], None).insert(services);
        self.add_region(region)
    }

    fn add_polygon<P, Ps>(&self, points: Ps, services: &mut Services) -> Self
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let exterior = Cycle::polygon(points, services).insert(services);
        let region = Region::new(exterior, [], None).insert(services);
        self.add_region(region)
    }
}
