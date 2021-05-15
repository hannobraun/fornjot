use std::fmt;

use nalgebra::Point;

pub struct DebugPoint<const D: usize>(pub Point<f32, D>);

impl<const D: usize> fmt::Debug for DebugPoint<D> {
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
