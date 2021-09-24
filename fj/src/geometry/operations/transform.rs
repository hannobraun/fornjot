use nalgebra::{
    allocator::Allocator, Const, DefaultAllocator, DimNameAdd, DimNameSum,
    TAffine, U1,
};

/// Applies an affine transformation to a shape
///
/// `D` defines the dimensionality of the transformation. Typically,
/// transformations will be 2- or 3-dimensional.
pub struct Transform<T, const D: usize>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator:
        Allocator<f32, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    /// The shape being transformed
    pub shape: T,

    /// The affine transform
    pub transform: nalgebra::Transform<f32, TAffine, D>,
}
