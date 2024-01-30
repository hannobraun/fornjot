use crate::{
    objects::{
        Curve, Cycle, Face, HalfEdge, Objects, Region, Shell, Sketch, Solid,
        Surface, Vertex,
    },
    storage::{Handle, HandleWrapper, ObjectId},
    validate::{Validate, ValidationConfig, ValidationError},
};

macro_rules! any_object {
    ($($ty:ident, $name:expr, $store:ident;)*) => {
        /// An enum that can hold object
        ///
        /// This enum is generic over the form that the object takes. An
        /// `AnyObject<Bare>` contains bare objects, like `Curve`. An
        /// `AnyObject<BehindHandle>` contains handles, like `Handle<Curve>`.
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        pub enum AnyObject<F: Form> {
            $(
                #[doc = concat!("A ", $name)]
                $ty(F::Form<$ty>),
            )*
        }

        impl AnyObject<Stored> {
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

            /// Validate the object with a pre-defined validation configuration
            pub fn validate_with_config(&self, config: &ValidationConfig, errors: &mut Vec<ValidationError>) {
                match self {
                    $(
                        Self::$ty(object) => object.validate_with_config(config, errors),
                    )*
                }
            }
        }

        impl AnyObject<WithHandle> {
            /// Insert the object into its respective store
            pub fn insert(self, objects: &mut Objects) -> AnyObject<Stored> {
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

        impl From<AnyObject<WithHandle>> for AnyObject<Stored> {
            fn from(object: AnyObject<WithHandle>) -> Self {
                match object {
                    $(
                        AnyObject::$ty((handle, _)) => Self::$ty(handle.into()),
                    )*
                }
            }
        }

        $(
            impl From<$ty> for AnyObject<Bare> {
                fn from(object: $ty) -> Self {
                    Self::$ty(object)
                }
            }

            impl From<Handle<$ty>> for AnyObject<Stored> {
                fn from(object: Handle<$ty>) -> Self {
                    Self::$ty(object.into())
                }
            }

            impl From<(Handle<$ty>, $ty)> for AnyObject<WithHandle> {
                fn from((handle, object): (Handle<$ty>, $ty)) -> Self {
                    Self::$ty((handle.into(), object))
                }
            }
        )*
    };
}

any_object!(
    Curve, "curve", curves;
    Cycle, "cycle", cycles;
    Face, "face", faces;
    HalfEdge, "half-edge", half_edges;
    Region, "region", regions;
    Shell, "shell", shells;
    Sketch, "sketch", sketches;
    Solid, "solid", solids;
    Surface, "surface", surfaces;
    Vertex, "vertex", vertices;
);

/// The form that an object can take
///
/// An object can be bare ([`Bare`]), behind a [`Handle`] ([`Stored`]), or can
/// take the form of a handle *and* an object [`WithHandle`].
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
pub struct Stored;

impl Form for Stored {
    type Form<T> = HandleWrapper<T>;
}

/// Implementation of [`Form`] for objects that are paired with their handle
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WithHandle;

impl Form for WithHandle {
    type Form<T> = (HandleWrapper<T>, T);
}
