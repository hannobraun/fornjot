use std::{
    any::type_name,
    fmt::{self, Debug},
    marker::PhantomData,
    ops::Index,
    vec,
};

/// # Append-only store for topological primitives
///
/// Allows you to store topological primitives, which are then identified by a
/// stable [`Handle`]. Each instance of `Store` stores one specific type of
/// primitive. You don't actually have to set this up yourself though, as you
/// can just create an instance of [`Topology`] instead.
///
/// While nothing prevents you from creating multiple `Store`s per type of
/// primitive, Fornjot's design assumes that one store per type of primitive
/// exists. If you decide to create more, you must take care not to mix up
/// [`Handle`] instances from different stores.
///
/// [`Topology`]: crate::new::topology::Topology
#[derive(Debug)]
pub struct Store<T> {
    inner: Vec<T>,
}

impl<T> Store<T> {
    /// # Create a new instance of `Store`
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// # Push a new primitive into the store
    ///
    /// Returns a [`Handle`] that can be used to access the primitive later.
    /// See [`Store::get`] for more information on how to do that.
    pub fn push(&mut self, primitive: impl Into<T>) -> Handle<T> {
        let index = self.inner.len();
        self.inner.push(primitive.into());

        Handle {
            index,
            _t: PhantomData,
        }
    }

    /// # Access a primitive in the store
    ///
    /// Access a primitive in the store using a [`Handle`]. Those are returned
    /// by [`Store::push`].
    ///
    /// This method exists to make the means of accessing primitives in the
    /// store discoverable in the documentation, but it's not the recommended
    /// way to do so. Indexing the store using the handle using `Store`'s
    /// [`Index`] implementation is more convenient:
    ///
    /// ```
    /// use fj_core::{math::Point, new::topology::{Store, Vertex}};
    ///
    /// let mut store: Store<Vertex> = Store::new();
    /// let handle = store.push([0., 0., 0.]);
    ///
    /// // Access the primitive without using `Store::get`.
    /// let vertex = store[handle];
    /// ```
    pub fn get(&self, handle: &Handle<T>) -> &T {
        &self.inner[handle.index]
    }
}

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<Handle<T>> for Store<T> {
    type Output = T;

    fn index(&self, handle: Handle<T>) -> &Self::Output {
        self.get(&handle)
    }
}

impl<T> IntoIterator for Store<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

/// # A handle to a topological primitive
///
/// Topological primitives are stored inside [`Store`]. An instance of `Handle`
/// references a primitive that has been stored and can be used to access it.
///
/// The easiest way to do so, is using [`Store`]'s [`Index`] implementation:
///
/// ```
/// use fj_core::{math::Point, new::topology::{Store, Vertex}};
///
/// let mut store: Store<Vertex> = Store::new();
/// let handle = store.push([0., 0., 0.]);
///
/// let vertex = store[handle];
/// ```
///
/// Though it's also possible to use [`Store::get`] instead.
///
/// Handles are typed, to prevent mix-ups between different stores. However,
/// there's nothing built into `Handle` to prevent mix-ups between different
/// stores of the same type.
///
/// Fornjot's design assumes that only one store exists per type of primitive.
/// As long as the user upholds this assumption, there should be no issue. If a
/// user decides to use multiple sets of stores, they must take care not to mix
/// up handles between different stores of the same type.
#[derive(Eq, Ord, PartialOrd, PartialEq)]
pub struct Handle<T> {
    index: usize,
    _t: PhantomData<T>,
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Handle<T> {}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let primitive = {
            let full_name = type_name::<T>();

            full_name
                .rsplit_once("::")
                .map(|(_, name)| name)
                .unwrap_or(full_name)
        };

        write!(f, "Handle<{primitive}>({index})", index = self.index)
    }
}
