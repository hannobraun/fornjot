use crate::{models::Error, abi::ffi_safe::Result};

/// Contextual information passed to a [`Model`][crate::models::Model] when it
/// is being initialized.
pub trait Context {
    /// Get an argument that was passed to this model.
    fn get_argument(&self, name: &str) -> Result<Option<&str>, Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_is_object_safe() {
        let _: &dyn Context;
    }
}
