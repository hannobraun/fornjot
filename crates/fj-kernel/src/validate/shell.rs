use std::convert::Infallible;

use crate::objects::Shell;

use super::{Validate2, ValidationConfig};

impl Validate2 for Shell {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
