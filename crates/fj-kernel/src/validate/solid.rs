use std::convert::Infallible;

use crate::{objects::Solid, storage::Store};

use super::{Validate2, ValidationConfig};

impl Validate2 for Solid {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &Store<Self>,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
