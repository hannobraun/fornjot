use crate::{objects::Face, partial2::Partial};

/// A partial [`Shell`]
///
/// [`Shell`]: crate::objects::Shell
pub struct PartialShell {
    /// The faces that make up the shell
    pub faces: Vec<Partial<Face>>,
}
