use crate::objects::Cycle;

/// Build a [`Cycle`]
pub trait BuildCycle {
    /// Build an empty cycle
    fn empty() -> Cycle {
        Cycle::new([])
    }
}

impl BuildCycle for Cycle {}
