/// A two-dimensional shape
///
///
/// ## Equality
///
/// `Surface` contains no data and exists purely to be referenced via a
/// `Handle`, where `Handle::id` can be used to compare different instances of
/// it.
///
/// If `Surface` had `Eq`/`PartialEq` implementations, it containing no data
/// would mean that all instances of `Surface` would be considered equal. This
/// would be very error-prone.
///
/// If you need to reference a `Surface` from a struct that needs to derive
/// `Eq`/`Ord`/..., you can use `HandleWrapper<Vertex>` to do that. It will
/// use `Handle::id` to provide those `Eq`/`Ord`/... implementations.
#[derive(Clone, Debug, Default)]
pub struct Surface {}

impl Surface {
    /// Construct an instance of `Surface`
    pub fn new() -> Self {
        Self::default()
    }
}
