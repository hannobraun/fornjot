/// Contextual information passed to a [`Model`][crate::models::Model] when it
/// is being initialized.
///
/// Check out the [`ContextExt`] trait for some helper methods.
pub trait Context {
    /// Get an argument that was passed to this model.
    fn get_argument(&self, name: &str) -> Option<&str>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_is_object_safe() {
        let _: &dyn Context;
    }
}
