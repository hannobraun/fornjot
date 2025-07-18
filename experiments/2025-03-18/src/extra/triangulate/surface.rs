use fj_interop::Tolerance;
use fj_math::Aabb;

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
    pub fn from_surface(
        surface: &Surface,
        boundary: &Aabb<2>,
        _: impl Into<Tolerance>,
    ) -> Self {
        let approx = surface.geometry.approximate(boundary);

        let surface_points = approx
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

        let mut all_points = surface_points.clone();
        all_points.extend(boundary_points);

        let triangles = triangles([], all_points)
            .into_iter()
            .map(|triangle| MeshTriangle { points: triangle })
            .collect();

        Self {
            points: surface_points,
            triangles,
        }
    }
}

#[derive(Debug)]
pub struct MeshTriangle {
    pub points: [TriangulationPoint; 3],
}
