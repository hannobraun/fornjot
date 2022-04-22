use fj_math::Point;

use crate::topology::Vertex;

pub fn approximate_edge(
    mut points: Vec<Point<3>>,
    vertices: Option<[Vertex; 2]>,
) -> Vec<Point<3>> {
    // Insert the exact vertices of this edge into the approximation. This means
    // we don't rely on the curve approximation to deliver accurate
    // representations of these vertices, which they might not be able to do.
    //
    // If we used inaccurate representations of those vertices here, then that
    // would lead to bugs in the approximation, as points that should refer to
    // the same vertex would be understood to refer to very close, but distinct
    // vertices.
    if let Some([a, b]) = &vertices {
        points.insert(0, a.point());
        points.push(b.point());
    }

    if vertices.is_none() {
        // The edge has no vertices, which means it connects to itself. We need
        // to reflect that in the approximation.

        if let Some(&point) = points.first() {
            points.push(point);
        }
    }

    points
}

#[cfg(test)]
mod test {
    use fj_math::Point;

    use crate::{shape::Shape, topology::Vertex};

    #[test]
    fn approximate_edge() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);
        let d = Point::from([5., 8., 13.]);

        let v1 = Vertex::builder(&mut shape).build_from_point(a)?;
        let v2 = Vertex::builder(&mut shape).build_from_point(d)?;

        // Regular edge
        assert_eq!(
            super::approximate_edge(vec![b, c], Some([v1.get(), v2.get()])),
            vec![a, b, c, d],
        );

        // Continuous edge
        assert_eq!(super::approximate_edge(vec![b, c], None), vec![b, c, b],);

        Ok(())
    }
}
