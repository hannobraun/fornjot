//! # Fornjot CAD Operations
//!
//! This library is part of the [Fornjot] ecosystem. Fornjot is an open-source,
//! code-first CAD application; and collection of libraries that make up the CAD
//! application, but can be used independently.
//!
//! This library is an internal component of Fornjot. It is not relevant to end
//! users that just want to create CAD models.
//!
//! Fornjot models use the [`fj`] crate to define a shape. This crate provides
//! the connection between [`fj`] and the Fornjot kernel. It translates those
//! operations into terms the kernel can understand.
//!
//! [Fornjot]: https://www.fornjot.app/
//! [`fj`]: https://crates.io/crates/fj

#![warn(missing_docs)]

pub mod shape_processor;

mod difference_2d;
mod group;
mod sketch;
mod sweep;
mod transform;

use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    objects::Face,
    validation::{Validated, ValidationConfig, ValidationError},
};
use fj_math::Aabb;

/// Implemented for all operations from the [`fj`] crate
pub trait Shape {
    /// Compute the boundary representation of the shape
    fn to_shape(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Vec<Face>>, ValidationError>;

    /// Access the axis-aligned bounding box of a shape
    ///
    /// If a shape is empty, its [`Aabb`]'s `min` and `max` points must be equal
    /// (but are otherwise not specified).
    fn bounding_volume(&self) -> Aabb<3>;
}

macro_rules! dispatch {
    ($($method:ident($($arg_name:ident: $arg_ty:ty,)*) -> $ret:ty;)*) => {
        impl Shape for fj::Shape {
            $(
                fn $method(&self, $($arg_name: $arg_ty,)*) -> $ret {
                    match self {
                        Self::Shape2d(shape) => shape.$method($($arg_name,)*),
                        Self::Shape3d(shape) => shape.$method($($arg_name,)*),
                    }
                }
            )*
        }

        impl Shape for fj::Shape2d {
            $(
                fn $method(&self, $($arg_name: $arg_ty,)*) -> $ret {
                    match self {
                        Self::Difference(shape) => shape.$method($($arg_name,)*),
                        Self::Sketch(shape) => shape.$method($($arg_name,)*),
                    }
                }
            )*
        }

        impl Shape for fj::Shape3d {
            $(
                fn $method(&self, $($arg_name: $arg_ty,)*) -> $ret {
                    match self {
                        Self::Group(shape) => shape.$method($($arg_name,)*),
                        Self::Sweep(shape) => shape.$method($($arg_name,)*),
                        Self::Transform(shape) => shape.$method($($arg_name,)*),
                    }
                }
            )*
        }
    };
}

dispatch! {
    to_shape(
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Vec<Face>>, ValidationError>;
    bounding_volume() -> Aabb<3>;
}
