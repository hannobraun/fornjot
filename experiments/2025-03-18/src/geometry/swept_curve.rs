use fj_math::{Point, Vector};

use super::{AnchoredCurve, Line};

pub struct SweptCurve {
    pub u: AnchoredCurve,
    pub v: Vector<3>,
}

impl SweptCurve {
    pub fn plane_from_coord_system(
        origin: impl Into<Point<3>>,
        axes: [impl Into<Vector<3>>; 2],
    ) -> Self {
        let origin = origin.into();
        let [u, v] = axes.map(Into::into);

        let u = Line { direction: u };

        Self {
            u: AnchoredCurve {
                origin,
                floating: Box::new(u),
            },
            v,
        }
    }

    pub fn plane_from_points(points: [impl Into<Point<3>>; 3]) -> Self {
        let [a, b, c] = points.map(Into::into);
        Self::plane_from_coord_system(a, [b - a, c - a])
    }

    pub fn point_from_local(&self, point: impl Into<Point<2>>) -> Point<3> {
        let [u, v] = point.into().coords.components;
        self.u.point_from_local(Point::from([u])) + self.v * v
    }

    pub fn project_point(&self, point: impl Into<Point<3>>) -> Point<2> {
        let point = point.into();

        let u = self.u.project_point(point);
        let v = {
            let origin = self.u.point_from_local(u);
            let line =
                AnchoredCurve::line_from_origin_and_direction(origin, self.v);

            line.project_point(point)
        };

        Point::from([u.t, v.t])
    }

    pub fn flip(&self) -> Self {
        Self {
            u: self.u.clone(),
            v: -self.v,
        }
    }

    pub fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            u: self.u.translate(offset),
            v: self.v,
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
