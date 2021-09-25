pub type Point<const D: usize> = nalgebra::Point<f32, D>;
pub type Vector<const D: usize> = nalgebra::SVector<f32, D>;
pub type Matrix<const R: usize, const C: usize> = nalgebra::Matrix<
    f32,
    nalgebra::Const<R>,
    nalgebra::Const<C>,
    nalgebra::ArrayStorage<f32, R, C>,
>;
pub type Transform<const D: usize> =
    nalgebra::Transform<f32, nalgebra::TAffine, D>;
