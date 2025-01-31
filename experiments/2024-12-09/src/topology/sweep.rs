use crate::{
    geometry::Handle,
    math::{Plane, Vector},
    storage::Store,
};

use super::{Face, Solid, Vertex};

pub trait SweepExt {
    fn sweep(
        self,
        path: impl Into<Vector<3>>,
        faces: &mut Store<Face>,
        surfaces: &mut Store<Plane>,
        vertices: &mut Store<Vertex>,
    ) -> Solid;
}

impl SweepExt for Handle<Face> {
    fn sweep(
        self,
        path: impl Into<Vector<3>>,
        faces: &mut Store<Face>,
        surfaces: &mut Store<Plane>,
        vertices: &mut Store<Vertex>,
    ) -> Solid {
        let target = faces
            .insert(self.flip(surfaces).translate(path, surfaces, vertices));

        Solid::connect_faces([target, self], faces, surfaces)
    }
}
