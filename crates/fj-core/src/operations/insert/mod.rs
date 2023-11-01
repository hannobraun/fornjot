//! Insert objects into their respective store

mod insert_trait;
mod is_inserted;

pub use self::{
    insert_trait::Insert,
    is_inserted::{IsInserted, IsInsertedNo, IsInsertedYes},
};
