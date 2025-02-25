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

    pub fn all_triangles(&self) -> impl Iterator<Item = Triangle<3>> {
        self.triangles.iter().map(|triangle| triangle.inner)
    }
}

#[derive(Debug)]
pub struct MeshTriangle {
    pub inner: Triangle<3>,
    #[allow(unused)] // code using this is being worked on
    pub is_internal: bool,
}
