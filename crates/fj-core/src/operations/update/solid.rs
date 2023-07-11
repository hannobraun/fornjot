use crate::{
    objects::{Shell, Solid},
    storage::Handle,
};

/// Update a [`Solid`]
pub trait UpdateSolid {
    /// Add a shell to the solid
    #[must_use]
    fn add_shells(
        &self,
        shells: impl IntoIterator<Item = Handle<Shell>>,
    ) -> Self;
}

impl UpdateSolid for Solid {
    fn add_shells(
        &self,
        shells: impl IntoIterator<Item = Handle<Shell>>,
    ) -> Self {
        let shells = self.shells().cloned().chain(shells);
        Solid::new(shells)
    }
}
