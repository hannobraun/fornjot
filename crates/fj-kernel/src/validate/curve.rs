use std::convert::Infallible;

use crate::{
    objects::{Curve, GlobalCurve},
    storage::Store,
};

use super::{Validate2, ValidationConfig};

impl Validate2 for Curve {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Validate2 for GlobalCurve {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
