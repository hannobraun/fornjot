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
    fn triangulate(self) -> Mesh<Point<3>> {
        let mut mesh = Mesh::new();
        self.triangulate_into_mesh(&mut mesh);
        mesh
    }

    /// Triangulate a partial shape into the provided mesh
    ///
    /// This is a low-level method, intended for implementation of
    /// `Triangulate`. Most callers should prefer [`Triangulate::triangulate`].
    fn triangulate_into_mesh(self, mesh: &mut Mesh<Point<3>>);
}

impl<T> Triangulate for (T, Tolerance)
where
    T: Approx,
    T::Approximation: IntoIterator<Item = FaceApprox>,
{
    fn triangulate_into_mesh(self, mesh: &mut Mesh<Point<3>>) {
        let (approx, tolerance) = self;

        let approx = approx.approx(tolerance);

        for approx in approx {
            approx.triangulate_into_mesh(mesh);
        }
    }
}

impl Triangulate for FaceApprox {
    fn triangulate_into_mesh(self, mesh: &mut Mesh<Point<3>>) {
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
        objects::{Face, Objects},
    };

    use super::Triangulate;

    #[test]
    fn simple() -> anyhow::Result<()> {
        let objects = Objects::new();

        let a = [0., 0.];
        let b = [2., 0.];
        let c = [2., 2.];
        let d = [0., 1.];

        let surface = objects.surfaces.xy_plane();
        let face = Face::builder(&objects)
            .with_surface(surface)
            .with_exterior_polygon_from_points([a, b, c, d])
            .build();

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
        let objects = Objects::new();

        let a = [0., 0.];
        let b = [4., 0.];
        let c = [4., 4.];
        let d = [0., 4.];

        let e = [1., 1.];
        let f = [1., 2.];
        let g = [3., 3.];
        let h = [3., 1.];

        let surface = objects.surfaces.xy_plane();
        let face = Face::builder(&objects)
            .with_surface(surface.clone())
            .with_exterior_polygon_from_points([a, b, c, d])
            .with_interior_polygon_from_points([e, f, g, h])
            .build();

        let triangles = triangulate(face)?;

        let a = surface.point_from_surface_coords(a);
        let b = surface.point_from_surface_coords(b);
        let e = surface.point_from_surface_coords(e);
        let f = surface.point_from_surface_coords(f);
        let g = surface.point_from_surface_coords(g);
        let h = surface.point_from_surface_coords(h);

        // Let's test that some correct triangles are present. We don't need to
        // test them all.
        //
        // Please note that there are multiple valid triangulations of any given
        // polygon. So if you change the input data above, or the algorithm, the
        // following assertions might break.
        //
        // This limits the usefulness of this test. It would be better to have a
        // smarter way of verifying the triangulation.
        assert!(triangles.contains_triangle([a, b, e]));
        assert!(triangles.contains_triangle([b, e, h]));

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
        let objects = Objects::new();

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

        let a = [0., 0.];
        let b = [0.4, 0.];
        //let b = [0.5, 0.]; // test passes with this change
        let c = [0.4, 1.0];
        let d = [0.1, 0.1];
        let e = [0., 0.8];

        let surface = objects.surfaces.xy_plane();
        let face = Face::builder(&objects)
            .with_surface(surface.clone())
            .with_exterior_polygon_from_points([a, b, c, d, e])
            .build();

        let triangles = triangulate(face)?;

        let a3 = surface.point_from_surface_coords(a);
        let b3 = surface.point_from_surface_coords(b);
        let c3 = surface.point_from_surface_coords(c);
        let d3 = surface.point_from_surface_coords(d);
        let e3 = surface.point_from_surface_coords(e);

        assert!(triangles.contains_triangle([a3, b3, d3]));
        assert!(triangles.contains_triangle([b3, c3, d3]));
        assert!(triangles.contains_triangle([a3, d3, e3]));

        assert!(!triangles.contains_triangle([b3, e3, d3]));

        Ok(())
    }

    fn triangulate(face: impl Into<Face>) -> anyhow::Result<Mesh<Point<3>>> {
        let tolerance = Tolerance::from_scalar(Scalar::ONE)?;
        Ok(face.into().approx(tolerance).triangulate())
    }
}
