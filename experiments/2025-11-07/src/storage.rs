use std::marker::PhantomData;

/// # A handle to a shared topological object
///
/// Handles confer an identity to a topological object, which makes it possible
/// to identify them cheaply and unambiguously.
///
/// This concept of identity allows for clear semantic expression within the
/// object graph, by making it possible to distinguish between identity and mere
/// coincidental equality. It also enables easy caching and reusing of object
/// approximations.
///
/// Both of these properties combined, guard against issues of numerical
/// inaccuracy. In fact, they resolve those issues unambiguously, through the
/// semantic information in the object graph, instead of relying on inaccurate
/// measures like tolerance values.
pub struct Handle<T: ?Sized> {
    _t: PhantomData<T>,
}
