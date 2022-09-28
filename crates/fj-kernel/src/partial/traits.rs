use crate::stores::Stores;

/// Implemented for types that are partial objects
///
/// # Implementation Note
///
/// It would be nicer to require a conversion from `&Self` into the partial
/// form, but I think we need a `where` clause on the associated type to specify
/// that, which is unstable. It should become stable soon though, together with
/// generic associated types:
/// <https://github.com/rust-lang/rust/issues/44265>
pub trait HasPartial: Into<Self::Partial> {
    /// The full version of this partial object
    type Partial;

    /// Build a full object from the partial object
    fn from_partial(partial: Self::Partial, stores: &Stores) -> Self;
}
