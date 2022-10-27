use std::convert::Infallible;

use crate::objects::{GlobalVertex, SurfaceVertex, Vertex};

use super::{Validate2, ValidationConfig};

impl Validate2 for Vertex {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Validate2 for SurfaceVertex {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Validate2 for GlobalVertex {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
