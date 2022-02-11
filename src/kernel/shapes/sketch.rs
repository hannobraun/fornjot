use parry3d_f64::bounding_volume::AABB;

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::{Curve, Line, Surface},
        topology::{
            edges::{Edge, Edges},
            faces::{Face, Faces},
            vertices::{Vertex, Vertices},
        },
        Shape,
    },
    math::Point,
};

impl Shape for fj::Sketch {
    fn bounding_volume(&self) -> AABB {
        let vertices = self.vertices();
        AABB::from_points(vertices.0.iter().map(|vertex| vertex.location()))
    }

    fn faces(&self, _: f64, _: &mut DebugInfo) -> Faces {
        let edges = self.edges();
        let face = Face::Face {
            edges,
            surface: Surface::x_y_plane(),
        };
        Faces(vec![face])
    }

    fn edges(&self) -> Edges {
        let vertices = match self.vertices() {
            vertices if vertices.0.is_empty() => vertices.0,
            vertices => {
                let mut vertices = vertices.0;

                // Add the first vertex at the end again, to close the loop.
                //
                // This can't panic. This `match` expression makes sure that
                // there are vertices.
                vertices.push(vertices[0]);
                vertices
            }
        };

        let mut edges = Vec::new();
        for window in vertices.windows(2) {
            // Can't panic, we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable.
            let a = window[0];
            let b = window[1];

            let line = Curve::Line(Line {
                origin: *a.location(),
                direction: *b.location() - *a.location(),
            });
            let edge = Edge::new(line, Some([a, b]));

            edges.push(edge);
        }

        Edges::single_cycle(edges)
    }

    fn vertices(&self) -> Vertices {
        let vertices = self
            .to_points()
            .into_iter()
            // These calls to `create_at` are valid, since the points of this
            // sketch come directly from the user and define new vertices of the
            // shape.
            .map(|[x, y]| Vertex::create_at(Point::from([x, y, 0.])))
            .collect();
        Vertices(vertices)
    }
}
