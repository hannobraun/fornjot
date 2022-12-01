use crate::{
    objects::{Face, Shell},
    partial2::{Partial, PartialObject},
};

/// A partial [`Shell`]
///
/// [`Shell`]: crate::objects::Shell
pub struct PartialShell {
    /// The faces that make up the shell
    pub faces: Vec<Partial<Face>>,
}

impl PartialObject for PartialShell {
    type Full = Shell;
}
