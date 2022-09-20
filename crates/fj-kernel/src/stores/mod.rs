//! Append-only object storage

mod blocks;
mod handle;
mod store;

use crate::objects::GlobalCurve;

pub use self::{
    handle::{Handle, HandleWrapper, ObjectId},
    store::{Iter, Reservation, Store},
};

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
