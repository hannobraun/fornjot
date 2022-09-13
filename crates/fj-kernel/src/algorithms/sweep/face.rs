use fj_math::{Scalar, Vector};

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    objects::{CurveKind, Face, Shell},
};

use super::Sweep;

impl Sweep for Face {
    type Swept = Shell;

    fn sweep(self, path: impl Into<Vector<3>>) -> Self::Swept {
        let path = path.into();

        let mut faces = Vec::new();

        let is_negative_sweep = {
            let a = match self.surface().u {
                CurveKind::Circle(_) => todo!(
                    "Sweeping from faces defined in round surfaces is not \
                    supported"
                ),
                CurveKind::Line(line) => line.direction(),
            };
            let b = self.surface().path;

            let normal = a.cross(&b);

            normal.dot(&path) < Scalar::ZERO
        };

        let bottom_face = create_bottom_face(self.clone(), is_negative_sweep);
        faces.push(bottom_face);

        let top_face = create_top_face(self.clone(), path, is_negative_sweep);
        faces.push(top_face);

        for cycle in self.all_cycles() {
            for &half_edge in cycle.half_edges() {
                let edge = if is_negative_sweep {
                    half_edge.reverse()
                } else {
                    half_edge
                };
                let face = (edge, self.color()).sweep(path);
                faces.push(face);
            }
        }

        Shell::new().with_faces(faces)
    }
}

fn create_bottom_face(face: Face, is_negative_sweep: bool) -> Face {
    if is_negative_sweep {
        face
    } else {
        face.reverse()
    }
}

fn create_top_face(
    face: Face,
    path: Vector<3>,
    is_negative_sweep: bool,
) -> Face {
    let mut face = face.translate(path);

    if is_negative_sweep {
        face = face.reverse();
    };

    face
}

#[cfg(test)]
mod tests {
    use fj_interop::mesh::Color;

    use crate::{
        algorithms::{reverse::Reverse, transform::TransformObject},
        objects::{Face, HalfEdge, Sketch, Surface},
    };

    use super::Sweep;

    const TRIANGLE: [[f64; 2]; 3] = [[0., 0.], [1., 0.], [0., 1.]];

    const UP: [f64; 3] = [0., 0., 1.];
    const DOWN: [f64; 3] = [0., 0., -1.];

    #[test]
    fn sweep_up() {
        let surface = Surface::xy_plane();
        let solid = Sketch::build(surface)
            .polygon_from_points(TRIANGLE)
            .sweep(UP);

        let bottom = Face::build(surface)
            .polygon_from_points(TRIANGLE)
            .into_face()
            .reverse();
        let top = Face::build(surface.translate(UP))
            .polygon_from_points(TRIANGLE)
            .into_face();

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let mut side_faces = TRIANGLE.windows(2).map(|window| {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable:
            // https://doc.rust-lang.org/std/primitive.slice.html#method.array_windows
            let [a, b] = [window[0], window[1]];

            let half_edge = HalfEdge::build(Surface::xy_plane())
                .line_segment_from_points([a, b]);
            (half_edge, Color::default()).sweep(UP)
        });

        assert!(side_faces.all(|face| solid.find_face(&face).is_some()));
    }

    #[test]
    fn sweep_down() {
        let surface = Surface::xy_plane();
        let solid = Sketch::build(surface)
            .polygon_from_points(TRIANGLE)
            .sweep(DOWN);

        let bottom = Face::build(surface.translate(DOWN))
            .polygon_from_points(TRIANGLE)
            .into_face()
            .reverse();
        let top = Face::build(surface)
            .polygon_from_points(TRIANGLE)
            .into_face();

        assert!(solid.find_face(&bottom).is_some());
        assert!(solid.find_face(&top).is_some());

        let mut side_faces = TRIANGLE.windows(2).map(|window| {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable:
            // https://doc.rust-lang.org/std/primitive.slice.html#method.array_windows
            let [a, b] = [window[0], window[1]];

            let half_edge = HalfEdge::build(Surface::xy_plane())
                .line_segment_from_points([a, b])
                .reverse();
            (half_edge, Color::default()).sweep(DOWN)
        });

        assert!(side_faces.all(|face| solid.find_face(&face).is_some()));
    }
}
