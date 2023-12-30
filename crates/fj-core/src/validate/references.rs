use std::collections::HashMap;
use std::hash::Hash;

use crate::objects::{Cycle, Face, HalfEdge, Region, Shell};
use crate::storage::Handle;

#[derive(Default)]
pub struct ReferenceCounter<T, U>(HashMap<Handle<T>, Vec<Handle<U>>>);

impl<T: Eq + PartialEq + Hash, U> ReferenceCounter<T, U> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_count(&mut self, object: Handle<T>, found: Handle<U>) {
        self.0
            .entry(object)
            .and_modify(|references| references.push(found.clone()))
            .or_insert(vec![found]);
    }

    pub fn get_multiples(&self) -> Vec<MultipleReferences<T, U>> {
        self.0
            .iter()
            .filter(|(_, references)| references.len() > 1)
            .map(|(referenced, references)| MultipleReferences {
                referenced: referenced.clone(),
                references: references.to_vec(),
            })
            .collect()
    }
}

/// Find errors and convert to [`crate::validate::ValidationError`]
#[macro_export]
macro_rules! validate_references {
    ($errors:ident, $error_ty:ty;$($counter:ident, $err:ident;)*) => {
        $(
            $counter.get_multiples().iter().for_each(|multiple| {
                let reference_error = ReferenceCountError::$err { references: multiple.clone() };
                $errors.push(Into::<$error_ty>::into(reference_error).into());
            });
        )*
    };
}

/// Validation errors for when an object is referenced by multiple other objects. Each object
/// should only be referenced by a single other object  
#[derive(Clone, Debug, thiserror::Error)]
pub enum ReferenceCountError {
    /// [`crate::objects::Region`] referenced by more than one [`crate::objects::Face`]
    #[error(
        "[`Region`] referenced by more than one [`Face`]\n{references:#?}"
    )]
    Region {
        references: MultipleReferences<Region, Face>,
    },
    /// [`crate::objects::Face`] referenced by more than one [`crate::objects::Shell`]
    #[error("[`Face`] referenced by more than one [`Shell`]\n{references:#?}")]
    Face {
        references: MultipleReferences<Face, Shell>,
    },
    /// [`crate::objects::HalfEdge`] referenced by more than one [`crate::objects::Cycle`]
    #[error(
        "[`HalfEdge`] referenced by more than one [`Cycle`]\n{references:#?}"
    )]
    HalfEdge {
        references: MultipleReferences<HalfEdge, Cycle>,
    },
    /// [`crate::objects::Cycle`] referenced by more than one [`crate::objects::Region`]
    #[error(
        "[`Cycle`] referenced by more than one [`Region`]\n{references:#?}"
    )]
    Cycle {
        references: MultipleReferences<Cycle, Region>,
    },
}

pub struct MultipleReferences<T, U> {
    referenced: Handle<T>,
    references: Vec<Handle<U>>,
}

use std::fmt::Debug;

impl<T: Debug, U: Debug> Debug for MultipleReferences<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} referenced by {:?}",
            self.referenced, self.references
        )
    }
}

impl<T, U> Clone for MultipleReferences<T, U> {
    fn clone(&self) -> Self {
        Self {
            referenced: self.referenced.clone(),
            references: self.references.to_vec(),
        }
    }
}
