use crate::{
    geometry::{AnyOp, Handle, Operation, TriMesh},
    math::Plane,
    storage::Store,
};

use super::face::Face;

pub struct Solid {
    faces: Vec<Handle<Face>>,
}

impl Solid {
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }

    /// Connect two faces by creating a side wall of faces from their vertices
    ///
    /// ## Panics
    ///
    /// Panics, if the two faces provided do not have the same number of
    /// vertices.
    ///
    /// ## Implementation Note
    ///
    /// This method has very particular (and undocumented) requirements about
    /// the orientation of the two faces relative to each other, and will
    /// happily generate invalid geometry, if those undocumented requirements
    /// aren't met.
    ///
    /// It should be seen as more of a placeholder for a real implementation of
    /// this operation.
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
    type Output = Self;

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
