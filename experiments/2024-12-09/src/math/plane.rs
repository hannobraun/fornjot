use super::{Bivector, Point, Vector};

#[derive(Clone, Copy)]
pub struct Plane {
    pub origin: Point<3>,
    pub coords: Bivector<3>,
}

impl Plane {
    pub fn point_from_local(&self, point: impl Into<Point<2>>) -> Point<3> {
        let [u, v] = point.into().coords.components;
        self.origin + self.coords.a * u + self.coords.b * v
    }

    pub fn translate(self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            origin: self.origin + offset,
            coords: self.coords,
        }
    }
}
