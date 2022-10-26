use crate::{objects::Sketch, storage::Store};

use super::{Validate2, ValidationConfig, ValidationError};

impl Validate2 for Sketch {
    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}
