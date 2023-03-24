use crate::{
    objects::{Shell, Solid},
    storage::Handle,
};

/// Update a [`Solid`]
pub trait UpdateSolid {
    /// Add a shell to the solid
    fn add_shell(&self, shell: Handle<Shell>) -> Solid;
}

impl UpdateSolid for Solid {
    fn add_shell(&self, shell: Handle<Shell>) -> Solid {
        let shells = self.shells().cloned().chain([shell]);
        Solid::new(shells)
    }
}
