use parry3d_f64::bounding_volume::AABB;

use crate::{
    geometry::{
        edges::Edges,
        faces::{triangulate, Faces},
        Shape,
    },
    math::Point,
};

impl Shape for fj::Sketch {
    fn bounding_volume(&self) -> AABB {
        AABB::from_points(&self.vertices())
    }

    fn faces(&self, _: f64) -> Faces {
        // TASK: This assumes that the sketch is convex. Remove this
        //       precondition, or at least add a check for it.
        Faces(triangulate(&self.vertices()))
    }

    fn edges(&self) -> Edges {
        // TASK: Implement.
        todo!()
    }

    fn vertices(&self) -> Vec<Point> {
        self.to_points()
            .into_iter()
            .map(|[x, y]| Point::new(x, y, 0.))
            .collect()
    }
}
