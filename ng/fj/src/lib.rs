/// A 3-dimensional shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape3d {
    /// A cube
    Cube(Cube),
}

impl From<Cube> for Shape3d {
    fn from(cube: Cube) -> Self {
        Self::Cube(cube)
    }
}

/// A cube
#[derive(Debug)]
#[repr(C)]
pub struct Cube {
    /// The side length of the cube
    pub size: f32,
}
