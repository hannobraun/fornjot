use fj_math::Point;

use crate::{
    geometry::region::Region,
    objects::{Cycle, Sketch},
    operations::{BuildCycle, Insert},
    services::Services,
};

/// Update a [`Sketch`]
pub trait UpdateSketch {
    /// Add a polygon to the sketch
    fn add_polygon<P, Ps>(&self, points: Ps, services: &mut Services) -> Self
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator;
}

impl UpdateSketch for Sketch {
    fn add_polygon<P, Ps>(&self, points: Ps, services: &mut Services) -> Self
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let exterior = Cycle::polygon(points, services).insert(services);
        let region = Region::new(exterior, Vec::new(), None);

        Sketch::new(self.regions().cloned().chain([region]))
    }
}
