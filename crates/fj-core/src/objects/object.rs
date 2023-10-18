use crate::{
    objects::{
        Curve, Cycle, Face, HalfEdge, Objects, Region, Shell, Sketch, Solid,
        Surface, Vertex,
    },
    storage::{Handle, HandleWrapper, ObjectId},
    validate::{Validate, ValidationError},
};

macro_rules! object {
    ($($ty:ident, $name:expr, $store:ident;)*) => {
        /// An object
        ///
        /// This enum is generic over the form that the object takes. An
        /// `Object<Bare>` contains bare objects, like `Curve`. An
        /// `Object<BehindHandle>` contains handles, like `Handle<Curve>`.
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Object<F: Form> {
            $(
                #[doc = concat!("A ", $name)]
                $ty(F::Form<$ty>),
            )*
        }

        impl Object<BehindHandle> {
            /// Access the ID of the object
            pub fn id(&self) -> ObjectId {
                match self {
                    $(
                        Self::$ty(handle) => handle.id(),
                    )*
                }
            }

            /// Validate the object
            pub fn validate(&self, errors: &mut Vec<ValidationError>) {
                match self {
                    $(
                        Self::$ty(object) => object.validate(errors),
                    )*
                }
            }
        }

        impl Object<WithHandle> {
            /// Insert the object into its respective store
            pub fn insert(self, objects: &mut Objects) -> Object<BehindHandle> {
                match self {
                    $(
                        Self::$ty((handle, object)) => {
                            objects.$store.insert(
                                handle.clone().into(), object
                            );
                            handle.0.into()
                        }
                    )*
                }
            }
        }

        impl From<Object<WithHandle>> for Object<BehindHandle> {
            fn from(object: Object<WithHandle>) -> Self {
                match object {
                    $(
                        Object::$ty((handle, _)) => Self::$ty(handle.into()),
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
                    Self::$ty(object.into())
                }
            }

            impl From<(Handle<$ty>, $ty)> for Object<WithHandle> {
                fn from((handle, object): (Handle<$ty>, $ty)) -> Self {
                    Self::$ty((handle.into(), object))
                }
            }
        )*
    };
}

object!(
    Curve, "curve", curves;
    Cycle, "cycle", cycles;
    Face, "face", faces;
    HalfEdge, "edge", edges;
    Region, "region", regions;
    Shell, "shell", shells;
    Sketch, "sketch", sketches;
    Solid, "solid", solids;
    Surface, "surface", surfaces;
    Vertex, "vertex", vertices;
);

/// The form that an object can take
///
/// An object can be bare ([`Bare`]), behind a [`Handle`] ([`BehindHandle`]), or
/// can take the form of a handle *and* an object [`WithHandle`].
pub trait Form {
    /// The form that the object takes
    type Form<T>;
}

/// Implementation of [`Form`] for bare objects
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bare;

impl Form for Bare {
    type Form<T> = T;
}

/// Implementation of [`Form`] for objects behind a handle
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BehindHandle;

impl Form for BehindHandle {
    type Form<T> = HandleWrapper<T>;
}

/// Implementation of [`Form`] for objects that are paired with their handle
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WithHandle;

impl Form for WithHandle {
    type Form<T> = (HandleWrapper<T>, T);
}
