use crate::{
    get::Get,
    insert::Insert,
    objects::Objects,
    services::Service,
    storage::Handle,
    validate::{Validate, ValidationError},
};

use super::{HasPartial, MergeWith, Partial, Replace};

/// Can be used everywhere either a partial or full objects are accepted
///
/// Some convenience methods are available for specific instances of
/// `MaybePartial` (like, `MaybePartial<Curve>`, or `MaybePartial<Vertex>`).
///
/// # Implementation Note
///
/// The set of available convenience methods is far from complete. Please feel
/// free to just add more, if you need them.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum MaybePartial<T: HasPartial> {
    /// A full object
    Full(Handle<T>),

    /// A partial object
    Partial(T::Partial),
}

impl<T: HasPartial> MaybePartial<T> {
    /// Indicate whether this is a full object
    pub fn is_full(&self) -> bool {
        if let Self::Full(_) = self {
            return true;
        }

        false
    }

    /// Indicate whether this is a partial object
    pub fn is_partial(&self) -> bool {
        if let Self::Partial(_) = self {
            return true;
        }

        false
    }

    /// If this is a partial object, update it
    ///
    /// This is useful whenever a partial object can infer something about its
    /// parts from other parts, and wants to update what was inferred, in case
    /// it *can* be updated.
    pub fn update_partial(
        self,
        f: impl FnOnce(T::Partial) -> T::Partial,
    ) -> Self {
        match self {
            Self::Partial(partial) => Self::Partial(f(partial)),
            _ => self,
        }
    }

    /// Return or build a full object
    ///
    /// If this already is a full object, it is returned. If this is a partial
    /// object, the full object is built from it, using [`Partial::build`].
    pub fn into_full(self, objects: &mut Service<Objects>) -> Handle<T>
    where
        T: Insert,
        ValidationError: From<<T as Validate>::Error>,
    {
        match self {
            Self::Partial(partial) => partial.build(objects).insert(objects),
            Self::Full(full) => full,
        }
    }

    /// Return or convert a partial object
    ///
    /// If this already is a partial object, is is returned. If this is a full
    /// object, it is converted into a partial object using
    /// [`HasPartial::to_partial`].
    pub fn into_partial(self) -> T::Partial {
        match self {
            Self::Partial(partial) => partial,
            Self::Full(full) => full.to_partial(),
        }
    }
}

impl<T> Default for MaybePartial<T>
where
    T: HasPartial,
    T::Partial: Default,
{
    fn default() -> Self {
        Self::Partial(T::Partial::default())
    }
}

impl<T> MergeWith for MaybePartial<T>
where
    T: HasPartial,
    T::Partial: MergeWith,
{
    fn merge_with(self, other: impl Into<Self>) -> Self {
        match (self, other.into()) {
            (Self::Full(a), Self::Full(b)) => Self::Full(a.merge_with(b)),
            (Self::Full(full), Self::Partial(_))
            | (Self::Partial(_), Self::Full(full)) => Self::Full(full),
            (Self::Partial(a), Self::Partial(b)) => {
                Self::Partial(a.merge_with(b))
            }
        }
    }
}

impl<T, R> Replace<R> for MaybePartial<T>
where
    T: HasPartial + Get<R>,
    T::Partial: Replace<R>,
{
    fn replace(&mut self, object: Handle<R>) -> &mut Self {
        match self {
            Self::Full(full) => {
                if full.get().id() != object.id() {
                    let mut partial = full.to_partial();
                    partial.replace(object);
                    *self = Self::Partial(partial);
                }
            }
            Self::Partial(partial) => {
                partial.replace(object);
            }
        }

        self
    }
}

impl<T> From<Handle<T>> for MaybePartial<T>
where
    T: HasPartial,
{
    fn from(full: Handle<T>) -> Self {
        Self::Full(full)
    }
}

// Unfortunately, we can't add a blanket implementation from `T::Partial` for
// `MaybePartial<T>`, as that would conflict.
