use crate::{
    geometry::{
        bounding_volume::Aabb,
        edges::{Edge, Edges},
        faces::Triangle,
        Shape,
    },
    math::Point,
};

impl Shape for fj::Square {
    fn bounding_volume(&self) -> Aabb {
        Aabb::from_vertices(self.vertices())
    }

    fn edges(&self) -> Edges {
        let v = self.vertices();

        vec![
            Edge::line_segment(v[0], v[1]),
            Edge::line_segment(v[1], v[2]),
            Edge::line_segment(v[2], v[3]),
            Edge::line_segment(v[3], v[0]),
        ]
    }

    fn triangles(&self, _: f64) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        let v = self.vertices();

        triangles.push([v[0], v[1], v[2]].into());
        triangles.push([v[0], v[2], v[3]].into());

        triangles
    }

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
