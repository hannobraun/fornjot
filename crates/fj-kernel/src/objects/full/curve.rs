use crate::geometry::path::SurfacePath;

/// A curve, defined in local surface coordinates
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Curve {
    path: SurfacePath,
}

impl Curve {
    /// Construct a new instance of `Curve`
    pub fn new(path: SurfacePath) -> Self {
        Self { path }
    }

    /// Access the path that defines the curve
    pub fn path(&self) -> SurfacePath {
        self.path
    }
}
