use fj_interop::{Tolerance, TriMesh};
use fj_math::{Aabb, Point, Scalar, Triangle};

use crate::{
    approx::{delaunay::triangles, point::ApproxPoint},
    topology::surface::Surface,
};

#[derive(Debug)]
pub struct SurfaceApprox {
    points: Vec<ApproxPoint>,
    triangles: Vec<MeshTriangle>,
}

impl SurfaceApprox {
    pub fn new(
        surface: &Surface,
        boundary: &Aabb<2>,
        tolerance: impl Into<Tolerance>,
    ) -> Self {
        let surface_mesh = surface_to_mesh(surface, boundary, tolerance);
        check_that_triangles_are_valid(&surface_mesh);

        surface_mesh
    }

    pub fn points(&self) -> impl Iterator<Item = &ApproxPoint> {
        self.points.iter()
    }

    pub fn project_point(
        &mut self,
        point_global: Point<3>,
        tolerance: Tolerance,
    ) -> Point<2> {
        let mut projection = None;

        for triangle in &self.triangles {
            let (point_surface, closest_point, distance) =
                triangle.project_point(point_global);

            let Some((_, _, min_distance)) = projection else {
                projection = Some((point_surface, closest_point, distance));
                continue;
            };

            if distance < min_distance {
                projection = Some((point_surface, closest_point, distance));
            }
        }

        let Some((point_surface, closest_point, distance)) = projection else {
            unreachable!(
                "Surface mesh can't be empty. At the very least, there must be \
                two triangles from the AABB. This means that the loop above \
                ran at least twice, and the first time it ran it definitely \
                initialized `projection`."
            );
        };

        assert!(
            distance < tolerance.inner(),
            "Expected to project a global point that is coincident with the \
            surface.\n\
            \n\
            Original point: {point_global:?}\n\
            Projected point: {point_surface:?}\n\
            Closest point on surface mesh: {closest_point:?}\n\
            \n\
            Distance to surface: {distance}\n\
            \n\
            Surface mesh: {self:#?}",
        );

        point_surface
    }

    #[allow(unused)] // occasionally useful for debugging
    pub fn to_tri_mesh(&self) -> TriMesh {
        let triangles = self
            .triangles
            .iter()
            .map(|triangle| {
                let triangle = Triangle {
                    points: triangle.points.map(|point| point.point_global),
                };

                fj_interop::MeshTriangle {
                    inner: triangle,
                    is_internal: false,
                    color: fj_interop::Color([0, 0, 255, 255]),
                }
            })
            .collect();

        TriMesh { triangles }
    }
}

#[derive(Debug)]
pub struct MeshTriangle {
    pub points: [ApproxPoint; 3],
}

impl MeshTriangle {
    pub fn to_surface_triangle(&self) -> Triangle<2> {
        Triangle {
            points: self.points.map(|point| point.point_surface),
        }
    }

    pub fn to_global_triangle(&self) -> Triangle<3> {
        Triangle {
            points: self.points.map(|point| point.point_global),
        }
    }

    pub fn project_point(
        &self,
        point_global: Point<3>,
    ) -> (Point<2>, Point<3>, Scalar) {
        let triangle_global = self.to_global_triangle();
        let triangle_surface = self.to_surface_triangle();

        let barycentric_coords =
            triangle_global.point_to_barycentric_coords(point_global);

        let point_surface =
            triangle_surface.point_from_barycentric_coords(barycentric_coords);

        let closest_point = triangle_global.closest_point(point_global);
        let distance = (point_global - closest_point).magnitude();

        (point_surface, closest_point, distance)
    }
}

fn surface_to_mesh(
    surface: &Surface,
    boundary: &Aabb<2>,
    tolerance: impl Into<Tolerance>,
) -> SurfaceApprox {
    let approx = surface.geometry.approximate(boundary, tolerance.into());

    let curvature_points = approx
        .curvature
        .into_iter()
        .map(|point_surface| {
            ApproxPoint::from_surface_point(
                point_surface,
                surface.geometry.as_ref(),
            )
        })
        .collect::<Vec<_>>();

    let boundary_points = approx.boundary.into_iter().map(|point_surface| {
        ApproxPoint::from_surface_point(
            point_surface,
            surface.geometry.as_ref(),
        )
    });

    let mut all_points = curvature_points.clone();
    all_points.extend(boundary_points);

    let triangles = triangles([], all_points)
        .into_iter()
        .map(|triangle| MeshTriangle { points: triangle })
        .collect();

    SurfaceApprox {
        points: curvature_points,
        triangles,
    }
}

fn check_that_triangles_are_valid(surface_approx: &SurfaceApprox) {
    for triangle in &surface_approx.triangles {
        assert!(
            triangle.to_surface_triangle().is_valid(),
            "Triangle is degenerate in surface form: {triangle:#?}",
        );
        assert!(
            triangle.to_global_triangle().is_valid(),
            "Triangle is degenerate in global form: {triangle:#?}",
        );
    }
}
