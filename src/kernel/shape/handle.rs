use std::{cell::Cell, hash::Hash, rc::Rc};

#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub struct Handle<T: Copy>(HandleInner<T>);

impl<T: Copy> Handle<T> {
    pub(super) fn new(value: T) -> Self {
        Self(Rc::new(Cell::new(value)))
    }

    pub(super) fn inner(&self) -> HandleInner<T> {
        self.0.clone()
    }

    pub fn get(&self) -> T {
        self.0.get()
    }
}

impl<T: Copy> PartialEq for Handle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.get().eq(&other.get())
    }
}

impl<T: Copy> Hash for Handle<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state)
    }
}

pub(super) type HandleInner<T> = Rc<Cell<T>>;
