/// # A specific validation check on a given primitive
///
/// This trait is implemented once per validation check and primitive it applies
/// to. `Self` is the primitive, while `T` identifies the validation check.
pub trait ValidationCheck<T> {
    /// # Run the validation check on the implementing primitive
    fn check<'r>(primitive: &'r T) -> impl Iterator<Item = Self> + 'r;
}
