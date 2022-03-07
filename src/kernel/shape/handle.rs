use std::{hash::Hash, rc::Rc};

#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub struct Handle<T>(HandleInner<T>);

impl<T> Handle<T> {
    pub(super) fn new(value: T) -> Self {
        Self(Rc::new(value))
    }

    pub(super) fn inner(&self) -> HandleInner<T> {
        self.0.clone()
    }

    pub fn get(&self) -> &T {
        &*self.0
    }
}

impl<T> PartialEq for Handle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.get().eq(other.get())
    }
}

impl<T> Hash for Handle<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state)
    }
}

pub(super) type HandleInner<T> = Rc<T>;
