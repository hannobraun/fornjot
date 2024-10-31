#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub triangles: Vec<[u32; 3]>,
}

impl Mesh {
    pub fn push_vertex(&mut self, vertex: [f32; 3]) {
        self.vertices.push(vertex);
    }
}
