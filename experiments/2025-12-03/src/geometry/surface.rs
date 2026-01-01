use fj_math::{Point, Vector};

pub struct Plane {
    pub origin: Point<3>,
    pub axes: [Vector<3>; 2],
}

impl Plane {
    pub fn local_to_global(&self, local: Point<2>) -> Point<3> {
        let [u, v] = local.coords.components;
        let [axis_u, axis_v] = self.axes;

        self.origin + axis_u * u + axis_v * v
    }
}
