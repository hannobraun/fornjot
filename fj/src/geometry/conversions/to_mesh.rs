use std::convert::Infallible;

use nalgebra::Point3;

use crate::geometry::{
    conversions::ToPolygon,
    operations::linear_extrude::LinearExtrude,
    shapes::{Mesh, Point, Polygon},
    triangulation::brute_force::{self, triangulate, InternalError},
};

pub trait ToMesh {
    type Error;

    fn to_mesh(self, tolerance: f32) -> Result<Mesh, Self::Error>;
}

impl ToMesh for Mesh {
    type Error = Infallible;

    fn to_mesh(self, _tolerance: f32) -> Result<Mesh, Self::Error> {
        Ok(self)
    }
}

impl<T> ToMesh for T
where
    T: ToPolygon,
{
    type Error = brute_force::InternalError;

    fn to_mesh(self, tolerance: f32) -> Result<Mesh, Self::Error> {
        let polygon = self.to_polygon(tolerance);
        polygon_to_mesh(polygon, 0.0)
    }
}

impl<Sketch> ToMesh for LinearExtrude<Sketch>
where
    Sketch: ToPolygon,
{
    type Error = InternalError;

    fn to_mesh(self, tolerance: f32) -> Result<Mesh, Self::Error> {
        let sketch = self.sketch.to_polygon(tolerance);

        let mut lower = polygon_to_mesh(sketch.clone(), 0.0)?;
        let upper = polygon_to_mesh(sketch.clone(), self.height)?;

        // Triangles need to point down, which is the outside direction.
        lower.invert_triangles();

        // Merge meshes.
        for [a, b, c] in upper.triangles() {
            lower.triangle(a.position, b.position, c.position);
        }

        // Build walls.
        for edge in sketch.edges() {
            lower.triangle(
                Point::from_xyz(edge.a.x, edge.a.y, 0.0),
                Point::from_xyz(edge.b.x, edge.b.y, 0.0),
                Point::from_xyz(edge.a.x, edge.a.y, self.height),
            );
            lower.triangle(
                Point::from_xyz(edge.a.x, edge.a.y, self.height),
                Point::from_xyz(edge.b.x, edge.b.y, 0.0),
                Point::from_xyz(edge.b.x, edge.b.y, self.height),
            );
        }

        Ok(lower)
    }
}

fn polygon_to_mesh(
    polygon: Polygon,
    z: f32,
) -> Result<Mesh, brute_force::InternalError> {
    let mut mesh = Mesh::new();
    let triangles = triangulate(polygon)?;

    for triangle in triangles {
        let a_x: f32 = triangle.a.x.into();
        let a_y: f32 = triangle.a.y.into();
        let b_x: f32 = triangle.b.x.into();
        let b_y: f32 = triangle.b.y.into();
        let c_x: f32 = triangle.c.x.into();
        let c_y: f32 = triangle.c.y.into();

        mesh.triangle(
            Point3::new(a_x, a_y, z),
            Point3::new(b_x, b_y, z),
            Point3::new(c_x, c_y, z),
        );
    }

    Ok(mesh)
}
