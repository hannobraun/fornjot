use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::{Curve, Line, Surface},
        topology::{
            edges::{Edge, Edges},
            faces::{Face, Faces},
            vertices::{Vertex, Vertices},
            Shape,
        },
    },
    math::{Aabb, Point, Scalar, Vector},
};

use super::ToShape;

impl ToShape for fj::Sketch {
    fn to_shape(&self, _: Scalar, _: &mut DebugInfo) -> Shape {
        let vertices = self
            .to_points()
            .into_iter()
            .map(|[x, y]| Vertex::create_at(Point::from([x, y, 0.])))
            .collect();
        let vertices = Vertices(vertices);

        let edges = {
            let vertices = match vertices.clone() {
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
        };

        let face = Face::Face {
            edges: edges.clone(),
            surface: Surface::x_y_plane(),
        };
        let faces = Faces(vec![face]);

        Shape {
            vertices,
            edges,
            faces,
        }
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
