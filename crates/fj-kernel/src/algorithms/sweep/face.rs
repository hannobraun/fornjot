use fj_math::{Scalar, Vector};

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    objects::{Face, Objects, Shell},
    path::GlobalPath,
};

use super::{Sweep, SweepCache};

impl Sweep for Face {
    type Swept = Shell;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &Objects,
    ) -> Self::Swept {
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
                self.clone().reverse()
            }
        };
        faces.push(bottom_face);

        let top_face = {
            let mut face = self.clone().translate(path, objects);

            if is_negative_sweep {
                face = face.reverse();
            };

            face
        };
        faces.push(top_face);

        // Generate side faces
        for cycle in self.all_cycles() {
            for half_edge in cycle.half_edges() {
                let half_edge = if is_negative_sweep {
                    half_edge.clone().reverse()
                } else {
                    half_edge.clone()
                };

                let face = (half_edge, self.color())
                    .sweep_with_cache(path, cache, objects);

                faces.push(face);
            }
        }

        Shell::new().with_faces(faces)
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::mesh::Color;

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
    fn sweep_up() {
        let objects = Objects::new();

        let surface = objects.surfaces.xy_plane();
        let solid = Sketch::builder(&objects, surface.clone())
            .build_polygon_from_points(TRIANGLE)
            .sweep(UP, &objects);

        let bottom = Face::builder(&objects, surface.clone())
            .with_exterior_polygon_from_points(TRIANGLE)
            .build()
            .reverse();
        let top = Face::builder(&objects, surface.translate(UP, &objects))
            .with_exterior_polygon_from_points(TRIANGLE)
            .build();

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let mut side_faces = TRIANGLE.windows(2).map(|window| {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable:
            // https://doc.rust-lang.org/std/primitive.slice.html#method.array_windows
            let [a, b] = [window[0], window[1]];

            let half_edge = HalfEdge::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_line_segment_from_points([a, b])
                .build(&objects);
            (half_edge, Color::default()).sweep(UP, &objects)
        });

        assert!(side_faces.all(|face| solid.find_face(&face).is_some()));
    }

    #[test]
    fn sweep_down() {
        let objects = Objects::new();

        let surface = objects.surfaces.xy_plane();
        let solid = Sketch::builder(&objects, surface.clone())
            .build_polygon_from_points(TRIANGLE)
            .sweep(DOWN, &objects);

        let bottom =
            Face::builder(&objects, surface.clone().translate(DOWN, &objects))
                .with_exterior_polygon_from_points(TRIANGLE)
                .build()
                .reverse();
        let top = Face::builder(&objects, surface)
            .with_exterior_polygon_from_points(TRIANGLE)
            .build();

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let mut side_faces = TRIANGLE.windows(2).map(|window| {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable:
            // https://doc.rust-lang.org/std/primitive.slice.html#method.array_windows
            let [a, b] = [window[0], window[1]];

            let half_edge = HalfEdge::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_line_segment_from_points([a, b])
                .build(&objects)
                .reverse();
            (half_edge, Color::default()).sweep(DOWN, &objects)
        });

        assert!(side_faces.all(|face| solid.find_face(&face).is_some()));
    }
}
