use std::fmt;

use itertools::Itertools;
use spade::Triangulation;

use crate::{
    geometry::{AnyOp, Handle, Operation, OperationOutput, TriMesh, Triangle},
    math::{Plane, Point, Vector},
};

use super::vertex::Vertex;

#[derive(Debug)]
pub struct Face {
    surface: Plane,
    vertices: Vec<Handle<Vertex>>,
}

impl Face {
    pub fn new(
        surface: Plane,
        vertices: impl IntoIterator<Item = Handle<Vertex>>,
    ) -> Self {
        Self {
            surface,
            vertices: vertices.into_iter().collect(),
        }
    }

    pub fn surface(&self) -> &Plane {
        &self.surface
    }

    pub fn vertices(&self) -> impl Iterator<Item = &Handle<Vertex>> {
        self.vertices.iter()
    }

    pub fn half_edges(&self) -> impl Iterator<Item = [&Handle<Vertex>; 2]> {
        self.vertices
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| [a, b])
    }

    pub fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();

        Self::new(
            self.surface().translate(offset),
            self.vertices()
                .map(|vertex| Handle::new(vertex.translate(offset))),
        )
    }
}

impl Operation for Face {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Face")
    }

    fn tri_mesh(&self) -> TriMesh {
        // This is a placeholder implementation that only supports convex faces.

        let mut triangulation =
            spade::ConstrainedDelaunayTriangulation::<_>::new();

        triangulation
            .add_constraint_edges(
                self.vertices.iter().map(|vertex| {
                    // Here, we project a 3D point (from the vertex) into the
                    // face's surface, creating a 2D point. Through the surface,
                    // this 2D point has a position in 3D space.
                    //
                    // But this position isn't necessarily going to be the same
                    // as the position of the original 3D point, due to
                    // numerical inaccuracy.
                    //
                    // This doesn't matter. Neither does the fact, that other
                    // faces might share the same vertices and project them into
                    // their own surfaces, creating more redundancy.
                    //
                    // The reason that it doesn't, is that we're using the
                    // projected 2D points _only_ for this local triangulation.
                    // Once that tells us how the different 3D points must
                    // connect, we use the original 3D points to build those
                    // triangles. We never convert the 2D points back into 3D.
                    let point_surface =
                        self.surface.project_point(vertex.point);

                    TriangulationPoint {
                        point_surface,
                        point_vertex: vertex.point,
                    }
                }),
                true,
            )
            .unwrap();

        let mut mesh = TriMesh::new();
        mesh.triangles
            .extend(triangulation.inner_faces().map(|triangle| {
                let points = triangle
                    .vertices()
                    .map(|vertex| vertex.data().point_vertex);
                Triangle { points }
            }));

        mesh
    }

    fn children(&self) -> Vec<AnyOp> {
        self.vertices.iter().map(|vertex| vertex.to_any()).collect()
    }
}

impl OperationOutput for Face {
    fn output(&self) -> &Self {
        self
    }
}

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
