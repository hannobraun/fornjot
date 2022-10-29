use std::convert::Infallible;

use crate::objects::Cycle;

use super::{Validate2, ValidationConfig};

impl Validate2 for Cycle {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
