use crate::objects::Solid;

/// Build a [`Solid`]
pub trait BuildSolid {
    /// Build an empty solid
    fn empty() -> Solid {
        Solid::new([])
    }
}

impl BuildSolid for Solid {}
