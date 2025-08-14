use fj_interop::{Tolerance, TriMesh};
use fj_math::{Aabb, Point, Scalar, Triangle};

use crate::{
    extra::triangulate::{TriangulationPoint, delaunay::triangles},
    topology::surface::Surface,
};

#[derive(Debug)]
pub struct SurfaceMesh {
    pub points: Vec<TriangulationPoint>,
    pub triangles: Vec<MeshTriangle>,
}

impl SurfaceMesh {
    pub fn new(
        surface: &Surface,
        boundary: &Aabb<2>,
        tolerance: impl Into<Tolerance>,
    ) -> Self {
        let approx = surface.geometry.approximate(boundary, tolerance.into());

        let curvature_points = approx
            .curvature
            .into_iter()
            .map(|point_surface| {
                TriangulationPoint::from_surface_point(
                    point_surface,
                    surface.geometry.as_ref(),
                )
            })
            .collect::<Vec<_>>();

        let boundary_points =
            approx.boundary.into_iter().map(|point_surface| {
                TriangulationPoint::from_surface_point(
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

        Self {
            points: curvature_points,
            triangles,
        }
    }

    #[allow(unused)] // useful for occasional debugging
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

    pub fn project_point(
        &self,
        point_global: Point<3>,
        tolerance: Tolerance,
    ) -> Point<2> {
        let mut projection = None;

        for triangle in &self.triangles {
            let (point_surface, distance) =
                triangle.project_point(point_global);

            let Some((_, min_distance)) = projection else {
                projection = Some((point_surface, distance));
                continue;
            };

            if distance < min_distance {
                projection = Some((point_surface, distance));
            }
        }

        let Some((point_surface, distance)) = projection else {
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
            \n\
            Distance to surface: {distance}\n\
            \n\
            Surface mesh: {self:#?}",
        );

        point_surface
    }
}

#[derive(Debug)]
pub struct MeshTriangle {
    pub points: [TriangulationPoint; 3],
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

    pub fn project_point(&self, point_global: Point<3>) -> (Point<2>, Scalar) {
        let triangle_global = self.to_global_triangle();
        let triangle_surface = self.to_surface_triangle();

        let barycentric_coords =
            triangle_global.point_to_barycentric_coords(point_global);

        let point_surface =
            triangle_surface.point_from_barycentric_coords(barycentric_coords);

        let distance = (point_global
            - triangle_global.closest_point(point_global))
        .magnitude();

        (point_surface, distance)
    }
}
