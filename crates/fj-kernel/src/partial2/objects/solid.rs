use crate::{objects::Shell, partial2::Partial};

/// A partial [`Solid`]
///
/// [`Solid`]: crate::objects::Solid
pub struct PartialSolid {
    /// The shells that make up the solid
    pub shells: Vec<Partial<Shell>>,
}
