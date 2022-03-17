pub mod circle;
pub mod difference_2d;
pub mod group;
pub mod sketch;
pub mod sweep;
pub mod transform;

use fj_debug::DebugInfo;
use fj_kernel::shape::Shape;
use fj_math::{Aabb, Scalar};

/// Implemented by all shapes
pub trait ToShape {
    /// Compute the boundary representation of the shape
    fn to_shape(&self, tolerance: Scalar, debug: &mut DebugInfo) -> Shape;

    /// Access the axis-aligned bounding box of a shape
    ///
    /// If a shape is empty, its [`Aabb`]'s `min` and `max` points must be equal
    /// (but are otherwise not specified).
    fn bounding_volume(&self) -> Aabb<3>;
}

macro_rules! dispatch {
    ($($method:ident($($arg_name:ident: $arg_ty:ty,)*) -> $ret:ty;)*) => {
        impl ToShape for fj::Shape {
            $(
                fn $method(&self, $($arg_name: $arg_ty,)*) -> $ret {
                    match self {
                        Self::Shape2d(shape) => shape.$method($($arg_name,)*),
                        Self::Shape3d(shape) => shape.$method($($arg_name,)*),
                    }
                }
            )*
        }

        impl ToShape for fj::Shape2d {
            $(
                fn $method(&self, $($arg_name: $arg_ty,)*) -> $ret {
                    match self {
                        Self::Circle(shape) => shape.$method($($arg_name,)*),
                        Self::Difference(shape) => shape.$method($($arg_name,)*),
                        Self::Sketch(shape) => shape.$method($($arg_name,)*),
                    }
                }
            )*
        }

        impl ToShape for fj::Shape3d {
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
        tolerance: Scalar,
        debug: &mut DebugInfo,
    ) -> Shape;
    bounding_volume() -> Aabb<3>;
}
