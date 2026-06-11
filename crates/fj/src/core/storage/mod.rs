//! Append-only object storage

mod blocks;
mod handle;
mod store;

pub use self::{
    handle::{Handle, ObjectId},
    store::{Iter, Store},
};
