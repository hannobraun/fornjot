use bytemuck::{Pod, Zeroable};
use fj_interop::{
    debug::DebugInfo,
    mesh::{Index, Mesh},
};
use fj_math::{Point, Vector};

#[derive(Debug)]
pub struct Vertices {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Vertices {
    pub fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[Index] {
        self.indices.as_slice()
    }

    pub fn push_line(
        &mut self,
        line: [Point<3>; 2],
        normal: [f32; 3],
        color: [f32; 4],
    ) {
        let line = line.into_iter().map(|point| Vertex {
            position: point.coords.components.map(|scalar| scalar.into_f32()),
            normal,
            color,
        });

        self.vertices.extend(line);

        self.indices.push(self.indices.len() as u32);
        self.indices.push(self.indices.len() as u32);
    }

    pub fn push_cross(
        &mut self,
        position: Point<3>,
        normal: [f32; 3],
        color: [f32; 4],
    ) {
        let d = 0.05;

        self.push_line(
            [
                position - Vector::from([d, 0., 0.]),
                position + Vector::from([d, 0., 0.]),
            ],
            normal,
            color,
        );
        self.push_line(
            [
                position - Vector::from([0., d, 0.]),
                position + Vector::from([0., d, 0.]),
            ],
            normal,
            color,
        );
    }
}

impl From<&Mesh<fj_math::Point<3>>> for Vertices {
    fn from(mesh: &Mesh<fj_math::Point<3>>) -> Self {
        let mut m = Mesh::new();

        for triangle in mesh.triangles() {
            let [a, b, c] = triangle.inner.points();

            let normal = (b - a).cross(&(c - a)).normalize();
            let color = triangle.color;

            m.push_vertex((a, normal, color));
            m.push_vertex((b, normal, color));
            m.push_vertex((c, normal, color));
        }

        let vertices = m
            .vertices()
            .map(|(vertex, normal, color)| Vertex {
                position: vertex.into(),
                normal: normal.into(),
                color: color.0.map(|v| f32::from(v) / 255.0),
            })
            .collect();

        let indices = m.indices().collect();

        Self { vertices, indices }
    }
}

impl From<&DebugInfo> for Vertices {
    fn from(debug_info: &DebugInfo) -> Self {
        let mut self_ = Self::empty();

        for triangle_edge_check in &debug_info.triangle_edge_checks {
            let normal = [0.; 3];

            let red = [1., 0., 0., 1.];
            let green = [0., 1., 0., 1.];

            let color = if triangle_edge_check.hits.len() % 2 == 0 {
                red
            } else {
                green
            };

            self_.push_cross(triangle_edge_check.origin, normal, color);

            for &hit in &triangle_edge_check.hits {
                let line = hit.points();
                let color = [0., 0., 0., 1.];

                self_.push_line(line, normal, color);
            }
        }

        self_
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}
