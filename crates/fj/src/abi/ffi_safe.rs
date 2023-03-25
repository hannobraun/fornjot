//! FFI-safe versions of common `std` types.

use std::{
    alloc::{GlobalAlloc, Layout, System},
    fmt::{self, Debug, Display, Formatter},
    ops::Deref,
    ptr::NonNull,
};

// TODO with the new serialization focused approach to our FFI, is this module really necessary anymore?

/// A FFI-safe version of `Vec<T>`.
#[repr(C)]
pub(crate) struct Vec<T> {
    ptr: NonNull<T>,
    len: usize,
}

impl<T: Debug> Debug for Vec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &**self)
    }
}

impl<T: PartialEq> PartialEq for Vec<T> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T> From<std::vec::Vec<T>> for Vec<T> {
    fn from(mut items: std::vec::Vec<T>) -> Self {
        // Safety: To avoid accidental double-frees and other memory issues, we
        // need to go through a specific dance.
        unsafe {
            // first, get a pointer to the first element and its length
            let first_item = items.as_mut_ptr();
            let len = items.len();

            // next, tell Vec to forget about these items so it won't try to
            // run their destructors if we return early (e.g. via a panic).
            // We've now taken over ownership of the items, but *not* the Vec's
            // backing array.
            items.set_len(0);

            // Use the system allocator to create some space for our
            // FfiSafeVec's buffer.
            let layout = Layout::array::<T>(len).unwrap();
            let ptr: *mut T = System::default().alloc(layout).cast();
            let ptr = NonNull::new(ptr).expect("Allocation failed");

            // Now, we can copy the items across
            std::ptr::copy_nonoverlapping(first_item, ptr.as_ptr(), len);

            // the items are gone, time to free the original vec
            drop(items);

            Self { ptr, len }
        }
    }
}

impl<T: Clone> From<Vec<T>> for std::vec::Vec<T> {
    fn from(v: Vec<T>) -> Self {
        v.iter().map(Clone::clone).collect()
    }
}

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<T: Copy> From<Vec<T>> for Box<[T]> {
    fn from(v: Vec<T>) -> Self {
        Self::from(&*v)
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        std::vec::Vec::default().into()
    }
}

impl<T> FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let vec: std::vec::Vec<T> = iter.into_iter().collect();
        vec.into()
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        // Safety: We control "ptr" and "len", so we know they are always
        // initialized and within bounds.
        unsafe {
            let Self { ptr, len } = *self;
            std::slice::from_raw_parts(ptr.as_ptr(), len)
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        let Self { ptr, len } = *self;
        let ptr = ptr.as_ptr();

        for i in 0..self.len {
            // Safety: We control the "len" field, so the item we're accessing
            // is always within bounds. We also don't touch values after their
            // destructors are called.
            unsafe {
                let item = ptr.add(i);
                std::ptr::drop_in_place(item);
            }
        }

        // Safety: This vec is immutable, so we're using the same layout as the
        // original allocation. It's also not possible to touch the allocation
        // after Drop completes.
        unsafe {
            let layout = Layout::array::<T>(len).unwrap();
            System::default().dealloc(ptr.cast(), layout);
        }
    }
}

// Safety: We're Send+Sync as long as the underlying type is.
unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> serde::ser::Serialize for Vec<T>
where
    T: serde::ser::Serialize,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.deref().serialize(serializer)
    }
}

impl<'de, T> serde::de::Deserialize<'de> for Vec<T>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        Ok(std::vec::Vec::deserialize(deserializer)?.into())
    }
}

/// A FFI-safe version of `Box<str>`.
#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct String(Vec<u8>);

impl From<std::string::String> for String {
    fn from(s: std::string::String) -> Self {
        Self(s.into_bytes().into())
    }
}

impl From<String> for std::string::String {
    fn from(s: String) -> Self {
        s.to_string()
    }
}

impl From<String> for Box<str> {
    fn from(s: String) -> Self {
        Self::from(&*s)
    }
}
impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        **self == *other
    }
}

impl PartialEq<&str> for String {
    fn eq(&self, other: &&str) -> bool {
        *self == **other
    }
}

impl Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl Deref for String {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // Safety: The only way to create a FfiSafeString is from a valid Rust
        // string, so we can skip the UTF-8 checks.
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

/// A version of `Result` that is `#[repr(C)]`.
#[must_use]
#[repr(C)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E: Debug> Result<T, E> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(value) => value,
            Self::Err(e) => panic!("Unwrapped an Err({e:?})"),
        }
    }
}

impl<T, E> From<std::result::Result<T, E>> for Result<T, E> {
    fn from(result: std::result::Result<T, E>) -> Self {
        match result {
            Ok(ok) => Self::Ok(ok),
            Err(err) => Self::Err(err),
        }
    }
}

impl<T, E> From<Result<T, E>> for std::result::Result<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Result::Ok(ok) => Self::Ok(ok),
            Result::Err(err) => Self::Err(err),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Option<T> {
    Some(T),
    None,
}

impl<T> Option<T> {
    pub fn map<T2>(self, func: impl FnOnce(T) -> T2) -> Option<T2> {
        match self {
            Self::Some(value) => Option::Some(func(value)),
            Self::None => Option::None,
        }
    }
}

impl<T1, T2> From<std::option::Option<T1>> for Option<T2>
where
    T1: Into<T2>,
{
    fn from(opt: std::option::Option<T1>) -> Self {
        match opt {
            Some(value) => Self::Some(value.into()),
            None => Self::None,
        }
    }
}

impl<T> From<Option<T>> for std::option::Option<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Option::Some(value) => Some(value),
            Option::None => None,
        }
    }
}
