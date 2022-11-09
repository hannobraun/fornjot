use std::convert::Infallible;

use crate::objects::Sketch;

use super::{Validate, ValidationConfig};

impl Validate for Sketch {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
