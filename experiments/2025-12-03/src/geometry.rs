use fj_math::{Point, Scalar, Vector};

#[derive(Clone, Copy, Debug)]
pub struct Arc {
    pub to: Point<2>,
    pub radius: Scalar,
    pub tolerance: Scalar,
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub to: Point<2>,
}

pub struct Surface {
    pub origin: Point<3>,
    pub axes: [Vector<3>; 2],
}

impl Surface {
    pub fn local_to_global(&self, local: Point<2>) -> Point<3> {
        let [u, v] = local.coords.components;
        let [axis_u, axis_v] = self.axes;

        self.origin + axis_u * u + axis_v * v
    }
}
