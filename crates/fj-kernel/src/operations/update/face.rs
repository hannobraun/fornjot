use crate::{
    objects::{Cycle, Face},
    storage::Handle,
};

/// Update a [`Face`]
pub trait UpdateFace {
    /// Update the exterior of the face
    fn update_exterior(
        &self,
        f: impl FnOnce(&Handle<Cycle>) -> Handle<Cycle>,
    ) -> Self;

    /// Add the provides interiors to the face
    fn add_interiors(
        &self,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
    ) -> Self;
}

impl UpdateFace for Face {
    fn update_exterior(
        &self,
        f: impl FnOnce(&Handle<Cycle>) -> Handle<Cycle>,
    ) -> Self {
        let exterior = f(self.exterior());

        Face::new(
            self.surface().clone(),
            exterior,
            self.interiors().cloned(),
            self.color(),
        )
    }

    fn add_interiors(
        &self,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
    ) -> Self {
        let interiors = self.interiors().cloned().chain(interiors);

        Face::new(
            self.surface().clone(),
            self.exterior().clone(),
            interiors,
            self.color(),
        )
    }
}
