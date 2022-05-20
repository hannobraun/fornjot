use std::collections::HashMap;

use fj_math::{Transform, Triangle, Vector};

use crate::{
    geometry::{Surface, SweptCurve},
    shape::{Shape, ValidationError},
    topology::{Cycle, Edge, Face},
};

use super::{transform_shape, CycleApprox, Tolerance};

/// Create a new shape by sweeping an existing one
pub fn sweep_shape(
    source: Shape,
    path: Vector<3>,
    tolerance: Tolerance,
    color: [u8; 4],
) -> Result<Shape, ValidationError> {
    let translation = Transform::translation(path);

    let (mut bottom, source_to_bottom) = source.clone_shape();
    bottom
        .update()
        .update_all(|surface: &mut Surface| *surface = surface.reverse())
        .validate()?;

    let (mut top, source_to_top) = source.clone_shape();
    transform_shape(&mut top, &translation);

    let mut target = Shape::new();
    target.merge_shape(&bottom)?;
    target.merge_shape(&top)?;

    // Create the side faces.
    for cycle_source in source.cycles() {
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

            let approx = CycleApprox::new(&cycle_source.get(), tolerance);

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
        } else {
            // If there's no continuous edge, we can create the non-
            // continuous faces using boundary representation.

            let mut vertex_bottom_to_edge = HashMap::new();

            for edge_source in &cycle_source.get().edges {
                let edge_source = edge_source.canonical();

                // Can't panic. We already ruled out the continuous edge case
                // above, so this edge must have vertices.
                let vertices_source =
                    edge_source.get().vertices.clone().unwrap();

                // Create (or retrieve from the cache, `vertex_bottom_to_edge`)
                // side edges from the vertices of this source/bottom edge.
                let [side_edge_a, side_edge_b] =
                    vertices_source.map(|vertex_source| {
                        let vertex_bottom = source_to_bottom
                            .vertices()
                            .get(&vertex_source.canonical())
                            .unwrap()
                            .clone();

                        vertex_bottom_to_edge
                            .entry(vertex_bottom.clone())
                            .or_insert_with(|| {
                                let vertex_top = source_to_top
                                    .vertices()
                                    .get(&vertex_source.canonical())
                                    .unwrap()
                                    .clone();

                                let points = [vertex_bottom, vertex_top]
                                    .map(|vertex| vertex.get().point());

                                Edge::builder(&mut target)
                                    .build_line_segment_from_points(points)
                                    .unwrap()
                            })
                            .clone()
                    });

                // Now we have everything we need to create the side face from
                // this source/bottom edge.

                let bottom_edge =
                    source_to_bottom.edges().get(&edge_source).unwrap().clone();
                let top_edge =
                    source_to_top.edges().get(&edge_source).unwrap().clone();

                let surface =
                    target.insert(Surface::SweptCurve(SweptCurve {
                        curve: bottom_edge.get().curve(),
                        path,
                    }))?;

                let cycle = target.merge(Cycle::new(vec![
                    bottom_edge,
                    top_edge,
                    side_edge_a,
                    side_edge_b,
                ]))?;

                target.insert(Face::new(
                    surface,
                    vec![cycle],
                    Vec::new(),
                    color,
                ))?;
            }
        }
    }

    Ok(target)
}

#[cfg(test)]
mod tests {
    use fj_math::{Point, Scalar, Vector};

    use crate::{
        algorithms::Tolerance,
        geometry::Surface,
        shape::{Handle, Shape},
        topology::{Cycle, Edge, Face},
    };

    use super::sweep_shape;

    #[test]
    fn sweep() -> anyhow::Result<()> {
        let tolerance = Tolerance::from_scalar(Scalar::ONE).unwrap();

        let sketch =
            Triangle::new([[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]], false)?;

        let swept = sweep_shape(
            sketch.shape,
            Vector::from([0., 0., 1.]),
            tolerance,
            [255, 0, 0, 255],
        )?;

        let bottom_face =
            Triangle::new([[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]], true)?
                .face
                .get();
        let top_face =
            Triangle::new([[0., 0., 1.], [1., 0., 1.], [0., 1., 1.]], false)?
                .face
                .get();

        let mut contains_bottom_face = false;
        let mut contains_top_face = false;

        for face in swept.faces() {
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
        fn new(
            points: [impl Into<Point<3>>; 3],
            reverse: bool,
        ) -> anyhow::Result<Self> {
            let mut shape = Shape::new();

            let [a, b, c] = points.map(|point| point.into());

            let ab = Edge::builder(&mut shape)
                .build_line_segment_from_points([a, b])?;
            let bc = Edge::builder(&mut shape)
                .build_line_segment_from_points([b, c])?;
            let ca = Edge::builder(&mut shape)
                .build_line_segment_from_points([c, a])?;

            let cycles = shape.insert(Cycle::new(vec![ab, bc, ca]))?;

            let surface = Surface::plane_from_points([a, b, c]);
            let surface = if reverse { surface.reverse() } else { surface };
            let surface = shape.insert(surface)?;

            let abc =
                Face::new(surface, vec![cycles], Vec::new(), [255, 0, 0, 255]);

            let face = shape.insert(abc)?;

            Ok(Self { shape, face })
        }
    }
}
