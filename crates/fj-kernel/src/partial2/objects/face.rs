use fj_interop::mesh::Color;

use crate::{
    objects::{Cycle, Face},
    partial2::{Partial, PartialObject},
};

/// A partial [`Face`]
#[derive(Clone)]
pub struct PartialFace {
    /// The cycle that bounds the face on the outside
    pub exterior: Partial<Cycle>,

    /// The cycles that bound the face on the inside
    ///
    /// Each of these cycles defines a hole in the face.
    pub interiors: Vec<Partial<Cycle>>,

    /// The color of the face
    pub color: Option<Color>,
}

impl PartialObject for PartialFace {
    type Full = Face;
}
