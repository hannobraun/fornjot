use std::convert::Infallible;

use crate::objects::{Curve, GlobalCurve};

use super::{Validate, ValidationConfig};

impl Validate for Curve {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Validate for GlobalCurve {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
