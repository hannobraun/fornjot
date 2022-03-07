use std::{hash::Hash, ops::Deref, rc::Rc};

#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub struct Handle<T>(HandleInner<T>);

impl<T> Handle<T> {
    pub(super) fn new(value: T) -> Self {
        Self(Rc::new(value))
    }

    pub(super) fn inner(&self) -> HandleInner<T> {
        self.0.clone()
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T> PartialEq for Handle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Hash for Handle<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

pub(super) type HandleInner<T> = Rc<T>;
