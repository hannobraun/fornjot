use crate::{Point, Vector};

/// A plane
#[derive(Clone, Copy, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Plane {
    origin: Point<3>,
    u: Vector<3>,
    v: Vector<3>,
}

impl Plane {
    /// Create a `Plane` from a parametric description
    pub fn from_parametric(
        origin: Point<3>,
        u: Vector<3>,
        v: Vector<3>,
    ) -> Self {
        Self { origin, u, v }
    }

    /// Access the origin of the plane
    pub fn origin(&self) -> Point<3> {
        self.origin
    }

    /// Access the u-vector of the plane
    pub fn u(&self) -> Vector<3> {
        self.u
    }

    /// Access the v-vector of the plane
    pub fn v(&self) -> Vector<3> {
        self.v
    }
}
