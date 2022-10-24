use crate::{objects::Solid, storage::Store};

use super::{Validate2, ValidationConfig, ValidationError};

impl Validate2 for Solid {
    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}
