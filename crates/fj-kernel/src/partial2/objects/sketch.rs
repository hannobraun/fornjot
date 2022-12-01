use crate::{objects::Face, partial2::Partial};

/// A partial [`Sketch`]
///
/// [`Sketch`]: crate::objects::Sketch
pub struct PartialSketch {
    /// The faces that make up the sketch
    pub faces: Vec<Partial<Face>>,
}
