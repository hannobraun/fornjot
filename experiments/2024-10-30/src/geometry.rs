#[derive(Default)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn vertices(&self) -> impl Iterator<Item = Vertex> + '_ {
        self.vertices.iter().copied()
    }

    pub fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        self.triangles.iter().copied()
    }

    pub fn push_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn push_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub point: [f64; 3],
}

pub type Index = u32;
pub type Triangle = [Index; 3];
