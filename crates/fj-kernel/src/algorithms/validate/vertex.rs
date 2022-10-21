use crate::{
    objects::{GlobalVertex, SurfaceVertex, Vertex},
    storage::Store,
};

use super::{Validate2, ValidationConfig, ValidationError};

impl Validate2 for Vertex {
    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate2 for SurfaceVertex {
    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate2 for GlobalVertex {
    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}
