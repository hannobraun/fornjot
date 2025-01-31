use crate::{
    geometry::{AnyOp, Handle, Operation, TriMesh},
    math::{Plane, Vector},
    storage::Store,
};

use super::{Face, Vertex};

pub struct Solid {
    faces: Vec<Handle<Face>>,
}

impl Solid {
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }

    pub fn sweep_from(
        origin: Handle<Face>,
        path: impl Into<Vector<3>>,
        faces: &mut Store<Face>,
        surfaces: &mut Store<Plane>,
        vertices: &mut Store<Vertex>,
    ) -> Self {
        let target = faces
            .insert(origin.flip(surfaces).translate(path, surfaces, vertices));

        Solid::connect_faces([target, origin], faces, surfaces)
    }

    pub fn connect_faces(
        [a, b]: [Handle<Face>; 2],
        faces: &mut Store<Face>,
        surfaces: &mut Store<Plane>,
    ) -> Self {
        assert_eq!(
            a.vertices().count(),
            b.vertices().count(),
            "Can only connect faces that have the same number of vertices.",
        );

        let side_faces = a
            .half_edges()
            .zip(b.half_edges())
            .map(|([q, r], [t, s])| {
                let surface = surfaces.insert(Plane::from_points(
                    [q, r, s].map(|vertex| vertex.point),
                ));
                let face = Face::new(
                    surface,
                    [q, r, s, t].map(|vertex| vertex.clone()),
                );
                faces.insert(face)
            })
            .collect::<Vec<_>>();

        Solid::new([a, b].into_iter().chain(side_faces))
    }
}

impl Operation for Solid {
    fn label(&self) -> &'static str {
        "Solid"
    }

    fn tri_mesh(&self) -> TriMesh {
        let mut tri_mesh = TriMesh::new();

        for face in &self.faces {
            tri_mesh = tri_mesh.merge(face.tri_mesh());
        }

        tri_mesh
    }

    fn children(&self) -> Vec<AnyOp> {
        self.faces.iter().map(|face| face.to_any()).collect()
    }
}
