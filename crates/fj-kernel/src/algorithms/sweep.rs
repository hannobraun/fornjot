use std::collections::HashMap;

use fj_math::{Scalar, Transform, Triangle, Vector};

use crate::{
    geometry::{Surface, SweptCurve},
    shape::{Handle, Mapping, Shape, ValidationError, ValidationResult},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{transform_shape, CycleApprox, Tolerance};

/// Create a new shape by sweeping an existing one
pub fn sweep_shape(
    source: Shape,
    path: Vector<3>,
    tolerance: Tolerance,
    color: [u8; 4],
) -> Result<Shape, ValidationError> {
    let mut sweep = Sweep::init(source, path, tolerance, color);
    sweep.create_top_and_bottom_faces()?;
    sweep.create_side_faces()?;

    Ok(sweep.target)
}

struct Sweep {
    source: Shape,
    target: Shape,

    bottom: Shape,
    top: Shape,

    source_to_bottom: Mapping,
    source_to_top: Mapping,

    path: Vector<3>,
    translation: Transform,
    is_sweep_along_negative_direction: bool,

    tolerance: Tolerance,
    color: [u8; 4],
}

impl Sweep {
    fn init(
        source: Shape,
        path: Vector<3>,
        tolerance: Tolerance,
        color: [u8; 4],
    ) -> Self {
        let target = Shape::new();

        let (bottom, source_to_bottom) = source.clone_shape();
        let (top, source_to_top) = source.clone_shape();

        let translation = Transform::translation(path);
        let is_sweep_along_negative_direction =
            path.dot(&Vector::from([0., 0., 1.])) < Scalar::ZERO;

        Self {
            source,
            target,

            bottom,
            top,

            source_to_bottom,
            source_to_top,

            path,
            translation,
            is_sweep_along_negative_direction,

            tolerance,
            color,
        }
    }

    fn create_top_and_bottom_faces(&mut self) -> Result<(), ValidationError> {
        if self.is_sweep_along_negative_direction {
            reverse_surfaces(&mut self.top)?;
        } else {
            reverse_surfaces(&mut self.bottom)?;
        }
        transform_shape(&mut self.top, &self.translation)?;

        self.target.merge_shape(&self.bottom)?;
        self.target.merge_shape(&self.top)?;

        Ok(())
    }

    fn create_side_faces(&mut self) -> Result<(), ValidationError> {
        for face_source in self.source.faces() {
            for cycle_source in face_source.get().all_cycles() {
                if cycle_source.edges.len() == 1 {
                    // If there's only one edge in the cycle, it must be a
                    // continuous edge that connects to itself. By sweeping
                    // that, we create a continuous face.
                    //
                    // Continuous faces aren't currently supported by the
                    // approximation code, and hence can't be triangulated. To
                    // address that, we fall back to the old and almost obsolete
                    // triangle representation to create the face.
                    //
                    // This is the last piece of code that still uses the
                    // triangle representation.
                    create_continuous_side_face_fallback(
                        &cycle_source,
                        &self.translation,
                        self.tolerance,
                        self.color,
                        &mut self.target,
                    )?;

                    continue;
                }

                // If there's no continuous edge, we can create the non-
                // continuous faces using boundary representation.

                let mut vertex_bottom_to_edge = HashMap::new();

                for edge_source in &cycle_source.edges {
                    let edge_bottom =
                        self.source_to_bottom.edge(&edge_source.canonical());
                    let edge_top =
                        self.source_to_top.edge(&edge_source.canonical());

                    let surface = create_side_surface(self, &edge_bottom);

                    let cycle = create_side_cycle(
                        &mut self.target,
                        edge_bottom,
                        edge_top,
                        &mut vertex_bottom_to_edge,
                    )?;

                    create_side_face(self, surface, cycle)?;
                }
            }
        }

        Ok(())
    }
}

fn reverse_surfaces(shape: &mut Shape) -> Result<(), ValidationError> {
    shape
        .update()
        .update_all(|surface: &mut Surface| *surface = surface.reverse())
        .validate()
}

fn create_continuous_side_face_fallback(
    cycle_source: &Cycle<3>,
    translation: &Transform,
    tolerance: Tolerance,
    color: [u8; 4],
    target: &mut Shape,
) -> Result<(), ValidationError> {
    let approx = CycleApprox::new(cycle_source, tolerance);

    let mut quads = Vec::new();
    for segment in approx.segments() {
        let [v0, v1] = segment.points();
        let [v3, v2] = {
            let segment = translation.transform_segment(&segment);
            segment.points()
        };

        quads.push([v0, v1, v2, v3]);
    }

    let mut side_face: Vec<(Triangle<3>, _)> = Vec::new();
    for [v0, v1, v2, v3] in quads {
        side_face.push(([v0, v1, v2].into(), color));
        side_face.push(([v0, v2, v3].into(), color));
    }

    target.insert(Face::Triangles(side_face))?;

    Ok(())
}

fn create_side_surface(
    sweep: &Sweep,
    edge_bottom: &Handle<Edge<3>>,
) -> Surface {
    let mut surface = Surface::SweptCurve(SweptCurve {
        curve: edge_bottom.get().curve(),
        path: sweep.path,
    });

    if sweep.is_sweep_along_negative_direction {
        surface = surface.reverse();
    }

    surface
}

fn create_side_cycle(
    target: &mut Shape,
    edge_bottom: Handle<Edge<3>>,
    edge_top: Handle<Edge<3>>,
    vertex_bottom_to_edge: &mut HashMap<Handle<Vertex>, Handle<Edge<3>>>,
) -> ValidationResult<Cycle<3>> {
    // Can't panic. We already ruled out the "continuous edge" case above, so
    // these edges must have vertices.
    let [vertices_bottom, vertices_top] = [&edge_bottom, &edge_top]
        .map(|edge| edge.get().vertices.expect_vertices());

    // Can be simplified, once `zip` is stabilized:
    // https://doc.rust-lang.org/std/primitive.array.html#method.zip
    let [b_a, b_b] = vertices_bottom;
    let [t_a, t_b] = vertices_top;
    let vertices = [(b_a, t_a), (b_b, t_b)];

    // Create (or retrieve from the cache, `vertex_bottom_to_edge`) side edges
    // from the vertices of this source/bottom edge.
    //
    // Can be cleaned up, once `try_map` is stable:
    // https://doc.rust-lang.org/std/primitive.array.html#method.try_map
    let side_edges = vertices.map(|(vertex_bottom, vertex_top)| {
        // We only need to create the edge, if it hasn't already been created
        // for a neighboring side face. Let's check our cache, to see if that's
        // the case.
        let edge = vertex_bottom_to_edge
            .get(&vertex_bottom.canonical())
            .cloned();
        if let Some(edge) = edge {
            return Ok(edge);
        }

        let points_canonical = [vertex_bottom.clone(), vertex_top]
            .map(|vertex| vertex.canonical().get().point);

        let edge_canonical = Edge::builder(target)
            .build_line_segment_from_points(points_canonical)?;

        vertex_bottom_to_edge
            .insert(vertex_bottom.canonical(), edge_canonical.clone());

        Ok(edge_canonical)
    });
    let [a, b]: [Result<_, ValidationError>; 2] = side_edges;
    let [edge_side_a, edge_side_b] = [a?, b?];

    target.merge(Cycle::new([
        edge_bottom,
        edge_top,
        edge_side_a,
        edge_side_b,
    ]))
}

fn create_side_face(
    sweep: &mut Sweep,
    surface: Surface,
    cycle: Handle<Cycle<3>>,
) -> Result<(), ValidationError> {
    let surface = sweep.target.insert(surface)?;

    sweep.target.insert(Face::new(
        surface,
        vec![cycle],
        Vec::new(),
        sweep.color,
    ))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use fj_math::{Point, Scalar, Transform, Vector};

    use crate::{
        algorithms::Tolerance,
        geometry::Surface,
        shape::{Handle, Shape},
        topology::Face,
    };

    use super::sweep_shape;

    #[test]
    fn sweep() -> anyhow::Result<()> {
        let tolerance = Tolerance::from_scalar(Scalar::ONE)?;

        let sketch =
            Triangle::new([[0., 0.], [1., 0.], [0., 1.]], 0.0f64, false)?;

        let swept = sweep_shape(
            sketch.shape,
            Vector::from([0., 0., 1.]),
            tolerance,
            [255, 0, 0, 255],
        )?;

        let bottom_face =
            Triangle::new([[0., 0.], [1., 0.], [0., 1.]], 0.0f64, true)?
                .face
                .get();
        let top_face =
            Triangle::new([[0., 0.], [1., 0.], [0., 1.]], 1.0f64, false)?
                .face
                .get();

        let mut contains_bottom_face = false;
        let mut contains_top_face = false;

        for face in swept.faces() {
            if face.get().clone() == bottom_face {
                contains_bottom_face = true;
            }
            if face.get().clone() == top_face {
                contains_top_face = true;
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
        fn new(
            points: [impl Into<Point<2>>; 3],
            z_offset: impl Into<Scalar>,
            reverse: bool,
        ) -> anyhow::Result<Self> {
            let mut shape = Shape::new();

            let surface =
                Surface::xy_plane().transform(&Transform::translation([
                    Scalar::ZERO,
                    Scalar::ZERO,
                    z_offset.into(),
                ]));
            let face = Face::builder(surface, &mut shape)
                .with_exterior_polygon(points)
                .build()?;

            if reverse {
                shape
                    .update()
                    .update_all(|surface: &mut Surface| {
                        *surface = surface.reverse();
                    })
                    .validate()?;
            }

            Ok(Self { shape, face })
        }
    }
}
