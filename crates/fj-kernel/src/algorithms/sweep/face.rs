use fj_math::{Scalar, Vector};

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    geometry::path::GlobalPath,
    insert::Insert,
    objects::{Face, Objects, Shell},
    partial::{Partial, PartialObject, PartialShell},
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
        faces.push(bottom_face.clone());

        let top_face = {
            let mut face = self.clone().translate(path, objects);

            if is_negative_sweep {
                face = face.reverse(objects);
            };

            face
        };
        faces.push(top_face);

        // Generate side faces
        for cycle in bottom_face.all_cycles().cloned() {
            let cycle = cycle.reverse(objects);

            for half_edge in cycle.half_edges().cloned() {
                let face = (half_edge, self.color())
                    .sweep_with_cache(path, cache, objects);

                faces.push(face);
            }
        }

        let faces = faces.into_iter().map(Partial::from).collect();
        PartialShell { faces }.build(objects).insert(objects)
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::{ext::SliceExt, mesh::Color};

    use crate::{
        algorithms::{reverse::Reverse, transform::TransformObject},
        builder::{CycleBuilder, HalfEdgeBuilder, SketchBuilder},
        insert::Insert,
        partial::{
            Partial, PartialFace, PartialHalfEdge, PartialObject, PartialSketch,
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
        let sketch = {
            let mut sketch = PartialSketch::default();

            let mut face = sketch.add_face();
            face.write().exterior.write().surface =
                Partial::from(surface.clone());
            face.write()
                .exterior
                .write()
                .update_as_polygon_from_points(TRIANGLE);

            sketch
        };
        let solid = sketch
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .sweep(UP, &mut services.objects);

        let mut bottom = PartialFace::default();
        bottom.exterior.write().surface = Partial::from(surface.clone());
        bottom
            .exterior
            .write()
            .update_as_polygon_from_points(TRIANGLE);
        let bottom = bottom
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .reverse(&mut services.objects);
        let mut top = PartialFace::default();
        top.exterior.write().surface =
            Partial::from(surface.translate(UP, &mut services.objects));
        top.exterior.write().update_as_polygon_from_points(TRIANGLE);
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
        let sketch = {
            let mut sketch = PartialSketch::default();

            let mut face = sketch.add_face();
            face.write().exterior.write().surface =
                Partial::from(surface.clone());
            face.write()
                .exterior
                .write()
                .update_as_polygon_from_points(TRIANGLE);

            sketch
        };
        let solid = sketch
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .sweep(DOWN, &mut services.objects);

        let mut bottom = PartialFace::default();
        bottom.exterior.write().surface = Partial::from(
            surface.clone().translate(DOWN, &mut services.objects),
        );
        bottom
            .exterior
            .write()
            .update_as_polygon_from_points(TRIANGLE);
        let bottom = bottom
            .build(&mut services.objects)
            .insert(&mut services.objects)
            .reverse(&mut services.objects);
        let mut top = PartialFace::default();
        top.exterior.write().surface = Partial::from(surface);
        top.exterior.write().update_as_polygon_from_points(TRIANGLE);
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
