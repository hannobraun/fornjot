use fj_math::{Line, Point, Vector};

use super::CurveGeometry;

pub struct SweptCurve {
    pub curve: Box<dyn CurveGeometry>,
    pub path: Vector<3>,
}

impl SweptCurve {
    pub fn plane_from_points(points: [impl Into<Point<3>>; 3]) -> Self {
        let [a, b, c] = points.map(Into::into);

        let (curve, _) = Line::from_points([a, b]);
        Self {
            curve: Box::new(curve),
            path: c - a,
        }
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
            let origin = self.curve.point_from_local(u);
            let line = Line::from_origin_and_direction(origin, self.path);
            line.point_to_line_coords(point)
        };

        Point::from([u.t, v.t])
    }

    pub fn flip(&self) -> Self {
        Self {
            curve: self.curve.clone_curve_geometry(),
            path: -self.path,
        }
    }

    pub fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            curve: self.curve.translate(offset.into()),
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
            curve: Box::new(Line::from_origin_and_direction(
                [1., 1., 1.],
                [1., 0., 0.],
            )),
            path: Vector::from([0., 1., 0.]),
        };

        assert_eq!(plane.project_point([2., 2., 2.]), Point::from([1., 1.]));
    }
}
