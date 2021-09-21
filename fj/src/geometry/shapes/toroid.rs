use crate::geometry::{operations::Sweep, shapes::Circle};

pub type Toroid<T> = Sweep<T, Circle>;
