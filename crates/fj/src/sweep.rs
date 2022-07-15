#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Shape, Shape2d};

/// A sweep of a 2-dimensional shape along straight path
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Sweep {
    /// The 2-dimensional shape being swept
    shape: Shape2d,

    /// The length and direction of the sweep
    path: [f64; 3],
}

impl Sweep {
    /// Create a `Sweep` along a straight path
    pub fn from_path(shape: Shape2d, path: [f64; 3]) -> Self {
        Self { shape, path }
    }

    /// Access the shape being swept
    pub fn shape(&self) -> &Shape2d {
        &self.shape
    }

    /// Access the path of the sweep
    pub fn path(&self) -> [f64; 3] {
        self.path
    }
}

impl From<Sweep> for Shape {
    fn from(shape: Sweep) -> Self {
        Self::Sweep(shape)
    }
}
