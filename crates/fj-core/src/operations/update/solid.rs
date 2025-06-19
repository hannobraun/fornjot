use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert, selector::Selector},
    storage::Handle,
    topology::{Shell, Solid},
};

/// Update a [`Solid`]
pub trait UpdateSolid {
    /// Add a shell to the solid
    #[must_use]
    fn add_shells<T>(
        &self,
        shells: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>;

    /// Update a shell of the solid
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in multiple handles referencing the same object.
    #[must_use]
    fn update_shell<T, R>(
        &self,
        handle: &Handle<Shell>,
        update: impl FnOnce(&Handle<Shell>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>,
        R: IntoIterator<Item = T>;
}

/// Update a [`Solid`] with flexible selectors
///
/// This trait provides a more flexible interface for updating solids, allowing
/// objects to be selected using the `Selector` trait.
pub trait UpdateSolidWithSelector {
    /// Add a shell to the solid
    #[must_use]
    fn add_shells<T>(
        &self,
        shells: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>;

    /// Update shells selected by the given selector
    ///
    /// # Panics
    ///
    /// Panics, if any selected object can't be found.
    ///
    /// Panics, if the update results in multiple handles referencing the same object.
    #[must_use]
    fn update_shells<T, R>(
        &self,
        selector: impl Selector<Shell>,
        update: impl Fn(&Handle<Shell>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>,
        R: IntoIterator<Item = T>;
}

impl UpdateSolid for Solid {
    fn add_shells<T>(
        &self,
        shells: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>,
    {
        let shells = shells.into_iter().map(|shell| shell.insert(core));
        let shells = self.shells().iter().cloned().chain(shells);
        Solid::new(shells)
    }

    fn update_shell<T, R>(
        &self,
        handle: &Handle<Shell>,
        update: impl FnOnce(&Handle<Shell>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>,
        R: IntoIterator<Item = T>,
    {
        let shells = self
            .shells()
            .replace(
                handle,
                update(handle, core).into_iter().map(|object| {
                    object.insert(core).derive_from(handle, core)
                }),
            )
            .expect("Shell not found");
        Solid::new(shells)
    }
}

impl UpdateSolidWithSelector for Solid {
    fn add_shells<T>(
        &self,
        shells: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>,
    {
        let shells = shells.into_iter().map(|shell| shell.insert(core));
        let shells = self.shells().iter().cloned().chain(shells);
        Solid::new(shells)
    }

    fn update_shells<T, R>(
        &self,
        selector: impl Selector<Shell>,
        update: impl Fn(&Handle<Shell>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Shell>>,
        R: IntoIterator<Item = T>,
    {
        let selected_handles: Vec<_> = selector.select(self.shells()).collect();

        let mut result = self.clone();
        for handle in selected_handles {
            result = result.update_shell(handle, &update, core);
        }
        result
    }
}
