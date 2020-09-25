use crate::graphics::Mesh;

pub struct Triangle {
    pub a: [f32; 3],
    pub b: [f32; 3],
    pub c: [f32; 3],
}

impl Triangle {
    pub fn to_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new();

        let i0 = mesh.vertex(self.a);
        let i1 = mesh.vertex(self.b);
        let i2 = mesh.vertex(self.c);

        mesh.triangle(i0, i1, i2);

        mesh
    }
}

#[cfg(test)]
mod tests {
    use super::Triangle;

    #[test]
    fn test() {
        let triangle = Triangle {
            a: [0.0, 0.0, 0.0],
            b: [1.0, 0.0, 0.0],
            c: [0.0, 1.0, 0.0],
        };

        let mesh = triangle.to_mesh();
        let triangles: Vec<_> = mesh.triangles().collect();

        assert_eq!(triangles, vec![[triangle.a, triangle.b, triangle.c]]);
    }
}
