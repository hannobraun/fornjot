use crate::{
    objects::{Curve, GlobalCurve},
    storage::Store,
};

use super::{Validate2, ValidationConfig, ValidationError};

impl Validate2 for Curve {
    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate2 for GlobalCurve {
    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}
