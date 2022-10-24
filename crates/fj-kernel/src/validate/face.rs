use crate::{objects::Face, storage::Store};

use super::{Validate2, ValidationConfig, ValidationError};

impl Validate2 for Face {
    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}
