use fj_math::{Scalar, Vector};

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    geometry::path::GlobalPath,
    objects::{Face, Objects, Shell},
    services::Service,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for Handle<Face> {
    type Swept = Handle<Shell>;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &mut Service<Objects>,
    ) -> Self::Swept {
        let path = path.into();

        let mut faces = Vec::new();

        let is_negative_sweep = {
            let u = match self.surface().geometry().u {
                GlobalPath::Circle(_) => todo!(
                    "Sweeping from faces defined in round surfaces is not \
                    supported"
                ),
                GlobalPath::Line(line) => line.direction(),
            };
            let v = self.surface().geometry().v;

            let normal = u.cross(&v);

            normal.dot(&path) < Scalar::ZERO
        };

        let bottom_face = {
            if is_negative_sweep {
                self.clone()
            } else {
                self.clone().reverse(objects)
            }
        };
        faces.push(bottom_face);

        let top_face = {
            let mut face = self.clone().translate(path, objects);

            if is_negative_sweep {
                face = face.reverse(objects);
            };

            face
        };
        faces.push(top_face);

        // Generate side faces
        for cycle in self.all_cycles() {
            for half_edge in cycle.half_edges() {
                let half_edge = if is_negative_sweep {
                    half_edge.clone().reverse(objects)
                } else {
                    half_edge.clone()
                };

                let face = (half_edge, self.color())
                    .sweep_with_cache(path, cache, objects);

                faces.push(face);
            }
        }

        Shell::builder().with_faces(faces).build(objects)
    }
}

#[cfg(test)]
mod tests {
    use std::array;

    use fj_interop::{
        ext::{ArrayExt, SliceExt},
        mesh::Color,
    };

    use crate::{
        algorithms::{reverse::Reverse, transform::TransformObject},
        builder::{FaceBuilder, HalfEdgeBuilder},
        insert::Insert,
        objects::{Sketch, Vertex},
        partial::{
            Partial, PartialFace, PartialGlobalEdge, PartialHalfEdge,
            PartialObject,
        },
        services::Services,
    };

    use super::Sweep;

    const TRIANGLE: [[f64; 2]; 3] = [[0., 0.], [1., 0.], [0., 1.]];

    const UP: [f64; 3] = [0., 0., 1.];
    const DOWN: [f64; 3] = [0., 0., -1.];

    #[test]
    fn sweep_up() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();
        let solid = Sketch::builder()
            .with_polygon_from_points(
                surface.clone(),
                TRIANGLE,
                &mut services.objects,
            )
            .build(&mut services.objects)
            .sweep(UP, &mut services.objects);

        let bottom = PartialFace::default()
            .with_exterior_polygon_from_points(surface.clone(), TRIANGLE)
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .reverse(&mut services.objects);
        let top = PartialFace::default()
            .with_exterior_polygon_from_points(
                surface.translate(UP, &mut services.objects),
                TRIANGLE,
            )
            .build(&mut services.objects)
            .insert(&mut services.objects);

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let triangle = TRIANGLE.as_slice();
        let side_faces = triangle.array_windows_ext().map(|&[a, b]| {
            let half_edge = {
                let vertices = array::from_fn(|_| Partial::<Vertex>::new());
                let global_curve = {
                    let [vertex, _] = &vertices;
                    vertex.read().curve.read().global_form.clone()
                };
                let global_vertices = vertices.each_ref_ext().map(|vertex| {
                    vertex.read().surface_form.read().global_form.clone()
                });

                let mut half_edge = PartialHalfEdge {
                    vertices,
                    global_form: Partial::from_partial(PartialGlobalEdge {
                        curve: global_curve,
                        vertices: global_vertices,
                    }),
                };
                half_edge.update_as_line_segment_from_points(
                    Partial::from_full_entry_point(
                        services.objects.surfaces.xy_plane(),
                    ),
                    [a, b],
                );

                half_edge
                    .build(&mut services.objects)
                    .insert(&mut services.objects)
            };
            (half_edge, Color::default()).sweep(UP, &mut services.objects)
        });

        assert!(side_faces
            .into_iter()
            .all(|face| solid.find_face(&face).is_some()));
    }

    #[test]
    fn sweep_down() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();
        let solid = Sketch::builder()
            .with_polygon_from_points(
                surface.clone(),
                TRIANGLE,
                &mut services.objects,
            )
            .build(&mut services.objects)
            .sweep(DOWN, &mut services.objects);

        let bottom = PartialFace::default()
            .with_exterior_polygon_from_points(
                surface.clone().translate(DOWN, &mut services.objects),
                TRIANGLE,
            )
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .reverse(&mut services.objects);
        let top = PartialFace::default()
            .with_exterior_polygon_from_points(surface, TRIANGLE)
            .build(&mut services.objects)
            .insert(&mut services.objects);

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let triangle = TRIANGLE.as_slice();
        let side_faces = triangle.array_windows_ext().map(|&[a, b]| {
            let half_edge = {
                let vertices = array::from_fn(|_| Partial::<Vertex>::new());
                let global_curve = {
                    let [vertex, _] = &vertices;
                    vertex.read().curve.read().global_form.clone()
                };
                let global_vertices = vertices.each_ref_ext().map(|vertex| {
                    vertex.read().surface_form.read().global_form.clone()
                });

                let mut half_edge = PartialHalfEdge {
                    vertices,
                    global_form: Partial::from_partial(PartialGlobalEdge {
                        curve: global_curve,
                        vertices: global_vertices,
                    }),
                };
                half_edge.update_as_line_segment_from_points(
                    Partial::from_full_entry_point(
                        services.objects.surfaces.xy_plane(),
                    ),
                    [a, b],
                );

                half_edge
                    .build(&mut services.objects)
                    .insert(&mut services.objects)
                    .reverse(&mut services.objects)
            };
            (half_edge, Color::default()).sweep(DOWN, &mut services.objects)
        });

        assert!(side_faces
            .into_iter()
            .all(|face| solid.find_face(&face).is_some()));
    }
}
