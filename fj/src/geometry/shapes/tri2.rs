use nalgebra::Point2;
use parry2d::{
    query::PointQueryWithLocation as _,
    shape::{Segment, Triangle, TrianglePointLocation},
};

use super::{Pnt2, Seg2};

#[derive(Clone, Copy, Debug)]
pub struct Tri2 {
    pub a: Pnt2,
    pub b: Pnt2,
    pub c: Pnt2,
}

impl Tri2 {
    pub fn new(
        a: impl Into<Pnt2>,
        b: impl Into<Pnt2>,
        c: impl Into<Pnt2>,
    ) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }

    pub fn new_ccw(
        a: impl Into<Pnt2>,
        b: impl Into<Pnt2>,
        c: impl Into<Pnt2>,
    ) -> Self {
        let a: Point2<f32> = a.into().into();
        let b: Point2<f32> = b.into().into();
        let c: Point2<f32> = c.into().into();

        let a_b = Segment::new(a, b);
        let a_c = c - a;
        let c_is_left_of_a_b = a_b.scaled_normal().dot(&a_c) < 0.0;

        if c_is_left_of_a_b {
            Self::new(a, b, c)
        } else {
            Self::new(a, c, b)
        }
    }

    pub fn vertices(&self) -> [Pnt2; 3] {
        [self.a, self.b, self.c]
    }

    pub fn edges(&self) -> [Seg2; 3] {
        [
            Seg2::new(self.a, self.b),
            Seg2::new(self.b, self.c),
            Seg2::new(self.c, self.a),
        ]
    }

    pub fn is_clockwise(&self) -> bool {
        // Algorithm from: https://algs4.cs.princeton.edu/91primitives/

        let a = self.a;
        let b = self.b;
        let c = self.c;

        (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y) < 0.0
    }

    pub fn contains(&self, point: impl Into<Pnt2>) -> bool {
        let triangle: Triangle = self.into();
        let point = point.into();

        let (projection, location) =
            triangle.project_local_point_and_get_location(&point.into(), true);

        let is_inside = match (projection.is_inside, location) {
            (false, _) => false,
            (true, TrianglePointLocation::OnVertex(_)) => false,
            (true, _) => true,
        };

        is_inside
    }
}

impl From<Triangle> for Tri2 {
    fn from(triangle: Triangle) -> Self {
        Self {
            a: triangle.a.into(),
            b: triangle.b.into(),
            c: triangle.c.into(),
        }
    }
}

impl From<Tri2> for Triangle {
    fn from(triangle: Tri2) -> Self {
        Self {
            a: triangle.a.into(),
            b: triangle.b.into(),
            c: triangle.c.into(),
        }
    }
}

impl From<&Tri2> for Triangle {
    fn from(triangle: &Tri2) -> Self {
        Self {
            a: triangle.a.into(),
            b: triangle.b.into(),
            c: triangle.c.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shapes::Pnt2;

    use super::Tri2;

    #[test]
    fn is_clockwise_should_tell_whether_triangle_is_clockwise() {
        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);
        let c = Pnt2::new(0.0, 1.0);

        let ccw = Tri2::new(a, b, c);
        assert_eq!(ccw.is_clockwise(), false);

        let cw = Tri2::new(a, c, b);
        assert_eq!(cw.is_clockwise(), true);
    }

    #[test]
    fn contains_should_tell_whether_triangle_contains_point() {
        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);
        let c = Pnt2::new(0.0, 1.0);
        let triangle = Tri2::new(a, b, c);

        let on_edge = Pnt2::new(0.5, 0.0);
        let in_triangle = Pnt2::new(0.5, 0.5);

        assert_eq!(triangle.contains(&a), false);
        assert_eq!(triangle.contains(&b), false);
        assert_eq!(triangle.contains(&b), false);
        assert_eq!(triangle.contains(&on_edge), true);
        assert_eq!(triangle.contains(&in_triangle), true);
    }
}
