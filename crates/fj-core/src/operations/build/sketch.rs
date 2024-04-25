use crate::topology::{Sketch, Topology};

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
}

impl BuildSketch for Sketch {}
