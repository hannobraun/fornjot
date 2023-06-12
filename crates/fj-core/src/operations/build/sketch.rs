use fj_math::Point;

use crate::{
    geometry::region::Region,
    objects::{Cycle, Sketch},
    operations::{BuildCycle, Insert},
    services::Services,
};

/// Build a [`Sketch`]
pub trait BuildSketch {
    /// Create a sketch with no regions
    fn empty() -> Sketch {
        Sketch::new([])
    }

    /// Build a polygon
    fn polygon<P, Ps>(points: Ps, services: &mut Services) -> Sketch
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let exterior = Cycle::polygon(points, services).insert(services);
        let region = Region::new(exterior, Vec::new(), None);
        Sketch::new([region])
    }
}

impl BuildSketch for Sketch {}
