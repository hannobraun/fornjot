/// Placeholder for what will end up being the representation of a CAD model
#[derive(Debug)]
#[repr(C)]
pub enum Shape {
    Cube(Cube),
}

/// A cube
#[derive(Debug)]
#[repr(C)]
pub struct Cube {
    /// The side length of the cube
    pub size: f32,
}
