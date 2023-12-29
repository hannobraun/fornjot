use std::collections::HashMap;
use std::hash::Hash;

use crate::storage::Handle;

#[derive(Default)]
pub struct ReferenceCounter<T>(HashMap<Handle<T>, i32>);

impl<T: Eq + PartialEq + Hash> ReferenceCounter<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_count(&mut self, object: Handle<T>) {
        self.0
            .entry(object)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn has_multiple(&self) -> bool {
        self.0.iter().any(|(_, count)| *count > 1)
    }
}
