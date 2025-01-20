use super::{Bivector, Point, Vector};

#[derive(Clone, Copy, Debug)]
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

    pub fn normal(&self) -> Vector<3> {
        self.u().cross(self.v()).normalize()
    }

    pub fn point_from_local(&self, point: impl Into<Point<2>>) -> Point<3> {
        let [u, v] = point.into().coords.components;
        self.origin + self.coords.a * u + self.coords.b * v
    }

    pub fn project_point(&self, point: impl Into<Point<3>>) -> Point<2> {
        let point = point.into();
        let origin_to_point = point - self.origin;

        let min_distance_plane_to_point = origin_to_point.dot(&self.normal());
        let point_in_plane =
            point - self.normal() * min_distance_plane_to_point;
        let origin_to_point_in_plane = point_in_plane - self.origin;

        let u = origin_to_point_in_plane.dot(&self.u());
        let v = origin_to_point_in_plane.dot(&self.v());

        Point::from([u, v])
    }

    pub fn flip(mut self) -> Self {
        self.coords.b = -self.coords.b;
        self
    }

    pub fn translate(self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            origin: self.origin + offset,
            coords: self.coords,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Bivector, Point, Vector};

    use super::Plane;

    #[test]
    fn project_point() {
        let plane = Plane {
            origin: Point::from([1., 1., 1.]),
            coords: Bivector {
                a: Vector::from([1., 0., 0.]),
                b: Vector::from([0., 1., 0.]),
            },
        };

        assert_eq!(plane.project_point([2., 2., 2.]), Point::from([1., 1.]));
    }
}
