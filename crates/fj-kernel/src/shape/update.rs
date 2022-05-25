use fj_math::Scalar;

use super::{stores::Stores, validate::Validate as _, Object, ValidationError};

/// API to update a `Shape`
///
/// See [`Shape::update`].
#[must_use]
pub struct Update<'r> {
    min_distance: Scalar,
    max_distance: Scalar,
    stores: &'r mut Stores,
    executed: bool,
}

impl<'r> Update<'r> {
    pub(super) fn new(
        min_distance: Scalar,
        max_distance: Scalar,
        stores: &'r mut Stores,
    ) -> Self {
        Self {
            min_distance,
            max_distance,
            stores,
            executed: false,
        }
    }

    /// Update all objects of a specific type
    pub fn update_all<T: Object>(self, f: impl FnMut(&mut T)) -> Self {
        self.stores.get::<T>().update(f);
        self
    }

    /// Validate the update
    ///
    /// The update is validated automatically, when this `Update` instance is
    /// dropped. It is recommended to validate by calling this method though, as
    /// no [`ValidationError`] will be available otherwise.
    pub fn validate(mut self) -> Result<(), ValidationError> {
        self.validate_inner()
    }

    fn validate_inner(&mut self) -> Result<(), ValidationError> {
        if !self.executed {
            self.executed = true;

            // Validating every single object is certainly not ideal from a
            // performance perspective, but it will do for now.
            for object in self.stores.points.iter() {
                object.get().validate(
                    Some(&object),
                    self.min_distance,
                    self.max_distance,
                    self.stores,
                )?;
            }
            for object in self.stores.curves.iter() {
                object.get().validate(
                    Some(&object),
                    self.min_distance,
                    self.max_distance,
                    self.stores,
                )?;
            }
            for object in self.stores.surfaces.iter() {
                object.get().validate(
                    Some(&object),
                    self.min_distance,
                    self.max_distance,
                    self.stores,
                )?;
            }
            for object in self.stores.vertices.iter() {
                object.get().validate(
                    Some(&object),
                    self.min_distance,
                    self.max_distance,
                    self.stores,
                )?;
            }
            for object in self.stores.edges.iter() {
                object.get().validate(
                    Some(&object),
                    self.min_distance,
                    self.max_distance,
                    self.stores,
                )?;
            }
            for object in self.stores.cycles.iter() {
                object.get().validate(
                    Some(&object),
                    self.min_distance,
                    self.max_distance,
                    self.stores,
                )?;
            }
            for object in self.stores.faces.iter() {
                object.get().validate(
                    Some(&object),
                    self.min_distance,
                    self.max_distance,
                    self.stores,
                )?;
            }
        }

        Ok(())
    }
}

impl Drop for Update<'_> {
    fn drop(&mut self) {
        self.validate_inner().expect("Dropped invalid update");
    }
}
