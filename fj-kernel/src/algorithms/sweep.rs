use std::collections::HashMap;

use fj_math::{Scalar, Segment, Transform, Triangle, Vector};

use crate::{
    geometry::{Surface, SweptCurve},
    shape::{Handle, Shape},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::approximation::approximate_cycle;

/// Create a new shape by sweeping an existing one
pub fn sweep_shape(
    mut source: Shape,
    path: Vector<3>,
    tolerance: Scalar,
    color: [u8; 4],
) -> Shape {
    let mut target = Shape::new();

    let translation = Transform::translation(path);

    let mut source_to_bottom = Relation::new();
    let mut source_to_top = Relation::new();

    // Create the new vertices.
    for vertex_source in source.topology().vertices() {
        let point_bottom = target.insert(vertex_source.get().point()).unwrap();
        let point_top = target.insert(point_bottom.get() + path).unwrap();

        let vertex_bottom = target
            .insert(Vertex {
                point: point_bottom,
            })
            .unwrap();
        let vertex_top = target.insert(Vertex { point: point_top }).unwrap();

        source_to_bottom
            .vertices
            .insert(vertex_source.clone(), vertex_bottom);
        source_to_top.vertices.insert(vertex_source, vertex_top);
    }

    // Create the new edges.
    for edge_source in source.topology().edges() {
        let curve_bottom = target.insert(edge_source.get().curve()).unwrap();
        let curve_top = target
            .insert(curve_bottom.get().transform(&translation))
            .unwrap();

        let vertices_bottom = source_to_bottom.vertices_for_edge(&edge_source);
        let vertices_top = source_to_top.vertices_for_edge(&edge_source);

        let edge_bottom = target
            .insert(Edge {
                curve: curve_bottom,
                vertices: vertices_bottom,
            })
            .unwrap();
        let edge_top = target
            .insert(Edge {
                curve: curve_top,
                vertices: vertices_top,
            })
            .unwrap();

        source_to_bottom
            .edges
            .insert(edge_source.clone(), edge_bottom);
        source_to_top.edges.insert(edge_source, edge_top);
    }

    // Create the new cycles.
    for cycle_source in source.topology().cycles() {
        let edges_bottom = source_to_bottom.edges_for_cycle(&cycle_source);
        let edges_top = source_to_top.edges_for_cycle(&cycle_source);

        let cycle_bottom = target
            .insert(Cycle {
                edges: edges_bottom,
            })
            .unwrap();
        let cycle_top = target.insert(Cycle { edges: edges_top }).unwrap();

        source_to_bottom
            .cycles
            .insert(cycle_source.clone(), cycle_bottom);
        source_to_top.cycles.insert(cycle_source, cycle_top);
    }

    // Create top faces.
    for face_source in source.topology().faces().values() {
        let surface_bottom = target.insert(face_source.surface()).unwrap();
        let surface_top = target
            .insert(surface_bottom.get().transform(&translation))
            .unwrap();

        let exteriors_bottom =
            source_to_bottom.exteriors_for_face(&face_source);
        let interiors_bottom =
            source_to_bottom.interiors_for_face(&face_source);
        let exteriors_top = source_to_top.exteriors_for_face(&face_source);
        let interiors_top = source_to_top.interiors_for_face(&face_source);

        target
            .topology()
            .add_face(Face::Face {
                surface: surface_bottom,
                exteriors: exteriors_bottom,
                interiors: interiors_bottom,
                color,
            })
            .unwrap();
        target
            .topology()
            .add_face(Face::Face {
                surface: surface_top,
                exteriors: exteriors_top,
                interiors: interiors_top,
                color,
            })
            .unwrap();
    }

    for cycle_source in source.topology().cycles() {
        if cycle_source.get().edges.len() == 1 {
            // If there's only one edge in the cycle, it must be a continuous
            // edge that connects to itself. By sweeping that, we create a
            // continuous face.
            //
            // Continuous faces aren't currently supported by the approximation
            // code, and hence can't be triangulated. To address that, we fall
            // back to the old and almost obsolete triangle representation to
            // create the face.
            //
            // This is the last piece of code that still uses the triangle
            // representation.

            let approx = approximate_cycle(&cycle_source.get(), tolerance);

            let mut quads = Vec::new();
            for segment in approx.windows(2) {
                let segment = Segment::from_points([segment[0], segment[1]]);

                let [v0, v1] = segment.points();
                let [v3, v2] = {
                    let segment = Transform::translation(path)
                        .transform_segment(&segment);
                    segment.points()
                };

                quads.push([v0, v1, v2, v3]);
            }

            let mut side_face: Vec<Triangle<3>> = Vec::new();
            for [v0, v1, v2, v3] in quads {
                side_face.push([v0, v1, v2].into());
                side_face.push([v0, v2, v3].into());
            }

            // FIXME: We probably want to allow the use of custom colors for the
            // "walls" of the swept object.
            for s in side_face.iter_mut() {
                s.set_color(color);
            }

            target
                .topology()
                .add_face(Face::Triangles(side_face))
                .unwrap();
        } else {
            // If there's no continuous edge, we can create the non-
            // continuous faces using boundary representation.

            let mut vertex_bottom_to_edge = HashMap::new();

            for edge_source in &cycle_source.get().edges {
                // Can't panic. We already ruled out the continuous edge case
                // above, so this edge must have vertices.
                let vertices_source =
                    edge_source.get().vertices.clone().unwrap();

                // Create (or retrieve from the cache, `vertex_bottom_to_edge`)
                // side edges from the vertices of this source/bottom edge.
                let [side_edge_a, side_edge_b] =
                    vertices_source.map(|vertex_source| {
                        let vertex_bottom = source_to_bottom
                            .vertices
                            .get(&vertex_source)
                            .unwrap()
                            .clone();

                        vertex_bottom_to_edge
                            .entry(vertex_bottom.clone())
                            .or_insert_with(|| {
                                let curve = target
                                    .insert(edge_source.get().curve())
                                    .unwrap();

                                let vertex_top = source_to_top
                                    .vertices
                                    .get(&vertex_source)
                                    .unwrap()
                                    .clone();

                                target
                                    .insert(Edge {
                                        curve,
                                        vertices: Some([
                                            vertex_bottom,
                                            vertex_top,
                                        ]),
                                    })
                                    .unwrap()
                            })
                            .clone()
                    });

                // Now we have everything we need to create the side face from
                // this source/bottom edge.

                let bottom_edge =
                    source_to_bottom.edges.get(edge_source).unwrap().clone();
                let top_edge =
                    source_to_top.edges.get(edge_source).unwrap().clone();

                let surface = target
                    .insert(Surface::SweptCurve(SweptCurve {
                        curve: bottom_edge.get().curve(),
                        path,
                    }))
                    .unwrap();

                let cycle = target
                    .insert(Cycle {
                        edges: vec![
                            bottom_edge,
                            top_edge,
                            side_edge_a,
                            side_edge_b,
                        ],
                    })
                    .unwrap();

                target
                    .topology()
                    .add_face(Face::Face {
                        surface,
                        exteriors: vec![cycle],
                        interiors: Vec::new(),
                        color,
                    })
                    .unwrap();
            }
        }
    }

    target
}

struct Relation {
    vertices: HashMap<Handle<Vertex>, Handle<Vertex>>,
    edges: HashMap<Handle<Edge>, Handle<Edge>>,
    cycles: HashMap<Handle<Cycle>, Handle<Cycle>>,
}

impl Relation {
    fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            cycles: HashMap::new(),
        }
    }

    fn vertices_for_edge(
        &self,
        edge: &Handle<Edge>,
    ) -> Option<[Handle<Vertex>; 2]> {
        edge.get().vertices.map(|vertices| {
            vertices.map(|vertex| self.vertices.get(&vertex).unwrap().clone())
        })
    }

    fn edges_for_cycle(&self, cycle: &Handle<Cycle>) -> Vec<Handle<Edge>> {
        cycle
            .get()
            .edges
            .iter()
            .map(|edge| self.edges.get(edge).unwrap().clone())
            .collect()
    }

    fn exteriors_for_face(&self, face: &Face) -> Vec<Handle<Cycle>> {
        let exteriors = match face {
            Face::Face { exteriors, .. } => exteriors,
            _ => {
                // Sketches are created using boundary representation, so this
                // case can't happen.
                unreachable!()
            }
        };

        exteriors
            .iter()
            .map(|cycle| self.cycles.get(cycle).unwrap().clone())
            .collect()
    }

    fn interiors_for_face(&self, face: &Face) -> Vec<Handle<Cycle>> {
        let interiors = match face {
            Face::Face { interiors, .. } => interiors,
            _ => {
                // Sketches are created using boundary representation, so this
                // case can't happen.
                unreachable!()
            }
        };

        interiors
            .iter()
            .map(|cycle| self.cycles.get(cycle).unwrap().clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Point, Scalar, Vector};

    use crate::{
        geometry::{Surface, SweptCurve},
        shape::{Handle, Shape},
        topology::{Cycle, Edge, Face, Vertex},
    };

    use super::sweep_shape;

    #[test]
    fn sweep() -> anyhow::Result<()> {
        let sketch = Triangle::new([[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]])?;

        let mut swept = sweep_shape(
            sketch.shape,
            Vector::from([0., 0., 1.]),
            Scalar::from_f64(0.),
            [255, 0, 0, 255],
        );

        let bottom_face = sketch.face.get();
        let top_face =
            Triangle::new([[0., 0., 1.], [1., 0., 1.], [0., 1., 1.]])?
                .face
                .get();

        let mut contains_bottom_face = false;
        let mut contains_top_face = false;

        for face in swept.topology().faces() {
            if matches!(face.get(), Face::Face { .. }) {
                if face.get().clone() == bottom_face {
                    contains_bottom_face = true;
                }
                if face.get().clone() == top_face {
                    contains_top_face = true;
                }
            }
        }

        assert!(contains_bottom_face);
        assert!(contains_top_face);

        // Side faces are not tested, as those use triangle representation. The
        // plan is to start testing them, as they are transitioned to b-rep.

        Ok(())
    }

    pub struct Triangle {
        shape: Shape,
        face: Handle<Face>,
    }

    impl Triangle {
        fn new([a, b, c]: [impl Into<Point<3>>; 3]) -> anyhow::Result<Self> {
            let mut shape = Shape::new();

            let a = shape.insert(a.into())?;
            let b = shape.insert(b.into())?;
            let c = shape.insert(c.into())?;

            let a = shape.insert(Vertex { point: a })?;
            let b = shape.insert(Vertex { point: b })?;
            let c = shape.insert(Vertex { point: c })?;

            let ab = Edge::build(&mut shape)
                .line_segment_from_vertices([a.clone(), b.clone()])?;
            let bc = Edge::build(&mut shape)
                .line_segment_from_vertices([b.clone(), c.clone()])?;
            let ca = Edge::build(&mut shape)
                .line_segment_from_vertices([c.clone(), a.clone()])?;

            let cycles = shape.insert(Cycle {
                edges: vec![ab, bc, ca],
            })?;

            let surface = shape.insert(Surface::SweptCurve(
                SweptCurve::plane_from_points(
                    [a, b, c].map(|vertex| vertex.get().point()),
                ),
            ))?;
            let abc = Face::Face {
                surface,
                exteriors: vec![cycles],
                interiors: Vec::new(),
                color: [255, 0, 0, 255],
            };

            let face = shape.topology().add_face(abc)?;

            Ok(Self { shape, face })
        }
    }
}
