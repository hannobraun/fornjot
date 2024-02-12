use crate::{
    objects::{Shell, Solid},
    operations::insert::Insert,
    storage::Handle,
    Instance,
};

/// Update a [`Solid`]
pub trait UpdateSolid {
    /// Add a shell to the solid
    #[must_use]
    fn add_shells<T>(
        &self,
        shells: impl IntoIterator<Item = T>,
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>;

    /// Update a shell of the solid
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in a duplicate object.
    #[must_use]
    fn update_shell<T, const N: usize>(
        &self,
        handle: &Handle<Shell>,
        update: impl FnOnce(&Handle<Shell>, &mut Instance) -> [T; N],
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>;
}

impl UpdateSolid for Solid {
    fn add_shells<T>(
        &self,
        shells: impl IntoIterator<Item = T>,
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>,
    {
        let shells = shells.into_iter().map(|shell| shell.insert(core));
        let shells = self.shells().iter().cloned().chain(shells);
        Solid::new(shells)
    }

    fn update_shell<T, const N: usize>(
        &self,
        handle: &Handle<Shell>,
        update: impl FnOnce(&Handle<Shell>, &mut Instance) -> [T; N],
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>,
    {
        let shells = self
            .shells()
            .replace(
                handle,
                update(handle, core).map(|object| object.insert(core)),
            )
            .expect("Shell not found");
        Solid::new(shells)
    }
}
