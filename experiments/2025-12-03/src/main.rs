use fj_interop::{Color, MeshTriangle, TriMesh};
use fj_math::Triangle;

fn main() -> anyhow::Result<()> {
    let tri_mesh = fj_viewer::make_viewer_and_spawn_thread(|viewer| {
        let tri_mesh = model();
        viewer.open_window().display_mesh(tri_mesh.clone());
        tri_mesh
    })?;

    fj_export::export(tri_mesh.external_triangles(), "output.3mf")?;

    Ok(())
}

fn model() -> TriMesh {
    let mut vertices = Vertices { inner: Vec::new() };
    let mut triangles = Vec::new();

    vertices.push([0., 0., 0.]); // v0
    vertices.push([0., 0., 1.]); // v1
    vertices.push([0., 1., 0.]); // v2
    vertices.push([0., 1., 1.]); // v3
    vertices.push([1., 0., 0.]); // v4
    vertices.push([1., 0., 1.]); // v5
    vertices.push([1., 1., 0.]); // v6
    vertices.push([1., 1., 1.]); // v7

    // front
    triangles.push([0, 4, 5]); // t0
    triangles.push([0, 5, 1]); // t1
    // right
    triangles.push([4, 6, 7]); // t2
    triangles.push([4, 7, 5]); // t3
    // back
    triangles.push([6, 2, 3]); // t4
    triangles.push([6, 3, 7]); // t5
    // left
    triangles.push([2, 0, 1]); // t6
    triangles.push([2, 1, 3]); // t7
    // bottom
    triangles.push([2, 6, 4]); // t8
    triangles.push([2, 4, 0]); // t9
    // top
    triangles.push([1, 5, 7]); // t10
    triangles.push([1, 7, 3]); // t11

    let mut tri_mesh = TriMesh::new();

    for [a, b, c] in triangles {
        tri_mesh.triangles.push(MeshTriangle {
            inner: Triangle::from_points([
                vertices.inner[a],
                vertices.inner[b],
                vertices.inner[c],
            ]),
            is_internal: false,
            color: Color::default(),
        });
    }

    tri_mesh
}

pub struct Vertices {
    inner: Vec<[f64; 3]>,
}

impl Vertices {
    pub fn push(&mut self, vertex: [f64; 3]) -> usize {
        let index = self.inner.len();
        self.inner.push(vertex);
        index
    }
}
