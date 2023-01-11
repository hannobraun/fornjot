//! API for building objects

// These are new-style builders that build on top of the partial object API.
mod curve;
mod cycle;
mod edge;
mod face;
mod shell;
mod sketch;
mod solid;
mod surface;
mod vertex;

pub use self::{
    curve::CurveBuilder,
    cycle::CycleBuilder,
    edge::{GlobalEdgeBuilder, HalfEdgeBuilder},
    face::FaceBuilder,
    shell::ShellBuilder,
    sketch::SketchBuilder,
    solid::SolidBuilder,
    surface::SurfaceBuilder,
    vertex::{GlobalVertexBuilder, SurfaceVertexBuilder, VertexBuilder},
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

    /// Create a return value by mapping the implementing type
    fn map<F, R>(self, f: F) -> Self::SameSize<R>
    where
        F: FnMut(T) -> R;
}

impl<T> ObjectArgument<T> for Vec<T> {
    type SameSize<R> = Vec<R>;

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
}

impl<T, const N: usize> ObjectArgument<T> for [T; N] {
    type SameSize<R> = [R; N];

    fn map<F, R>(self, f: F) -> Self::SameSize<R>
    where
        F: FnMut(T) -> R,
    {
        self.map(f)
    }
}
