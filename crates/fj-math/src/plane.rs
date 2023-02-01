use crate::{Line, Point, Scalar, Vector};

/// A plane
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Plane {
    origin: Point<3>,
    u: Vector<3>,
    v: Vector<3>,
}

impl Plane {
    /// Create a `Plane` from a parametric description
    pub fn from_parametric(
        origin: impl Into<Point<3>>,
        u: impl Into<Vector<3>>,
        v: impl Into<Vector<3>>,
    ) -> Self {
        let origin = origin.into();
        let u = u.into();
        let v = v.into();

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

    /// Compute the normal of the plane
    pub fn normal(&self) -> Vector<3> {
        self.u().cross(&self.v()).normalize()
    }

    /// Convert the plane to three-point form
    pub fn three_point_form(&self) -> [Point<3>; 3] {
        let a = self.origin();
        let b = self.origin() + self.u();
        let c = self.origin() + self.v();

        [a, b, c]
    }

    /// Convert the plane to constant-normal form
    pub fn constant_normal_form(&self) -> (Scalar, Vector<3>) {
        let normal = self.normal();
        let distance = normal.dot(&self.origin().coords);

        (distance, normal)
    }

    /// Determine whether the plane is parallel to the given vector
    pub fn is_parallel_to_vector(&self, vector: &Vector<3>) -> bool {
        self.normal().dot(vector) == Scalar::ZERO
    }

    /// Project a vector into the plane
    pub fn project_vector(&self, vector: impl Into<Vector<3>>) -> Vector<2> {
        let vector = vector.into();

        Vector::from([
            vector.scalar_projection_onto(&self.u()),
            vector.scalar_projection_onto(&self.v()),
        ])
    }

    /// Project a line into the plane
    pub fn project_line(&self, line: &Line<3>) -> Line<2> {
        let line_origin_relative_to_plane = line.origin() - self.origin();
        let line_origin_in_plane = Point {
            coords: Vector::from([
                self.u()
                    .scalar_projection_onto(&line_origin_relative_to_plane),
                self.v()
                    .scalar_projection_onto(&line_origin_relative_to_plane),
            ]),
        };

        let line_direction_in_plane = self.project_vector(line.direction());

        Line::from_origin_and_direction(
            line_origin_in_plane,
            line_direction_in_plane,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Plane, Vector};

    #[test]
    fn project_vector() {
        let plane =
            Plane::from_parametric([1., 1., 1.], [1., 0., 0.], [0., 1., 0.]);

        assert_eq!(plane.project_vector([1., 0., 1.]), Vector::from([1., 0.]));
        assert_eq!(plane.project_vector([0., 1., 1.]), Vector::from([0., 1.]));
    }
}
