use std::marker::PhantomData;

pub struct Store<T> {
    _t: PhantomData<T>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self { _t: PhantomData }
    }
}
