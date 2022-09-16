//! Append-only object storage

mod store;

pub use self::store::{Handle, Iter, Reservation, Store};

/// The available object stores
#[derive(Debug, Default)]
pub struct Stores {}

impl Stores {
    /// Construct a new instance of `Stores`
    pub fn new() -> Self {
        Self::default()
    }
}
