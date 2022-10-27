use std::convert::Infallible;

use crate::{
    objects::{GlobalVertex, SurfaceVertex, Vertex},
    storage::Store,
};

use super::{Validate2, ValidationConfig};

impl Validate2 for Vertex {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Validate2 for SurfaceVertex {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Validate2 for GlobalVertex {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
