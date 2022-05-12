use crate::{geometry, topology::Vertex};

pub fn approximate_edge(
    mut points: Vec<geometry::Point<3>>,
    vertices: Option<[Vertex; 2]>,
) -> Vec<geometry::Point<3>> {
    // Insert the exact vertices of this edge into the approximation. This means
    // we don't rely on the curve approximation to deliver accurate
    // representations of these vertices, which they might not be able to do.
    //
    // If we used inaccurate representations of those vertices here, then that
    // would lead to bugs in the approximation, as points that should refer to
    // the same vertex would be understood to refer to very close, but distinct
    // vertices.
    let vertices = vertices.map(|vertices| {
        vertices.map(|vertex| {
            let point = vertex.point();
            geometry::Point::new(point, point)
        })
    });
    if let Some([a, b]) = vertices {
        points.insert(0, a);
        points.push(b);
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

    use crate::{geometry, shape::Shape, topology::Vertex};

    #[test]
    fn approximate_edge() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);
        let d = Point::from([5., 8., 13.]);

        let a = geometry::Point::new(a, a);
        let b = geometry::Point::new(b, b);
        let c = geometry::Point::new(c, c);
        let d = geometry::Point::new(d, d);

        let v1 = Vertex::builder(&mut shape).build_from_point(a.canonical())?;
        let v2 = Vertex::builder(&mut shape).build_from_point(d.canonical())?;

        let vertices = [v1.get(), v2.get()];

        // Regular edge
        assert_eq!(
            super::approximate_edge(vec![b, c], Some(vertices)),
            vec![a, b, c, d],
        );

        // Continuous edge
        assert_eq!(super::approximate_edge(vec![b, c], None), vec![b, c, b],);

        Ok(())
    }
}
