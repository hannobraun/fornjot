use fj_math::{Point, Vector};

pub struct Plane {
    pub origin: Point<3>,
    pub axes: [Vector<3>; 2],
}

impl Plane {
    pub fn local_point_to_global(&self, local: Point<2>) -> Point<3> {
        self.origin + self.local_vector_to_global(local.coords)
    }

    pub fn local_vector_to_global(&self, local: Vector<2>) -> Vector<3> {
        let [u, v] = local.components;
        let [axis_u, axis_v] = self.axes;

        axis_u * u + axis_v * v
    }
}
