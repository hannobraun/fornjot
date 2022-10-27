use std::convert::Infallible;

use crate::objects::Solid;

use super::{Validate2, ValidationConfig};

impl Validate2 for Solid {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
