use fj_math::{Point, Scalar};

use crate::{
    operations::update::UpdateSketch,
    topology::{Region, Sketch, Topology},
    Core,
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
}

impl BuildSketch for Sketch {}
