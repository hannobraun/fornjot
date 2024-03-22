use crate::{
    geometry::Geometry,
    objects::{
        Curve, Cycle, Face, HalfEdge, Objects, Region, Shell, Sketch, Solid,
        Surface, Vertex,
    },
    storage::{Handle, ObjectId},
    validate::Validate,
    validation::{ValidationConfig, ValidationError},
};

macro_rules! any_object {
    ($($ty:ident, $name:expr, $store:ident;)*) => {
        /// An enum that can hold any object
        ///
        /// This enum is generic over the form that the object takes. An
        /// `AnyObject<Bare>` contains a bare objects, for example `Curve`. An
        /// `AnyObject<Stored>` contains a handle referencing a stored object,
        /// for example `Handle<Curve>`.
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

            /// Validate the object with a pre-defined validation configuration
            pub fn validate(&self,
                config: &ValidationConfig,
                errors: &mut Vec<ValidationError>,
                geometry: &Geometry,
            ) {
                match self {
                    $(
                        Self::$ty(object) => object.validate(
                            config,
                            errors,
                            geometry,
                        ),
                    )*
                }
            }
        }

        impl AnyObject<AboutToBeStored> {
            /// Insert the object into its respective store
            pub fn insert(self, objects: &mut Objects) -> AnyObject<Stored> {
                match self {
                    $(
                        Self::$ty((handle, object)) => {
                            objects.$store.insert(
                                handle.clone().into(), object
                            );
                            handle.into()
                        }
                    )*
                }
            }
        }

        impl From<AnyObject<AboutToBeStored>> for AnyObject<Stored> {
            fn from(object: AnyObject<AboutToBeStored>) -> Self {
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

            impl From<(Handle<$ty>, $ty)> for AnyObject<AboutToBeStored> {
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
/// This is used together with [`AnyObject`].
///
/// An object can be bare ([`Bare`]), stored ([`Stored`]), or it can be about to
/// be - but not yet - stored ([`AboutToBeStored`]).
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

/// Implementation of [`Form`] for stored objects
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Stored;

impl Form for Stored {
    type Form<T> = Handle<T>;
}

/// Implementation of [`Form`] for objects that are about to be stored
///
/// When storing an object, a [`Handle`] instance is generated first. Then both
/// that [`Handle`] instance and the bare object are sent to the object service,
/// for storage.
///
/// This is the one use case where this form is required.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AboutToBeStored;

impl Form for AboutToBeStored {
    type Form<T> = (Handle<T>, T);
}
