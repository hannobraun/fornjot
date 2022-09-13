//! Shape triangulation

mod delaunay;
mod polygon;

use fj_interop::mesh::Mesh;
use fj_math::Point;

use self::{delaunay::TriangulationPoint, polygon::Polygon};

use super::approx::{face::FaceApprox, Approx, Tolerance};

/// Triangulate a shape
pub trait Triangulate: Sized {
    /// Triangulate the shape
    fn triangulate(self, tolerance: impl Into<Tolerance>) -> Mesh<Point<3>> {
        let mut mesh = Mesh::new();
        self.triangulate_into_mesh(tolerance, &mut mesh);
        mesh
    }

    /// Triangulate a partial shape into the provided mesh
    ///
    /// This is a low-level method, intended for implementation of
    /// `Triangulate`. Most callers should prefer [`Triangulate::triangulate`].
    fn triangulate_into_mesh(
        self,
        tolerance: impl Into<Tolerance>,
        mesh: &mut Mesh<Point<3>>,
    );
}

impl<T> Triangulate for T
where
    T: Approx,
    T::Approximation: IntoIterator<Item = FaceApprox>,
{
    fn triangulate_into_mesh(
        self,
        tolerance: impl Into<Tolerance>,
        mesh: &mut Mesh<Point<3>>,
    ) {
        let tolerance = tolerance.into();
        let approx = self.approx(tolerance);

        for approx in approx {
            approx.triangulate_into_mesh(tolerance, mesh);
        }
    }
}

impl Triangulate for FaceApprox {
    fn triangulate_into_mesh(
        self,
        _: impl Into<Tolerance>,
        mesh: &mut Mesh<Point<3>>,
    ) {
        let points: Vec<_> = self
            .points()
            .into_iter()
            .map(|point| TriangulationPoint {
                point_surface: point.local_form,
                point_global: point.global_form,
            })
            .collect();
        let face_as_polygon = Polygon::new()
            .with_exterior(
                self.exterior
                    .points()
                    .into_iter()
                    .map(|point| point.local_form),
            )
            .with_interiors(self.interiors.into_iter().map(|interior| {
                interior.points().into_iter().map(|point| point.local_form)
            }));

        let mut triangles =
            delaunay::triangulate(points, self.coord_handedness);
        triangles.retain(|triangle| {
            face_as_polygon
                .contains_triangle(triangle.map(|point| point.point_surface))
        });

        for triangle in triangles {
            let points = triangle.map(|point| point.point_global);
            mesh.push_triangle(points, self.color);
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::mesh::Mesh;
    use fj_math::{Point, Scalar};

    use crate::{
        algorithms::approx::{Approx, Tolerance},
        objects::{Face, Surface},
    };

    use super::Triangulate;

    #[test]
    fn simple() -> anyhow::Result<()> {
        let a = [0., 0.];
        let b = [2., 0.];
        let c = [2., 2.];
        let d = [0., 1.];

        let surface = Surface::xy_plane();
        let face = Face::build(surface).polygon_from_points([a, b, c, d]);

        let a = Point::from(a).to_xyz();
        let b = Point::from(b).to_xyz();
        let c = Point::from(c).to_xyz();
        let d = Point::from(d).to_xyz();

        let triangles = triangulate(face)?;

        assert!(triangles.contains_triangle([a, b, d]));
        assert!(triangles.contains_triangle([b, c, d]));
        assert!(!triangles.contains_triangle([a, b, c]));
        assert!(!triangles.contains_triangle([a, c, d]));

        Ok(())
    }

    #[test]
    fn simple_hole() -> anyhow::Result<()> {
        let a = [0., 0.];
        let b = [4., 0.];
        let c = [4., 4.];
        let d = [0., 4.];

        let e = [1., 1.];
        let f = [3., 1.];
        let g = [3., 3.];
        let h = [1., 2.];

        let surface = Surface::xy_plane();
        let face = Face::build(surface)
            .polygon_from_points([a, b, c, d])
            .with_hole([e, f, g, h]);

        let triangles = triangulate(face)?;

        let a = Point::from(a).to_xyz();
        let d = Point::from(d).to_xyz();
        let e = Point::from(e).to_xyz();
        let f = Point::from(f).to_xyz();
        let g = Point::from(g).to_xyz();
        let h = Point::from(h).to_xyz();

        // Should contain some triangles from the polygon. Don't need to test
        // them all.
        assert!(triangles.contains_triangle([a, e, h]));
        assert!(triangles.contains_triangle([a, d, h]));

        // Shouldn't contain any possible triangle from the hole.
        assert!(!triangles.contains_triangle([e, f, g]));
        assert!(!triangles.contains_triangle([e, g, h]));
        assert!(!triangles.contains_triangle([e, f, h]));
        assert!(!triangles.contains_triangle([f, g, h]));

        Ok(())
    }

    #[ignore]
    #[test]
    fn sharp_concave_shape() -> anyhow::Result<()> {
        //
        //                c
        //               /|
        //   e          / |
        //   |\       /   |
        //   | |     /    |
        //   | \   /      |
        //   |  \ /       |
        //   |   d        |
        //   a ---------- b
        //

        let a = Point::from([0., 0.]);
        let b = Point::from([0.4, 0.]);
        //let b = Point::from([0.5, 0.]); // test passes with this change
        let c = Point::from([0.4, 1.0]);
        let d = Point::from([0.1, 0.1]);
        let e = Point::from([0., 0.8]);

        let surface = Surface::xy_plane();
        let face = Face::build(surface).polygon_from_points([a, b, c, d, e]);

        let triangles = triangulate(face)?;

        let a3 = a.to_xyz();
        let b3 = b.to_xyz();
        let c3 = c.to_xyz();
        let d3 = d.to_xyz();
        let e3 = e.to_xyz();

        assert!(triangles.contains_triangle([a3, b3, d3]));
        assert!(triangles.contains_triangle([b3, c3, d3]));
        assert!(triangles.contains_triangle([a3, d3, e3]));

        assert!(!triangles.contains_triangle([b3, e3, d3]));

        Ok(())
    }

    fn triangulate(face: impl Into<Face>) -> anyhow::Result<Mesh<Point<3>>> {
        let tolerance = Tolerance::from_scalar(Scalar::ONE)?;
        Ok(face.into().approx(tolerance).triangulate(tolerance))
    }
}
