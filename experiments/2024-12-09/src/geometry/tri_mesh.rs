use super::Triangle;

#[derive(Debug)]
pub struct TriMesh {
    pub triangles: Vec<Triangle<3>>,
}

impl TriMesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.triangles.extend(other.triangles);
        self
    }
}
