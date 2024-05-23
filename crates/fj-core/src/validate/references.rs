use std::{collections::HashMap, fmt};

use crate::{
    storage::Handle,
    topology::{Cycle, Face, HalfEdge, Region, Shell},
};

#[derive(Default)]
pub struct ReferenceCounter<T, U>(HashMap<Handle<T>, Vec<Handle<U>>>);

impl<T, U> ReferenceCounter<T, U> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn count_reference(
        &mut self,
        referenced: Handle<T>,
        reference: Handle<U>,
    ) {
        self.0.entry(referenced).or_default().push(reference);
    }

    pub fn find_multiples(&self) -> Vec<MultipleReferences<T, U>> {
        self.0
            .iter()
            .filter(|(_, referenced_by)| referenced_by.len() > 1)
            .map(|(object, referenced_by)| MultipleReferences {
                object: object.clone(),
                referenced_by: referenced_by.to_vec(),
            })
            .collect()
    }
}

/// Find errors and convert to [`crate::validate::ValidationError`]
#[macro_export]
macro_rules! validate_references {
    ($errors:ident, $error_ty:ty;$($counter:ident, $err:ident;)*) => {
        $(
            $counter.find_multiples().iter().for_each(|multiple| {
                let reference_error = ObjectNotExclusivelyOwned::$err { references: multiple.clone() };
                $errors.push(Into::<$error_ty>::into(reference_error).into());
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
pub enum ObjectNotExclusivelyOwned {
    /// [`Region`] referenced by more than one [`Face`]
    #[error("`Region` referenced by more than one `Face`\n{references}")]
    Region {
        references: MultipleReferences<Region, Face>,
    },
    /// [`Face`] referenced by more than one [`Shell`]
    #[error("`Face` referenced by more than one `Shell`\n{references}")]
    Face {
        references: MultipleReferences<Face, Shell>,
    },
    /// [`HalfEdge`] referenced by more than one [`Cycle`]
    #[error("`HalfEdge` referenced by more than one `Cycle`\n{references}")]
    HalfEdge {
        references: MultipleReferences<HalfEdge, Cycle>,
    },
    /// [`Cycle`] referenced by more than one [`Region`]
    #[error("`Cycle` referenced by more than one `Region`\n{references}")]
    Cycle {
        references: MultipleReferences<Cycle, Region>,
    },
}

#[derive(Clone, Debug)]
pub struct MultipleReferences<T, U> {
    object: Handle<T>,
    referenced_by: Vec<Handle<U>>,
}

impl<T, U> fmt::Display for MultipleReferences<T, U>
where
    T: fmt::Debug,
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} referenced by {:?}",
            self.object, self.referenced_by
        )
    }
}
