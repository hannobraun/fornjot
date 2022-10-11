use crate::{Shape, Shape2d};

/// A sweep of a 2-dimensional shape along straight path
///
/// # Examples
///
/// Convenient syntax for this operation is available through [`crate::syntax`].
///
/// ``` rust
/// # let shape = fj::Sketch::from_points(vec![[0., 0.], [1., 0.], [0., 1.]]);
/// use fj::syntax::*;
///
/// // `shape` can be anything that converts to `fj::Shape2d`
/// let group = shape.sweep([0., 0., 1.]);
/// ```
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
