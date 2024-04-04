use crate::topology::{Sketch, Topology};

/// Build a [`Sketch`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildSketch {
    /// Create a sketch with no regions
    fn empty(_: &Topology) -> Sketch {
        Sketch::new([])
    }
}

impl BuildSketch for Sketch {}
