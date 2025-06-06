use spade::Triangulation;

use crate::extra::triangulate::TriangulationPoint;

pub fn triangles<'a>(
    points_from_half_edges: impl IntoIterator<Item = TriangulationPoint>,
    points_from_surface: impl IntoIterator<Item = &'a TriangulationPoint>,
) -> Vec<[TriangulationPoint; 3]> {
    let mut triangulation = spade::ConstrainedDelaunayTriangulation::<_>::new();

    // We're passing duplicate points to the triangulation here. It doesn't seem
    // to mind though.
    triangulation
        .add_constraint_edges(points_from_half_edges, true)
        .unwrap();

    for point in points_from_surface {
        triangulation.insert(*point).unwrap();
    }

    triangulation
        .inner_faces()
        .map(|triangle| triangle.vertices().map(|vertex| *vertex.data()))
        .collect()
}
