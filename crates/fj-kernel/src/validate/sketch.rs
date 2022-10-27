use std::convert::Infallible;

use crate::objects::Sketch;

use super::{Validate2, ValidationConfig};

impl Validate2 for Sketch {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
