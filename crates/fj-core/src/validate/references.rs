use std::collections::HashMap;
use std::hash::Hash;

use crate::storage::Handle;

use super::ValidationError;

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

pub trait ValidateReferences<T> {
    fn validate(&self, errors: &mut Vec<ValidationError>);
}

/// Find errors and convert to [`ValidationError`]
#[macro_export]
macro_rules! validate_references {
    ($errors:ident, $error_ty:ty;$($counter:ident, $err:expr;)*) => {
        $(
            if $counter.has_multiple() {
                $errors.push(Into::<$error_ty>::into($err).into());
            }
        )*
    };
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ReferenceCountError {
    /// [`Region`] referenced by more than one [`Face`]
    #[error("[`Region`] referenced by more than one [`Face`]")]
    Region,
    /// [`Face`] referenced by more than one [`crate::objects::Shell`]
    #[error("[`Face`] referenced by more than one [`crate::objects::Shell`]")]
    Face,
    /// [`HalfEdge`] referenced by more than one [`Cycle`]
    #[error("[`HalfEdge`] referenced by more than one [`Cycle`]")]
    HalfEdge,
    /// [`Cycle`] referenced by more than one [`Region`]
    #[error("[`Cycle`] referenced by more than one [`Region`]")]
    Cycle,
}
