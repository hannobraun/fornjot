use std::convert::Infallible;

use crate::objects::Surface;

use super::{Validate2, ValidationConfig};

impl Validate2 for Surface {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
