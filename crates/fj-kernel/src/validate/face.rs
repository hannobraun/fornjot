use std::convert::Infallible;

use crate::objects::Face;

use super::{Validate2, ValidationConfig};

impl Validate2 for Face {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
