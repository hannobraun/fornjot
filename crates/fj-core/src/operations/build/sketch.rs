use crate::{
    Core,
    math::{Point, Scalar},
    operations::update::UpdateSketch,
    topology::{Region, Sketch, Topology},
};

use super::BuildRegion;

/// Build a [`Sketch`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildSketch {
    /// Create a sketch with no regions
    fn empty(topology: &Topology) -> Sketch {
        Sketch::new(topology.surfaces.space_2d(), [])
    }

    /// Build a circle
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        core: &mut Core,
    ) -> Sketch {
        let sketch = Sketch::empty(&core.layers.topology);
        sketch.add_regions(
            [Region::circle(
                center,
                radius,
                sketch.surface().clone(),
                core,
            )],
            core,
        )
    }

    /// Build a polygon
    fn polygon<P, Ps>(points: Ps, core: &mut Core) -> Sketch
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let sketch = Sketch::empty(&core.layers.topology);
        sketch.add_regions(
            [Region::polygon(points, sketch.surface().clone(), core)],
            core,
        )
    }
}

impl BuildSketch for Sketch {}
