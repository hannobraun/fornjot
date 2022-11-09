use std::convert::Infallible;

use crate::objects::Surface;

use super::{Validate, ValidationConfig};

impl Validate for Surface {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
