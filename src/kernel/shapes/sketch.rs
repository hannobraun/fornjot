use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::{Curve, Line, Surface},
        topology::{
            edges::{Edge, Edges},
            faces::{Face, Faces},
            vertices::{Vertex, Vertices},
        },
    },
    math::{Aabb, Point, Scalar, Vector},
};

use super::Shape;

impl Shape for fj::Sketch {
    fn bounding_volume(&self) -> Aabb<3> {
        let vertices = self.vertices();
        Aabb::<3>::from_points(
            vertices.0.iter().map(|vertex| *vertex.location()),
        )
    }

    fn faces(&self, _: Scalar, _: &mut DebugInfo) -> Faces {
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
                direction: Vector::from(*b.location() - *a.location()),
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
            // These calls to `Vertex::create_at` don't follow the rules that
            // the documentation of `create_at` lays out. This method can be
            // called by an outside caller, and is additionally called by other
            // methods in this trait implementation, either directly or
            // indirectly.
            //
            // This means the same vertices are re-created multiple times, which
            // is forbidden. I don't think this is causing any actual problems,
            // since these `Vertex` instances are created from points that come
            // directly from the user, and aren't being computed here.
            //
            // But still, this rule exists for a reason: to prevent subtle bugs
            // from creeping in. We should follow it, here and everywhere.
            //
            // Please refer to this issue for more context on the problem, as
            // well as a proposed solution:
            // https://github.com/hannobraun/Fornjot/issues/176
            .map(|[x, y]| Vertex::create_at(Point::from([x, y, 0.])))
            .collect();
        Vertices(vertices)
    }
}
