use super::Triangle;

pub struct TriMesh {
    pub triangles: Vec<Triangle>,
}

impl TriMesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }
}
