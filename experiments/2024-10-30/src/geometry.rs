#[derive(Default)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<[Index; 3]>,
}

impl Mesh {
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn triangles(&self) -> &[[Index; 3]] {
        &self.triangles
    }

    pub fn push_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn push_triangle(&mut self, triangle: [Index; 3]) {
        self.triangles.push(triangle);
    }
}

pub type Vertex = [f32; 3];
pub type Index = u32;
