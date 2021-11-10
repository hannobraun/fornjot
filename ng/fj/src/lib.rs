/// A shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape {
    /// A cube
    Cube(Cube),
}

impl From<Cube> for Shape {
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
