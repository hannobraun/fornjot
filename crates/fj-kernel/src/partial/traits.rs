use crate::stores::Stores;

/// Implemented for types that are partial objects
///
/// # Implementation Note
///
/// This trait is usually implemented for object types themselves, but types
/// that are already managed in the centralized object storage ([#1021])
/// implement this trait for `Handle<T>` instead. This is necessary, due to the
/// use of this type in [`MaybePartial`], but leads to some not so nice
/// inconsistencies.
///
/// Once [#1021] is addressed and all types are managed in the centralized
/// object storage, this should be changed to all object types implement this
/// directly.
///
/// [#1021]: https://github.com/hannobraun/Fornjot/issues/1021
/// [`MaybePartial`]: super::MaybePartial
pub trait HasPartial {
    /// The full version of this partial object
    type Partial: Partial<Full = Self>;

    /// Create an empty partial variant of this object
    ///
    /// This function exists just for convenience, and will just return a
    /// [`Default`] version of the partial object.
    fn partial() -> Self::Partial {
        Self::Partial::default()
    }

    /// Convert this object into its partial variant
    ///
    /// All fields of the partial variant are set from this object. This is
    /// useful when creating a new object that needs to share parts of an
    /// existing one.
    fn to_partial(&self) -> Self::Partial {
        self.into()
    }
}

/// Implemented for partial objects
///
/// # Implementation Note
///
/// It would be nicer to require an [`Into`] bound instead of [`From`] (see
/// documentation of those types for more information). But I think we'd need a
/// `where` clause on the associated type to specify that, which is unstable. It
/// should become stable soon though, together with generic associated types:
/// <https://github.com/rust-lang/rust/issues/44265>
pub trait Partial: Default + for<'a> From<&'a Self::Full> {
    /// The type representing the full variant of this object
    type Full;

    /// Build a full object from this partial one
    ///
    /// Implementations of this method will typically try to infer any missing
    /// parts of the partial object, but this is not possible in all cases. In
    /// such cases, implementations of this method may panic.
    ///
    /// Calling `build` on a partial object that can't infer its missing parts
    /// is considered a programmer error, hence why this method doesn't return a
    /// [`Result`].
    fn build(self, stores: &Stores) -> Self::Full;
}
