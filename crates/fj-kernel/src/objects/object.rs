use crate::{
    insert::Insert,
    objects::{
        Curve, Cycle, Face, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge,
        Objects, Shell, Sketch, Solid, Surface, SurfaceVertex, Vertex,
    },
    storage::Handle,
    validate::{Validate, ValidationError},
};

macro_rules! object {
    ($($ty:ident, $name:expr, $store:ident;)*) => {
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

        impl Object<WithHandle> {
            /// Insert the object into its respective store
            pub fn insert(
                self,
                objects: &Objects,
            ) -> Result<Object<BehindHandle>, ValidationError>
            where
                $(
                    crate::objects::$ty: Insert,
                    ValidationError: From<<$ty as Validate>::Error>,
                )*
            {
                match self {
                    $(
                        Self::$ty((handle, object)) => {
                            objects.$store.insert(handle.clone(), object)?;
                            Ok(handle.into())
                        }
                    )*
                }
            }
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
    Curve, "curve", curves;
    Cycle, "cycle", cycles;
    Face, "face", faces;
    GlobalCurve, "global curve", global_curves;
    GlobalEdge, "global edge", global_edges;
    GlobalVertex, "global vertex", global_vertices;
    HalfEdge, "half-edge", half_edges;
    Shell, "shell", shells;
    Sketch, "sketch", sketches;
    Solid, "solid", solids;
    Surface, "surface", surfaces;
    SurfaceVertex, "surface vertex", surface_vertices;
    Vertex, "vertex", vertices;
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
