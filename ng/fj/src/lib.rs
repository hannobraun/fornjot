/// Placeholder for what will end up being the representation of a CAD model
#[derive(Debug)]
#[repr(C)]
pub enum Shape {
    Cube { size: f32 },
}
