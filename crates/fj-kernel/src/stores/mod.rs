//! Append-only object storage

mod store;

pub use self::store::{Handle, Iter, Reservation, Store};
