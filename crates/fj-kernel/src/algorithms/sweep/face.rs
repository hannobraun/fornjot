use fj_math::{Scalar, Vector};

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    objects::{Face, Objects, Shell},
    path::GlobalPath,
    storage::Handle,
    validate::ValidationError,
};

use super::{Sweep, SweepCache};

impl Sweep for Handle<Face> {
    type Swept = Handle<Shell>;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &Objects,
    ) -> Result<Self::Swept, ValidationError> {
        let path = path.into();

        let mut faces = Vec::new();

        let is_negative_sweep = {
            let u = match self.surface().u() {
                GlobalPath::Circle(_) => todo!(
                    "Sweeping from faces defined in round surfaces is not \
                    supported"
                ),
                GlobalPath::Line(line) => line.direction(),
            };
            let v = self.surface().v();

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
            let mut face = self.clone().translate(path, objects)?;

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
                    .sweep_with_cache(path, cache, objects)?;

                faces.push(face);
            }
        }

        Ok(Shell::builder(objects).with_faces(faces).build())
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::{ext::SliceExt, mesh::Color};

    use crate::{
        algorithms::{reverse::Reverse, transform::TransformObject},
        objects::{Face, HalfEdge, Objects, Sketch},
        partial::HasPartial,
    };

    use super::Sweep;

    const TRIANGLE: [[f64; 2]; 3] = [[0., 0.], [1., 0.], [0., 1.]];

    const UP: [f64; 3] = [0., 0., 1.];
    const DOWN: [f64; 3] = [0., 0., -1.];

    #[test]
    fn sweep_up() -> anyhow::Result<()> {
        let objects = Objects::new();

        let surface = objects.surfaces.xy_plane();
        let solid = Sketch::builder(&objects)
            .with_surface(surface.clone())
            .with_polygon_from_points(TRIANGLE)
            .build()
            .sweep(UP, &objects)?;

        let bottom = Face::builder(&objects)
            .with_surface(surface.clone())
            .with_exterior_polygon_from_points(TRIANGLE)
            .build()
            .reverse(&objects);
        let top = Face::builder(&objects)
            .with_surface(surface.translate(UP, &objects)?)
            .with_exterior_polygon_from_points(TRIANGLE)
            .build();

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let triangle = TRIANGLE.as_slice();
        let side_faces = triangle
            .array_windows_ext()
            .map(|&[a, b]| {
                let half_edge = HalfEdge::partial()
                    .with_surface(Some(objects.surfaces.xy_plane()))
                    .as_line_segment_from_points([a, b])
                    .build(&objects)?;
                (half_edge, Color::default()).sweep(UP, &objects)
            })
            .collect::<Result<Vec<_>, _>>()?;

        assert!(side_faces
            .into_iter()
            .all(|face| solid.find_face(&face).is_some()));
        Ok(())
    }

    #[test]
    fn sweep_down() -> anyhow::Result<()> {
        let objects = Objects::new();

        let surface = objects.surfaces.xy_plane();
        let solid = Sketch::builder(&objects)
            .with_surface(surface.clone())
            .with_polygon_from_points(TRIANGLE)
            .build()
            .sweep(DOWN, &objects)?;

        let bottom = Face::builder(&objects)
            .with_surface(surface.clone().translate(DOWN, &objects)?)
            .with_exterior_polygon_from_points(TRIANGLE)
            .build()
            .reverse(&objects);
        let top = Face::builder(&objects)
            .with_surface(surface)
            .with_exterior_polygon_from_points(TRIANGLE)
            .build();

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let triangle = TRIANGLE.as_slice();
        let side_faces = triangle
            .array_windows_ext()
            .map(|&[a, b]| {
                let half_edge = HalfEdge::partial()
                    .with_surface(Some(objects.surfaces.xy_plane()))
                    .as_line_segment_from_points([a, b])
                    .build(&objects)?
                    .reverse(&objects);
                (half_edge, Color::default()).sweep(DOWN, &objects)
            })
            .collect::<Result<Vec<_>, _>>()?;

        assert!(side_faces
            .into_iter()
            .all(|face| solid.find_face(&face).is_some()));
        Ok(())
    }
}
