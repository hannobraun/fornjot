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

    /// Update a shell of the solid
    ///
    /// # Panics
    ///
    /// Uses [`Handles::update`] internally, and panics for the same reasons.
    ///
    /// [`Handles::update`]: crate::objects::Handles::update
    #[must_use]
    fn update_shell(
        &self,
        handle: &Handle<Shell>,
        update: impl FnOnce(&Handle<Shell>) -> Handle<Shell>,
    ) -> Self;
}

impl UpdateSolid for Solid {
    fn add_shells(
        &self,
        shells: impl IntoIterator<Item = Handle<Shell>>,
    ) -> Self {
        let shells = self.shells().iter().cloned().chain(shells);
        Solid::new(shells)
    }

    fn update_shell(
        &self,
        handle: &Handle<Shell>,
        update: impl FnOnce(&Handle<Shell>) -> Handle<Shell>,
    ) -> Self {
        let shells = self.shells().update(handle, update);
        Solid::new(shells)
    }
}
