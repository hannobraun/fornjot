use crate::{
    kernel::{
        shape::Shape,
        topology::{
            edges::{Cycle, Edge},
            faces::Face,
            vertices::Vertex,
        },
    },
    math::Transform,
};

/// Create a new shape that is a transformed version of an existing one
///
/// # Implementation note
///
/// This code isn't really correct, only transforming the faces of the original
/// shape and not taking care of anything else, but this is more a reflection of
/// the state of `Shape`, with its redundant data.
///
/// Addressing the shortcomings in this method probably doesn't make sense,
/// except as a side effect of addressing the shortcomings of `Shape`.
pub fn transform_shape(mut original: Shape, transform: &Transform) -> Shape {
    let mut transformed = Shape::new();

    for face in original.topology().faces() {
        let face = match face.get().clone() {
            Face::Face {
                cycles,
                surface,
                color,
            } => {
                let mut cycles_trans = Vec::new();

                for cycle in cycles {
                    let mut edges = Vec::new();

                    for edge in &cycle.edges {
                        let curve = transformed
                            .geometry()
                            .add_curve(edge.curve().transform(transform));

                        let vertices =
                            edge.vertices().clone().map(|vertices| {
                                vertices.map(|vertex| {
                                    let point =
                                        transformed.geometry().add_point(
                                            transform.transform_point(
                                                &vertex.point(),
                                            ),
                                        );

                                    transformed
                                        .topology()
                                        .add_vertex(Vertex { point })
                                        .unwrap()
                                })
                            });

                        let edge = Edge { curve, vertices };
                        let edge =
                            transformed.topology().add_edge(edge).unwrap();

                        edges.push(edge);
                    }

                    cycles_trans.push(
                        transformed
                            .topology()
                            .add_cycle(Cycle { edges })
                            .unwrap(),
                    );
                }

                let surface = transformed
                    .geometry()
                    .add_surface(surface.transform(transform));

                Face::Face {
                    cycles: cycles_trans,
                    surface,
                    color,
                }
            }
            Face::Triangles(mut triangles) => {
                for triangle in &mut triangles {
                    *triangle = transform.transform_triangle(triangle);
                }

                Face::Triangles(triangles)
            }
        };

        transformed.topology().add_face(face).unwrap();
    }

    transformed
}
