use spade::Triangulation;

use crate::{
    geometry::{TriMesh, Triangle},
    math::{Plane, Point},
    object::Handle,
    topology::vertex::Vertex,
};

pub fn triangulate(vertices: &[Handle<Vertex>], surface: &Plane) -> TriMesh {
    // This is a placeholder implementation that only supports convex faces.

    let triangles = triangles(vertices, surface);

    let mut mesh = TriMesh::new();
    mesh.triangles.extend(triangles);

    mesh
}

fn triangles(vertices: &[Handle<Vertex>], surface: &Plane) -> Vec<Triangle> {
    let mut triangulation = spade::ConstrainedDelaunayTriangulation::<_>::new();

    triangulation
        .add_constraint_edges(
            vertices.iter().map(|vertex| {
                // Here, we project a 3D point (from the vertex) into the face's
                // surface, creating a 2D point. Through the surface, this 2D
                // point has a position in 3D space.
                //
                // But this position isn't necessarily going to be the same as
                // the position of the original 3D point, due to numerical
                // inaccuracy.
                //
                // This doesn't matter. Neither does the fact, that other faces
                // might share the same vertices and project them into their own
                // surfaces, creating more redundancy.
                //
                // The reason that it doesn't, is that we're using the projected
                // 2D points _only_ for this local triangulation. Once that
                // tells us how the different 3D points must connect, we use the
                // original 3D points to build those triangles. We never convert
                // the 2D points back into 3D.
                let point_surface = surface.project_point(vertex.point);

                TriangulationPoint {
                    point_surface,
                    point_vertex: vertex.point,
                }
            }),
            true,
        )
        .unwrap();

    triangulation
        .inner_faces()
        .map(|triangle| {
            let points =
                triangle.vertices().map(|vertex| vertex.data().point_vertex);
            Triangle { points }
        })
        .collect()
}

#[derive(Clone, Copy)]
struct TriangulationPoint {
    point_surface: Point<2>,
    point_vertex: Point<3>,
}

impl spade::HasPosition for TriangulationPoint {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.point_surface.coords.components.map(|s| s.value());
        spade::Point2 { x, y }
    }
}
