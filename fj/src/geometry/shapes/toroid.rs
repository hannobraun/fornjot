use crate::geometry::{operations::Sweep, shapes::Circle};

/// A toroid
///
/// Defined as a sweep of a shape around a circle.
pub type Toroid<T> = Sweep<T, Circle>;
