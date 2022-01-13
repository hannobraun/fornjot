use parry3d_f64::bounding_volume::AABB;

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::Surface,
        topology::{
            edges::{Edge, Edges},
            faces::{Face, Faces},
        },
        Shape,
    },
    math::Point,
};

impl Shape for fj::Sketch {
    fn bounding_volume(&self) -> AABB {
        AABB::from_points(&self.vertices())
    }

    fn faces(&self, _: f64, _: &mut DebugInfo) -> Faces {
        let edges = self.edges();
        let face = Face::Face {
            edges,
            surface: Surface::XYPlane,
        };
        Faces(vec![face])
    }

    fn edges(&self) -> Edges {
        let v = match self.vertices() {
            vertices if vertices.is_empty() => vertices,
            mut vertices => {
                // Add the first vertex at the end again, to close the loop.
                //
                // This can't panic. This `match` expression makes sure that
                // there are vertices.
                vertices.push(vertices[0]);
                vertices
            }
        };

        let mut edges = Vec::new();
        for window in v.windows(2) {
            // Can't panic, we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable.
            let a = window[0];
            let b = window[1];

            edges.push(Edge::line_segment(a, b));
        }

        Edges::single_cycle(edges)
    }

    fn vertices(&self) -> Vec<Point> {
        self.to_points()
            .into_iter()
            .map(|[x, y]| Point::from([x, y, 0.]))
            .collect()
    }
}
