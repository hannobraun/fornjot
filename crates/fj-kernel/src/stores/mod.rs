//! Append-only object storage

mod blocks;
mod store;

use crate::objects::GlobalCurve;

pub use self::store::{Handle, Iter, ObjectId, Reservation, Store};

/// The available object stores
#[derive(Debug, Default)]
pub struct Stores {
    /// Store for [`GlobalCurve`]s
    pub global_curves: Store<GlobalCurve>,
}

impl Stores {
    /// Construct a new instance of `Stores`
    pub fn new() -> Self {
        Self::default()
    }
}
