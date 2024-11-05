#[derive(Default)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<[u32; 3]>,
}

impl Mesh {
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn triangles(&self) -> &[[u32; 3]] {
        &self.triangles
    }

    pub fn push_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn push_triangle(&mut self, triangle: [u32; 3]) {
        self.triangles.push(triangle);
    }
}

pub type Vertex = [f32; 3];
