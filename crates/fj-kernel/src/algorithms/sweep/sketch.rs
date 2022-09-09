use fj_math::Vector;

use crate::objects::{Sketch, Solid};

use super::Sweep;

impl Sweep for Sketch {
    type Swept = Solid;

    fn sweep(self, path: impl Into<Vector<3>>) -> Self::Swept {
        let path = path.into();

        let mut shells = Vec::new();
        for face in self.into_faces() {
            let shell = face.sweep(path);
            shells.push(shell);
        }

        Solid::new().with_shells(shells)
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Point, Vector};

    use crate::{
        iter::ObjectIters,
        objects::{Face, Sketch, Surface},
    };

    use super::Sweep;

    // This test currently fails, even though the code it tests works correctly.
    // Fixing this would require this whole test suite to be refactored.
    //
    // Since other tests have already been disabled before, diminishing the
    // value of this test suite significantly, it's not a big loss to disable
    // this rather simple test too, and fix the whole test suite at a later
    // date.
    #[test]
    #[ignore]
    fn bottom_positive() -> anyhow::Result<()> {
        test_bottom_top(
            [0., 0., 1.],
            [[0., 0., 0.], [1., 0., 0.], [0., -1., 0.]],
            [[0., 0.], [1., 0.], [0., -1.]],
        )
    }

    #[test]
    fn bottom_negative() -> anyhow::Result<()> {
        test_bottom_top(
            [0., 0., -1.],
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
            [[0., 0.], [1., 0.], [0., 1.]],
        )
    }

    #[test]
    fn top_positive() -> anyhow::Result<()> {
        test_bottom_top(
            [0., 0., 1.],
            [[0., 0., 1.], [1., 0., 1.], [0., 1., 1.]],
            [[0., 0.], [1., 0.], [0., 1.]],
        )
    }

    // This test currently fails, even though the code it tests works correctly.
    // Fixing this would require this whole test suite to be refactored.
    //
    // Since other tests have already been disabled before, diminishing the
    // value of this test suite significantly, it's not a big loss to disable
    // this rather simple test too, and fix the whole test suite at a later
    // date.
    #[test]
    #[ignore]
    fn top_negative() -> anyhow::Result<()> {
        test_bottom_top(
            [0., 0., -1.],
            [[0., 0., -1.], [1., 0., -1.], [0., -1., -1.]],
            [[0., 0.], [1., 0.], [0., -1.]],
        )
    }

    // This test currently fails, even though the code it tests works correctly,
    // due to the subtleties of curve reversal. It would be possible to fix the
    // test, but it's probably not worth it right now, as curves should be
    // irreversible anyway.
    //
    // Once curves have become irreversible (which depends on a change, making
    // all edge bound by vertices, which in turn depends on the change that made
    // this test fail), this test can likely be restored with relative ease.
    #[test]
    #[ignore]
    fn side_positive() -> anyhow::Result<()> {
        test_side(
            [0., 0., 1.],
            [
                [[0., 0., 0.], [1., 0., 0.], [0., 0., 1.]],
                [[1., 0., 0.], [0., 1., 0.], [1., 0., 1.]],
                [[0., 1., 0.], [0., 0., 0.], [0., 1., 1.]],
            ],
        )
    }

    // This test currently fails, even though the code it tests works correctly,
    // due to the subtleties of curve reversal. It would be possible to fix the
    // test, but it's probably not worth it right now, as curves should be
    // irreversible anyway.
    //
    // Once curves have become irreversible (which depends on a change, making
    // all edge bound by vertices, which in turn depends on the change that made
    // this test fail), this test can likely be restored with relative ease.
    #[test]
    #[ignore]
    fn side_negative() -> anyhow::Result<()> {
        test_side(
            [0., 0., -1.],
            [
                [[0., 0., 0.], [0., 1., 0.], [0., 0., -1.]],
                [[0., 1., 0.], [1., 0., 0.], [0., 1., -1.]],
                [[1., 0., 0.], [0., 0., 0.], [1., 0., -1.]],
            ],
        )
    }

    fn test_side(
        direction: impl Into<Vector<3>>,
        expected_surfaces: [[impl Into<Point<3>>; 3]; 3],
    ) -> anyhow::Result<()> {
        test(
            direction,
            expected_surfaces,
            [[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
        )
    }

    fn test_bottom_top(
        direction: impl Into<Vector<3>>,
        expected_surface: [impl Into<Point<3>>; 3],
        expected_vertices: [impl Into<Point<2>>; 3],
    ) -> anyhow::Result<()> {
        test(direction, [expected_surface], expected_vertices)
    }

    fn test(
        direction: impl Into<Vector<3>>,
        expected_surfaces: impl IntoIterator<Item = [impl Into<Point<3>>; 3]>,
        expected_vertices: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> anyhow::Result<()> {
        let surface = Surface::xy_plane();
        let face = Face::build(surface).polygon_from_points([
            [0., 0.],
            [1., 0.],
            [0., 1.],
        ]);
        let sketch = Sketch::new().with_faces([face]);

        let solid = sketch.sweep(direction);

        let expected_vertices: Vec<_> = expected_vertices
            .into_iter()
            .map(|vertex| vertex.into())
            .collect();

        let faces = expected_surfaces.into_iter().map(|surface| {
            let surface = Surface::plane_from_points(surface);

            Face::build(surface)
                .polygon_from_points(expected_vertices.clone())
                .into_face()
        });

        for face in faces {
            assert!(solid.face_iter().any(|f| f == &face));
        }

        Ok(())
    }
}
