pub type Point<const D: usize> = nalgebra::Point<f64, D>;
pub type Vector<const D: usize> = nalgebra::SVector<f64, D>;
pub type Transform = parry3d_f64::math::Isometry<f64>;
