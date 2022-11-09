use std::convert::Infallible;

use crate::objects::Solid;

use super::{Validate, ValidationConfig};

impl Validate for Solid {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
