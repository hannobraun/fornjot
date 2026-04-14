/// # A specific validation check on a given primitive
///
/// This trait is implemented once per validation check and primitive it applies
/// to. It is implemented on a type that represents the validation check and
/// carries information about a failure of that check. The type parameter `T` is
/// the primitive that is being checked.
pub trait ValidationCheck<T> {
    /// # Run the validation check on the provided primitive
    fn check<'r>(primitive: &'r T) -> impl Iterator<Item = Self> + 'r;
}
