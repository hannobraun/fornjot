use crate::{
    objects::{
        Curve, Cycle, Face, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge,
        Shell, Sketch, Solid, Surface, SurfaceVertex, Vertex,
    },
    storage::Handle,
};

macro_rules! object {
    ($($ty:ident, $name:expr;)*) => {
        /// An object
        ///
        /// This enum is generic over the form that the object takes. An
        /// `Object<Bare>` contains bare objects, like `Curve`. An
        /// `Object<BehindHandle>` contains handles, like `Handle<Curve>`.
        #[derive(Clone)]
        pub enum Object<F: Form> {
            $(
                #[doc = concat!("A ", $name)]
                $ty(F::Form<$ty>),
            )*
        }

        $(
            impl From<$ty> for Object<Bare> {
                fn from(object: $ty) -> Self {
                    Self::$ty(object)
                }
            }

            impl From<Handle<$ty>> for Object<BehindHandle> {
                fn from(object: Handle<$ty>) -> Self {
                    Self::$ty(object)
                }
            }

            impl From<(Handle<$ty>, $ty)> for Object<WithHandle> {
                fn from((handle, object): (Handle<$ty>, $ty)) -> Self {
                    Self::$ty((handle, object))
                }
            }
        )*
    };
}

object!(
    Curve, "curve";
    Cycle, "cycle";
    Face, "face";
    GlobalCurve, "global curve";
    GlobalEdge, "global edge";
    GlobalVertex, "global vertex";
    HalfEdge, "half-edge";
    Shell, "shell";
    Sketch, "sketch";
    Solid, "solid";
    Surface, "surface";
    SurfaceVertex, "surface vertex";
    Vertex, "vertex";
);

/// The form that an object can take
///
/// An object can be bare (see [`Bare`]) or behind a [`Handle`] (see
/// [`BehindHandle`]).
pub trait Form {
    /// The form that the object takes
    type Form<T>;
}

/// Implementation of [`Form`] for bare objects
#[derive(Clone)]
pub struct Bare;

impl Form for Bare {
    type Form<T> = T;
}

/// Implementation of [`Form`] for objects behind a handle
#[derive(Clone)]
pub struct BehindHandle;

impl Form for BehindHandle {
    type Form<T> = Handle<T>;
}

/// Implementation of [`Form`] for objects that are paired with their handle
#[derive(Clone)]
pub struct WithHandle;

impl Form for WithHandle {
    type Form<T> = (Handle<T>, T);
}
