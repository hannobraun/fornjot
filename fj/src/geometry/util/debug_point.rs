use std::fmt;

use nalgebra::{Point, Scalar};

pub struct DebugPoint<T, const D: usize>(pub Point<T, D>);

impl<T, const D: usize> fmt::Debug for DebugPoint<T, D>
where
    T: Scalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;

        for (i, v) in self.0.iter().enumerate() {
            write!(f, "{:.2}", v)?;
            if i < D - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")?;

        Ok(())
    }
}
