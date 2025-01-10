use std::fmt;

use spade::Triangulation;

use crate::{
    geometry::{AnyOp, Handle, Operation, Sketch, Triangle},
    math::{Plane, Point},
    storage::Store,
};

use super::Vertex;

pub struct Face {
    pub surface: Handle<Plane>,
    pub vertices: Vec<Handle<Vertex>>,
}

impl Face {
    pub fn new(
        sketch: &Sketch,
        surface: Handle<Plane>,
        vertices: &mut Store<Vertex>,
    ) -> Self {
        let vertices = sketch
            .points
            .iter()
            .copied()
            .map(|point| {
                let point = surface.point_from_local(point);
                let vertex = Vertex::from(point);
                vertices.insert(vertex)
            })
            .collect();

        Self { surface, vertices }
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "face")
    }
}

impl Operation for Face {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.extend(self.vertices.iter().map(|vertex| **vertex));
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        // This is a placeholder implementation that only supports convex faces.

        let mut triangulation =
            spade::ConstrainedDelaunayTriangulation::<_>::new();

        triangulation
            .add_constraint_edges(
                self.vertices.iter().map(|vertex| TriangulationPoint {
                    point_surface: self.surface.project_point(vertex.point),
                    point_vertex: vertex.point,
                }),
                true,
            )
            .unwrap();

        triangles.extend(triangulation.inner_faces().map(|triangle| {
            let vertices =
                triangle.vertices().map(|vertex| vertex.data().point_vertex);
            Triangle { vertices }
        }));
    }

    fn children(&self) -> Vec<AnyOp> {
        self.vertices.iter().map(|vertex| vertex.to_any()).collect()
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
