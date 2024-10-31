#[derive(Default)]
pub struct Mesh {
    vertices: Vec<[f32; 3]>,
    pub triangles: Vec<[u32; 3]>,
}

impl Mesh {
    pub fn vertices(&self) -> &[[f32; 3]] {
        &self.vertices
    }

    pub fn triangles(&self) -> &[[u32; 3]] {
        &self.triangles
    }

    pub fn push_vertex(&mut self, vertex: [f32; 3]) {
        self.vertices.push(vertex);
    }

    pub fn push_triangle(&mut self, triangle: [u32; 3]) {
        self.triangles.push(triangle);
    }
}
