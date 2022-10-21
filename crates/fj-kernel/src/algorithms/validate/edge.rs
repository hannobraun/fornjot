use crate::{
    objects::{GlobalEdge, HalfEdge},
    storage::Store,
};

use super::{Validate2, ValidationConfig, ValidationError};

impl Validate2 for HalfEdge {
    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate2 for GlobalEdge {
    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}
