/// A trait implemented for all object types
///
/// This trait is implemented for both `T` and `Handle<T>`, where `T` is the
/// type of any bare object. The `BareObject` associated type provides access to
/// the bare object type.
///
/// This is a piece of infrastructure that is useful for other traits, which
/// would otherwise have to duplicate its functionality. Users are unlikely to
/// be affected by this trait.
pub trait IsObject {
    /// The type of the bare object
    type BareObject;
}
