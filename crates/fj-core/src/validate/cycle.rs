use crate::{
    objects::Cycle,
    validation::{ValidationCheck, ValidationConfig, ValidationError},
};

use super::Validate;

impl Validate for Cycle {
    fn validate(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        errors.extend(self.check(config).map(Into::into));
    }
}
