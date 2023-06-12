use crate::objects::Sketch;

/// Build a [`Sketch`]
pub trait BuildSketch {
    /// Create a sketch with no regions
    fn empty() -> Sketch {
        Sketch::new([])
    }
}

impl BuildSketch for Sketch {}
