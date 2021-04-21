use std::convert::Infallible;

use nalgebra::Point3;

use crate::geometry::{
    conversions::ToPolygon,
    triangulation::brute_force::{self, triangulate},
    Mesh, Triangle3,
};

pub trait ToMesh {
    type Error;

    fn to_mesh(
        self,
        tolerance: f32,
        mesh: &mut Mesh,
    ) -> Result<(), Self::Error>;
}

impl ToMesh for Mesh {
    type Error = Infallible;

    fn to_mesh(
        self,
        _tolerance: f32,
        mesh: &mut Mesh,
    ) -> Result<(), Self::Error> {
        // TASK: I think just replacing the mesh works for current use cases,
        //       but it doesn't seem right. Unfortunately merging meshes seems
        //       to be somewhat non-trivial, and the whole distinction between
        //       geometry mesh and graphics mesh is confusing anyway.
        //
        //       This needs more investigation and probably a thorough clean-up.
        *mesh = self;
        Ok(())
    }
}

impl<T> ToMesh for T
where
    T: ToPolygon,
{
    type Error = brute_force::InternalError;

    fn to_mesh(
        self,
        tolerance: f32,
        mesh: &mut Mesh,
    ) -> Result<(), Self::Error> {
        let polygon = self.to_polygon(tolerance);
        let triangles = triangulate(polygon)?;

        for triangle in triangles {
            let a_x: f32 = triangle.a.x.into();
            let a_y: f32 = triangle.a.y.into();
            let b_x: f32 = triangle.b.x.into();
            let b_y: f32 = triangle.b.y.into();
            let c_x: f32 = triangle.c.x.into();
            let c_y: f32 = triangle.c.y.into();

            let a = mesh.vertex(Point3::new(a_x, a_y, 0.0));
            let b = mesh.vertex(Point3::new(b_x, b_y, 0.0));
            let c = mesh.vertex(Point3::new(c_x, c_y, 0.0));

            mesh.triangle(a, b, c);
        }

        Ok(())
    }
}

impl ToMesh for &Triangle3 {
    type Error = Infallible;

    fn to_mesh(
        self,
        _tolerance: f32,
        mesh: &mut Mesh,
    ) -> Result<(), Self::Error> {
        let i0 = mesh.vertex(self.a);
        let i1 = mesh.vertex(self.b);
        let i2 = mesh.vertex(self.c);

        mesh.triangle(i0, i1, i2);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Mesh, Triangle3};

    use crate::geometry::ToMesh as _;

    #[test]
    fn triangle_should_convert_to_mesh() {
        let triangle =
            Triangle3::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);

        let mut mesh = Mesh::new();
        triangle.to_mesh(0.0, &mut mesh).unwrap();

        let triangles = mesh.triangles();

        assert_eq!(triangles.0, vec![triangle]);
    }
}
