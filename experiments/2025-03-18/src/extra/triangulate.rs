use fj_interop::{Color, MeshTriangle, Tolerance, TriMesh};
use fj_math::Triangle;
use geo::{Contains, Coord};
use spade::Triangulation;

use crate::topology::face::Face;

use super::{point::TriangulationPoint, projected_face::ProjectedFace};

pub fn triangulate(face: &Face, tolerance: impl Into<Tolerance>) -> TriMesh {
    let face = ProjectedFace::new(face, tolerance);

    let triangles_in_face = triangles(&face.points)
        .into_iter()
        .filter(|triangle| {
            let points = triangle.map(|point| point.point_surface);
            let triangle = Triangle { points };

            let [x, y] =
                triangle.center().coords.components.map(|s| s.into_f64());
            face.polygon_from_half_edges.contains(&Coord { x, y })
        })
        .map(|triangle| {
            let points = triangle.map(|point| point.point_global);
            MeshTriangle {
                inner: Triangle { points },
                is_internal: face.is_internal,
                color: Color::default(),
            }
        });

    let mut mesh = TriMesh::new();
    mesh.triangles.extend(triangles_in_face);

    mesh
}

fn triangles(points: &[TriangulationPoint]) -> Vec<[TriangulationPoint; 3]> {
    let mut triangulation = spade::ConstrainedDelaunayTriangulation::<_>::new();

    // We're passing duplicate points to the triangulation here. It doesn't seem
    // to mind though.
    triangulation
        .add_constraint_edges(points.iter().copied(), true)
        .unwrap();

    triangulation
        .inner_faces()
        .map(|triangle| triangle.vertices().map(|vertex| *vertex.data()))
        .collect()
}
