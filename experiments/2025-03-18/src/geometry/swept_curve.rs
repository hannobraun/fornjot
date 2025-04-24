use fj_math::{Line, Point, Vector};

use super::AbsoluteCurveGeometry;

pub struct SweptCurve {
    pub curve: AbsoluteCurveGeometry,
    pub path: Vector<3>,
}

impl SweptCurve {
    pub fn plane_from_coord_system(
        origin: impl Into<Point<3>>,
        axes: [impl Into<Vector<3>>; 2],
    ) -> Self {
        let origin = origin.into();
        let [u, v] = axes.map(Into::into);

        let line = Line::from_origin_and_direction(origin, u);

        Self {
            curve: AbsoluteCurveGeometry {
                geometry: Box::new(line),
            },
            path: v,
        }
    }

    pub fn plane_from_points(points: [impl Into<Point<3>>; 3]) -> Self {
        let [a, b, c] = points.map(Into::into);
        Self::plane_from_coord_system(a, [b - a, c - a])
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

        let u = self.curve.geometry.project_point(point);
        let v = {
            let origin = self.curve.point_from_local(u);
            let line = Line::from_origin_and_direction(origin, self.path);
            line.point_to_line_coords(point)
        };

        Point::from([u.t, v.t])
    }

    pub fn flip(&self) -> Self {
        Self {
            curve: self.curve.clone(),
            path: -self.path,
        }
    }

    pub fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            curve: self.curve.translate(offset),
            path: self.path,
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Point;

    use super::SweptCurve;

    #[test]
    fn project_point() {
        let plane = SweptCurve::plane_from_coord_system(
            [1., 1., 1.],
            [[1., 0., 0.], [0., 1., 0.]],
        );

        assert_eq!(plane.project_point([2., 2., 2.]), Point::from([1., 1.]));
    }
}
