use super::{Bivector, Point, Vector};

#[derive(Clone, Copy)]
pub struct Plane {
    pub origin: Point<3>,
    pub coords: Bivector<3>,
}

impl Plane {
    pub fn u(&self) -> Vector<3> {
        self.coords.a
    }

    pub fn v(&self) -> Vector<3> {
        self.coords.b
    }

    #[allow(unused)] // code to use it is being worked on
    pub fn normal(&self) -> Vector<3> {
        self.u().cross(self.v()).normalize()
    }

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
