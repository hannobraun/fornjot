//! API for building objects

// These are new-style builders that build on top of the partial object API.
mod cycle;
mod edge;
mod face;
mod shell;
mod sketch;
mod solid;
mod surface;
mod vertex;

use std::array;

pub use self::{
    cycle::CycleBuilder, edge::HalfEdgeBuilder, face::FaceBuilder,
    shell::ShellBuilder, sketch::SketchBuilder, solid::SolidBuilder,
    surface::SurfaceBuilder, vertex::VertexBuilder,
};

/// Pass objects to a builder method
///
/// Many builder methods receive objects as arguments, and many builder
/// arguments return objects back, based on their input. In the general case,
/// the number of objects passed and returned is usually arbitrary, but many
/// callers pass a specific number of objects, and expect the same number of
/// objects back.
///
/// This trait can be used to do exactly that. It is implemented for `Vec` and
/// arrays. When passing a `Vec`, a `Vec` is returned. When passing an array, an
/// array of the same size is returned.
pub trait ObjectArgument<T>: IntoIterator<Item = T> {
    /// The value returned, if the implementing type is passed on an argument
    ///
    /// The return value has the same length as the implementing type, but it is
    /// not necessarily of the same type. For this reason, this associated type
    /// is generic.
    type SameSize<R>;

    /// A return value that has one more element than the argument
    type SizePlusOne<R>;

    /// Return the number of objects
    fn num_objects(&self) -> usize;

    /// Create a return value by mapping the implementing type
    fn map<F, R>(self, f: F) -> Self::SameSize<R>
    where
        F: FnMut(T) -> R;

    /// Create a return value by mapping the implementing type
    ///
    /// Provides access to the (circular) next item.
    fn map_with_prev<F, R>(self, f: F) -> Self::SameSize<R>
    where
        F: FnMut(T, T) -> R,
        T: Clone;

    /// Create a return value with one more element
    fn map_plus_one<F, R>(self, item: R, f: F) -> Self::SizePlusOne<R>
    where
        F: FnMut(T) -> R;
}

impl<T> ObjectArgument<T> for Vec<T> {
    type SameSize<R> = Vec<R>;
    type SizePlusOne<R> = Vec<R>;

    fn num_objects(&self) -> usize {
        self.len()
    }

    fn map<F, R>(self, mut f: F) -> Self::SameSize<R>
    where
        F: FnMut(T) -> R,
    {
        let mut ret = Vec::new();

        for item in self {
            ret.push(f(item));
        }

        ret
    }

    fn map_with_prev<F, R>(self, mut f: F) -> Self::SameSize<R>
    where
        F: FnMut(T, T) -> R,
        T: Clone,
    {
        let mut prev = Vec::new();
        for i in 0..self.len() {
            prev.push(self[(i + self.len() - 1) % self.len()].clone());
        }

        let mut ret = Vec::new();
        for (i, item) in self.into_iter().enumerate() {
            let prev = prev[i].clone();
            ret.push(f(item, prev));
        }

        ret
    }

    fn map_plus_one<F, R>(self, item: R, f: F) -> Self::SizePlusOne<R>
    where
        F: FnMut(T) -> R,
    {
        let mut ret = self.map(f);
        ret.push(item);
        ret
    }
}

// This macro implements `ObjectArgument` for a number of array types. This
// should just be a single implementation, but while const generic expressions
// are still unstable, this is unfortunately not possible:
// <https://github.com/rust-lang/rust/issues/76560>
macro_rules! impl_object_argument_for_arrays {
    ($($len:expr, $len_plus_one:expr;)*) => {
        $(
            impl<T> ObjectArgument<T> for [T; $len] {
                type SameSize<R> = [R; $len];
                type SizePlusOne<R> = [R; $len_plus_one];

                fn num_objects(&self) -> usize {
                    self.len()
                }

                fn map<F, R>(self, f: F) -> Self::SameSize<R>
                where
                    F: FnMut(T) -> R,
                {
                    self.map(f)
                }

                fn map_with_prev<F, R>(self, mut f: F) -> Self::SameSize<R>
                where
                    F: FnMut(T, T) -> R,
                    T: Clone,
                {
                    let prev: [_; $len] = array::from_fn(|i| {
                        self[(i + self.len() - 1) % self.len()].clone()
                    });

                    let mut i = 0;
                    self.map(|item| {
                        let prev = prev[i].clone();
                        i += 1;

                        f(item, prev)
                    })
                }

                fn map_plus_one<F, R>(self, item: R, mut f: F)
                    -> Self::SizePlusOne<R>
                where
                    F: FnMut(T) -> R,
                {
                    let mut tmp = array::from_fn(|_| None);
                    for (i, item) in self.into_iter().enumerate() {
                        tmp[i] = Some(f(item));
                    }

                    tmp[tmp.len() - 1] = Some(item);

                    tmp.map(Option::unwrap)
                }
            }
        )*
    };
}

impl_object_argument_for_arrays!(
    0, 1;
    1, 2;
    2, 3;
    3, 4;
    4, 5;
    5, 6;
    6, 7;
    7, 8;
);
