use std::f32::consts::PI;

use crate::{
    geometry::{operations::Difference, shapes::Polygon, Boundary as _},
    Circle,
};

pub trait ToPolygon {
    fn to_polygon(self, tolerance: f32) -> Polygon;
}

impl ToPolygon for Polygon {
    fn to_polygon(self, _tolerance: f32) -> Polygon {
        self
    }
}

impl ToPolygon for Circle {
    fn to_polygon(self, tolerance: f32) -> Polygon {
        // To approximate the circle, we use a regular polygon for which the
        // circle is the circumscribed circle. The `tolerance` parameter is the
        // maximum allowed distance between the polygon and the circle. This is
        // the same as the difference between the circumscribed circle and the
        // in circle.
        //
        // Let's figure which regular polygon we need to use, by just trying out
        // some of them until we find one whose maximum error is less than or
        // equal to the tolerance.
        let mut n = 3;
        loop {
            let incircle_radius = self.radius() * (PI / n as f32).cos();
            let maximum_error = self.radius() - incircle_radius;

            if maximum_error <= tolerance {
                break;
            }

            n += 1;
        }

        let mut circumference = Vec::new();
        for i in 0..n {
            let p = self.boundary(1.0 / n as f32 * i as f32);
            circumference.push(p);
        }

        let mut polygon = Polygon::new();
        polygon.insert_chain(circumference);

        polygon
    }
}

impl<A, B> ToPolygon for Difference<A, B>
where
    A: ToPolygon,
    B: ToPolygon,
{
    fn to_polygon(self, tolerance: f32) -> Polygon {
        let mut a = self.a.to_polygon(tolerance);
        let mut b = self.b.to_polygon(tolerance);

        b.reverse();
        a.merge(b);

        a
    }
}
