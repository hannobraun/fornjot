use crate::{
    geometry::{
        bounding_volume::{Aabb, BoundingVolume},
        edges::Edges,
        faces::{Faces, Triangle},
        vertices::Vertices,
    },
    math::Point,
};

impl BoundingVolume for fj::Square {
    fn aabb(&self) -> Aabb {
        Aabb::from_vertices(self.vertices())
    }
}

impl Edges for fj::Square {
    fn edge_vertices(&self, _: f64) -> Vec<Vec<Point>> {
        vec![self.vertices()]
    }
}

impl Faces for fj::Square {
    fn triangles(&self, _: f64) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        let v = self.vertices();

        triangles.push([v[0], v[1], v[2]].into());
        triangles.push([v[0], v[2], v[3]].into());

        triangles
    }
}

impl Vertices for fj::Square {
    fn vertices(&self) -> Vec<Point> {
        let s = self.size / 2.;

        #[rustfmt::skip]
        let v = [
            [-s, -s, 0.0],
            [ s, -s, 0.0],
            [ s,  s, 0.0],
            [-s,  s, 0.0],
        ];

        v.map(|coord| coord.into()).to_vec()
    }
}
