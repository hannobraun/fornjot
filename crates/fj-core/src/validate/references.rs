use std::{any::type_name_of_val, collections::HashMap, fmt};

use crate::storage::Handle;

#[derive(Default)]
pub struct ReferenceCounter<T, U>(HashMap<Handle<T>, Vec<Handle<U>>>);

impl<T, U> ReferenceCounter<T, U> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn count_reference(&mut self, to: Handle<T>, from: Handle<U>) {
        self.0.entry(to).or_default().push(from);
    }

    pub fn find_multiples(&self) -> Vec<ObjectNotExclusivelyOwned<T, U>> {
        self.0
            .iter()
            .filter(|(_, referenced_by)| referenced_by.len() > 1)
            .map(|(object, referenced_by)| ObjectNotExclusivelyOwned {
                object: object.clone(),
                referenced_by: referenced_by.to_vec(),
            })
            .collect()
    }
}

/// Find errors and convert to [`crate::validate::ValidationError`]
#[macro_export]
macro_rules! validate_references {
    ($errors:ident;$($counter:ident, $err:ident;)*) => {
        $(
            $counter.find_multiples().iter().for_each(|multiple| {
                let reference_error = ValidationError::$err(multiple.clone());
                $errors.push(reference_error.into());
            });
        )*
    };
}

/// Object that should be exclusively owned by another, is not
///
/// Some objects are expected to be "owned" by a single other object. This means
/// that only one reference to these objects must exist within the topological
/// object graph.
#[derive(Clone, Debug, thiserror::Error)]
pub struct ObjectNotExclusivelyOwned<T, U> {
    object: Handle<T>,
    referenced_by: Vec<Handle<U>>,
}

impl<T, U> fmt::Display for ObjectNotExclusivelyOwned<T, U>
where
    T: fmt::Debug,
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "`{}` ({:?}) referenced by multiple `{}` objects ({:?})",
            type_name_of_val(&self.object),
            self.object,
            type_name_of_val(&self.referenced_by),
            self.referenced_by
        )
    }
}
