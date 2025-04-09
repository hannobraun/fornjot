use fj_math::{Line, Point, Transform, Vector};

use super::CurveGeometry;

pub struct SweptCurve {
    pub curve: Line<3>,
    pub path: Vector<3>,
}

impl SweptCurve {
    pub fn plane_from_points([a, b, c]: [Point<3>; 3]) -> Self {
        let (curve, _) = Line::from_points([a, b]);
        Self { curve, path: c - a }
    }

    pub fn v(&self) -> Vector<3> {
        self.path
    }

    pub fn point_from_local(&self, point: impl Into<Point<2>>) -> Point<3> {
        let [u, v] = point.into().coords.components;
        self.curve.point_from_local(Point::from([u])) + self.v() * v
    }

    pub fn project_point(&self, point: impl Into<Point<3>>) -> Point<2> {
        let point = point.into();

        let u = self.curve.project_point(point);
        let v = {
            let origin = self.curve.point_from_line_coords(u);
            let line = Line::from_origin_and_direction(origin, self.path);
            line.point_to_line_coords(point)
        };

        Point::from([u.t, v.t])
    }

    pub fn flip(&self) -> Self {
        Self {
            curve: self.curve,
            path: -self.path,
        }
    }

    pub fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
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
