use fj_interop::DebugInfo;
use fj_kernel::{
    geometry::Surface,
    shape::Shape,
    topology::{Cycle, Edge, Face, Vertex},
};
use fj_math::{Aabb, Point, Scalar};

use super::ToShape;

impl ToShape for fj::Sketch {
    fn to_shape(&self, _: Scalar, _: &mut DebugInfo) -> Shape {
        let mut shape = Shape::new();
        let mut vertices = Vec::new();

        for [x, y] in self.to_points() {
            let point = shape.insert(Point::from([x, y, 0.])).unwrap();
            let vertex = shape.insert(Vertex { point }).unwrap();
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

                let edge = Edge::builder(&mut shape)
                    .build_line_segment_from_vertices([a, b])
                    .unwrap();
                edges.push(edge);
            }

            shape.insert(Cycle { edges }).unwrap();
        };

        let surface = shape.insert(Surface::x_y_plane()).unwrap();
        let face = Face::Face {
            exteriors: shape.topology().cycles().collect(),
            interiors: Vec::new(),
            surface,
            color: self.color(),
        };
        shape.insert(face).unwrap();

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
