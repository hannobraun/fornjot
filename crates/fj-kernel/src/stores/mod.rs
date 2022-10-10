//! Append-only object storage

mod blocks;
mod handle;
mod store;

pub use self::{
    handle::{Handle, HandleWrapper, ObjectId},
    store::{Iter, Reservation, Store},
};
