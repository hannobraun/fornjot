use fj_math::{Point, Vector};

/// # A plane
pub struct Plane {
    /// # The origin point of the plane
    pub origin: Point<3>,

    /// # The axes that define the plane's orientation and coordinate system
    pub axes: [Vector<3>; 2],
}

impl Plane {
    /// # Convert a surface-local point to a global one
    pub fn local_point_to_global(&self, local: Point<2>) -> Point<3> {
        self.origin + self.local_vector_to_global(local.coords)
    }

    /// # Convert a surface-local vector to a global one
    pub fn local_vector_to_global(&self, local: Vector<2>) -> Vector<3> {
        let [u, v] = local.components;
        let [axis_u, axis_v] = self.axes;

        axis_u * u + axis_v * v
    }
}
