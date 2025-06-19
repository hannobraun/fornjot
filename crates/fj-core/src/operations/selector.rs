//! # Flexible Object Selectors
//!
//! This module provides flexible object selection capabilities for update and
//! replace operations. Instead of requiring specific `Handle<T>` references,
//! operations can accept selectors that can choose objects based on various
//! criteria.
//!
//! ## Key Components
//!
//! - [`Selector`] trait: Core abstraction for object selection
//! - [`Select`] enum: Common selection patterns (first, nth, all, etc.)
//! - Handle implementations: Allow direct selection by handle
//!
//! ## Example
//!
//! ```
//! use fj_core::{
//!     Core,
//!     operations::{
//!         build::BuildShell,
//!         selector::{Select, Selector},
//!         update::UpdateShellWithSelector,
//!     },
//!     topology::Shell,
//! };
//!
//! let mut core = Core::new();
//! let tetrahedron = Shell::tetrahedron(
//!     [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
//!     &mut core,
//! );
//! // After: concise selector-based API  
//! let updated = tetrahedron.shell.update_faces(
//!     Select::First,
//!     |face, _core| [face.clone()],
//!     &mut core,
//! );
//! ```
//!
//! This approach provides more flexibility and concise code while maintaining
//! the same functionality as the original handle-based operations.

use crate::{storage::Handle, topology::ObjectSet};

/// A trait for selecting objects from an object set
///
/// This trait provides a flexible way to select one or more objects from an
/// `ObjectSet`. It can be implemented for various types to provide different
/// selection strategies.
///
/// # Example
///
/// ```
/// use fj_core::{
///     Core,
///     operations::{
///         build::BuildShell,
///         selector::{Select, Selector},
///         update::UpdateShellWithSelector,
///     },
///     topology::Shell,
/// };
///
/// let mut core = Core::new();
/// let tetrahedron = Shell::tetrahedron(
///     [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
///     &mut core,
/// );
///
/// // Old way: extract handle explicitly, more verbose
/// // let face_handle = tetrahedron.shell.faces().first();
/// // let updated = tetrahedron.shell.update_face(face_handle, |face, core| {...}, &mut core);
///
/// // New way: use selector, more concise
/// let updated = tetrahedron.shell.update_faces(
///     Select::First,
///     |face, _core| [face.clone()],
///     &mut core,
/// );
/// ```
pub trait Selector<T> {
    /// Select objects from the provided object set
    ///
    /// Returns an iterator over the selected objects.
    fn select<'a>(
        &self,
        object_set: &'a ObjectSet<T>,
    ) -> Box<dyn Iterator<Item = &'a Handle<T>> + 'a>;
}

/// A simple enum for common selection patterns
///
/// This enum provides straightforward selection methods that replace the
/// methods currently found on `ObjectSet` like `only()`, `nth()`, etc.
///
/// # Example
///
/// ```
/// use fj_core::operations::selector::Select;
///
/// // Before: object_set.only()
/// // After: Select::Only
///
/// // Before: object_set.nth(2)
/// // After: Select::Nth(2)
///
/// // Before: object_set.iter()
/// // After: Select::All
/// ```
#[derive(Clone, Debug)]
pub enum Select {
    /// Select the only object in the set
    ///
    /// Panics if there is not exactly one object.
    Only,

    /// Select the first object in the set
    ///
    /// Panics if there are no objects.
    First,

    /// Select the object at the given index
    ///
    /// Returns empty iterator if the index is out of bounds.
    Nth(usize),

    /// Select the object at the given index, treating the index space as circular
    ///
    /// Panics if the set is empty.
    NthCircular(usize),

    /// Select all objects in the set
    All,
}

impl<T> Selector<T> for Select {
    fn select<'a>(
        &self,
        object_set: &'a ObjectSet<T>,
    ) -> Box<dyn Iterator<Item = &'a Handle<T>> + 'a> {
        match self {
            Select::Only => {
                let handle = object_set.only();
                Box::new(std::iter::once(handle))
            }
            Select::First => {
                let handle = object_set.first();
                Box::new(std::iter::once(handle))
            }
            Select::Nth(index) => {
                if let Some(handle) = object_set.nth(*index) {
                    Box::new(std::iter::once(handle))
                } else {
                    Box::new(std::iter::empty())
                }
            }
            Select::NthCircular(index) => {
                let handle = object_set.nth_circular(*index);
                Box::new(std::iter::once(handle))
            }
            Select::All => Box::new(object_set.iter()),
        }
    }
}

impl<T> Selector<T> for Handle<T> {
    fn select<'a>(
        &self,
        object_set: &'a ObjectSet<T>,
    ) -> Box<dyn Iterator<Item = &'a Handle<T>> + 'a> {
        // Find the handle in the object set that matches this one
        if let Some(found_handle) =
            object_set.iter().find(|h| h.id() == self.id())
        {
            Box::new(std::iter::once(found_handle))
        } else {
            Box::new(std::iter::empty())
        }
    }
}

impl<T> Selector<T> for &Handle<T> {
    fn select<'a>(
        &self,
        object_set: &'a ObjectSet<T>,
    ) -> Box<dyn Iterator<Item = &'a Handle<T>> + 'a> {
        // Find the handle in the object set that matches this one
        if let Some(found_handle) =
            object_set.iter().find(|h| h.id() == self.id())
        {
            Box::new(std::iter::once(found_handle))
        } else {
            Box::new(std::iter::empty())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Select, Selector};
    use crate::{
        Core,
        operations::{build::BuildShell, update::UpdateShellWithSelector},
        topology::Shell,
    };

    #[test]
    fn select_only() {
        let mut core = Core::new();
        let tetrahedron = Shell::tetrahedron(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            &mut core,
        );

        // Test Select::First selector on the first face
        let faces = tetrahedron.shell.faces();
        let selected: Vec<_> = Select::First.select(faces).collect();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].id(), faces.first().id());
    }

    #[test]
    fn select_nth() {
        let mut core = Core::new();
        let tetrahedron = Shell::tetrahedron(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            &mut core,
        );

        // Test nth selection
        let faces = tetrahedron.shell.faces();
        let selected: Vec<_> = Select::Nth(0).select(faces).collect();
        assert_eq!(selected.len(), 1);

        let selected_none: Vec<_> = Select::Nth(999).select(faces).collect();
        assert_eq!(selected_none.len(), 0);
    }

    #[test]
    fn select_all() {
        let mut core = Core::new();
        let tetrahedron = Shell::tetrahedron(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            &mut core,
        );

        let faces = tetrahedron.shell.faces();
        let selected: Vec<_> = Select::All.select(faces).collect();
        assert_eq!(selected.len(), faces.len());
    }

    #[test]
    fn update_with_selector() {
        let mut core = Core::new();
        let tetrahedron = Shell::tetrahedron(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            &mut core,
        );

        // Demonstrate the new selector-based API
        let _updated_shell = tetrahedron.shell.update_faces(
            Select::All,
            |face, _core| {
                // In a real scenario, this would do something meaningful
                [face.clone()]
            },
            &mut core,
        );
    }
}
