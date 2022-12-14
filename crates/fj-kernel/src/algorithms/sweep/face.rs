use fj_math::{Scalar, Vector};

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    builder::ShellBuilder,
    geometry::path::GlobalPath,
    insert::Insert,
    objects::{Face, Objects, Shell},
    partial::{PartialObject, PartialShell},
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

        PartialShell::default()
            .with_faces(faces)
            .build(objects)
            .insert(objects)
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::{ext::SliceExt, mesh::Color};

    use crate::{
        algorithms::{reverse::Reverse, transform::TransformObject},
        builder::{FaceBuilder, HalfEdgeBuilder, SketchBuilder},
        insert::Insert,
        partial::{PartialFace, PartialHalfEdge, PartialObject, PartialSketch},
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
        let mut sketch = PartialSketch::default();
        sketch.add_polygon_from_points(surface.clone(), TRIANGLE);
        let solid = sketch
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .sweep(UP, &mut services.objects);

        let mut bottom = PartialFace::default();
        bottom.with_exterior_polygon_from_points(surface.clone(), TRIANGLE);
        let bottom = bottom
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .reverse(&mut services.objects);
        let mut top = PartialFace::default();
        top.with_exterior_polygon_from_points(
            surface.translate(UP, &mut services.objects),
            TRIANGLE,
        );
        let top = top
            .build(&mut services.objects)
            .insert(&mut services.objects);

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let triangle = TRIANGLE.as_slice();
        let side_faces = triangle.array_windows_ext().map(|&[a, b]| {
            let half_edge = {
                let mut half_edge = PartialHalfEdge::default();
                half_edge.update_as_line_segment_from_points(
                    services.objects.surfaces.xy_plane(),
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
        let mut sketch = PartialSketch::default();
        sketch.add_polygon_from_points(surface.clone(), TRIANGLE);
        let solid = sketch
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .sweep(DOWN, &mut services.objects);

        let mut bottom = PartialFace::default();
        bottom.with_exterior_polygon_from_points(
            surface.clone().translate(DOWN, &mut services.objects),
            TRIANGLE,
        );
        let bottom = bottom
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .reverse(&mut services.objects);
        let mut top = PartialFace::default();
        top.with_exterior_polygon_from_points(surface, TRIANGLE);
        let top = top
            .build(&mut services.objects)
            .insert(&mut services.objects);

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let triangle = TRIANGLE.as_slice();
        let side_faces = triangle.array_windows_ext().map(|&[a, b]| {
            let half_edge = {
                let mut half_edge = PartialHalfEdge::default();
                half_edge.update_as_line_segment_from_points(
                    services.objects.surfaces.xy_plane(),
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
