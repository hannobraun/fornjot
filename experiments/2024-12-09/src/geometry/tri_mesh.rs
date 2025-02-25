use super::Triangle;

#[derive(Debug)]
pub struct TriMesh {
    pub triangles: Vec<MeshTriangle>,
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

#[derive(Debug)]
pub struct MeshTriangle {
    pub inner: Triangle<3>,
    #[allow(unused)] // code using this is being worked on
    pub is_internal: bool,
}
