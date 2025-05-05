use std::{cmp::Ordering, fmt, ops::Deref, rc::Rc};

pub struct Handle<T> {
    inner: Rc<T>,
}

impl<T> Handle<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let [self_ptr, other_ptr] =
            [self, other].map(|handle| Rc::as_ptr(&handle.inner));

        self_ptr.cmp(&other_ptr)
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> fmt::Debug for Handle<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Handle")
            .field("inner", &Rc::as_ptr(&self.inner))
            .finish()?;

        Ok(())
    }
}
