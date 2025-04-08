use fj_math::{Line, Point, Transform, Vector};

#[derive(Clone, Copy, Debug)]
pub struct SweptCurve {
    pub curve: Line<3>,
    pub path: Vector<3>,
}

impl SweptCurve {
    pub fn from_points([a, b, c]: [Point<3>; 3]) -> Self {
        let (curve, _) = Line::from_points([a, b]);
        Self { curve, path: c - a }
    }

    pub fn origin(&self) -> Point<3> {
        self.curve.origin()
    }

    pub fn u(&self) -> Vector<3> {
        self.curve.direction()
    }

    pub fn v(&self) -> Vector<3> {
        self.path
    }

    pub fn normal(&self) -> Vector<3> {
        self.u().cross(&self.v()).normalize()
    }

    pub fn point_from_local(&self, point: impl Into<Point<2>>) -> Point<3> {
        let [u, v] = point.into().coords.components;
        self.origin() + self.u() * u + self.v() * v
    }

    pub fn project_point(&self, point: impl Into<Point<3>>) -> Point<2> {
        let point = point.into();
        let origin_to_point = point - self.origin();

        let min_distance_plane_to_point = origin_to_point.dot(&self.normal());
        let point_in_plane =
            point - self.normal() * min_distance_plane_to_point;
        let origin_to_point_in_plane = point_in_plane - self.origin();

        let u = origin_to_point_in_plane.dot(&self.u());
        let v = origin_to_point_in_plane.dot(&self.v());

        Point::from([u, v])
    }

    pub fn flip(&self) -> Self {
        Self {
            curve: self.curve,
            path: -self.path,
        }
    }

    pub fn translate(self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            curve: self.curve.transform(&Transform::translation(offset)),
            path: self.path,
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Line, Point, Vector};

    use super::SweptCurve;

    #[test]
    fn project_point() {
        let plane = SweptCurve {
            curve: Line::from_origin_and_direction([1., 1., 1.], [1., 0., 0.]),
            path: Vector::from([0., 1., 0.]),
        };

        assert_eq!(plane.project_point([2., 2., 2.]), Point::from([1., 1.]));
    }
}
