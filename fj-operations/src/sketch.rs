use fj_debug::DebugInfo;
use fj_kernel::{
    geometry::Surface,
    shape::Shape,
    topology::{Cycle, Face, Vertex},
};
use fj_math::{Aabb, Point, Scalar};

use super::ToShape;

impl ToShape for fj::Sketch {
    fn to_shape(&self, _: Scalar, _: &mut DebugInfo) -> Shape {
        let mut shape = Shape::new();
        let mut vertices = Vec::new();

        for [x, y] in self.to_points() {
            let point = shape.geometry().add_point(Point::from([x, y, 0.]));
            let vertex = shape.topology().add_vertex(Vertex { point }).unwrap();
            vertices.push(vertex);
        }

        {
            if !vertices.is_empty() {
                // Add the first vertex at the end again, to close the loop.
                //
                // This can't panic. We just checked that `vertices` is not
                // empty.
                vertices.push(vertices[0].clone());
            }

            let mut edges = Vec::new();
            for window in vertices.windows(2) {
                // Can't panic, we passed `2` to `windows`.
                //
                // Can be cleaned up, once `array_windows` is stable.
                let a = window[0].clone();
                let b = window[1].clone();

                let edge = shape.topology().add_line_segment([a, b]).unwrap();
                edges.push(edge);
            }

            shape.topology().add_cycle(Cycle { edges }).unwrap();
        };

        let surface = shape.geometry().add_surface(Surface::x_y_plane());
        let face = Face::Face {
            exteriors: shape.topology().cycles().collect(),
            interiors: Vec::new(),
            surface,
            color: self.color(),
        };
        shape.topology().add_face(face).unwrap();

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        Aabb::<3>::from_points(
            self.to_points()
                .into_iter()
                .map(Point::from)
                .map(Point::to_xyz),
        )
    }
}
