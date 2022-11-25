use crate::{
    objects::{Object, Objects, WithHandle},
    storage::Handle,
};

use super::{Service, State};

impl State for Objects {
    type Command = InsertObject;
    type Event = ObjectToInsert;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        let event = ObjectToInsert {
            object: command.object,
        };
        events.push(event);
    }

    fn evolve(&mut self, event: &Self::Event) {
        // This operation being fallible goes against the spirit of the `evolve`
        // method. The reason for that is, that `Objects` is not fully adapted
        // to this new design yet. In the future, validation will most likely
        // move into its own service, making this operation infallible.
        event.object.clone().insert(self).unwrap();
    }
}

/// Command for `Service<Objects>`
///
/// You might prefer to use [`ServiceObjectsExt::insert`], which is a convenient
/// wrapper around `Service<Objects>::execute`.
#[derive(Clone, Debug)]
pub struct InsertObject {
    /// The object to insert
    pub object: Object<WithHandle>,
}

/// Event produced by `Service<Objects>`
#[derive(Clone, Debug)]
pub struct ObjectToInsert {
    /// The object to insert
    pub object: Object<WithHandle>,
}

/// Convenient API for `Service<Objects>`
pub trait ServiceObjectsExt {
    /// Insert an object
    fn insert<T>(&mut self, handle: Handle<T>, object: T)
    where
        (Handle<T>, T): Into<Object<WithHandle>>;
}

impl ServiceObjectsExt for Service<Objects> {
    fn insert<T>(&mut self, handle: Handle<T>, object: T)
    where
        (Handle<T>, T): Into<Object<WithHandle>>,
    {
        self.execute(InsertObject {
            object: (handle, object).into(),
        })
    }
}
