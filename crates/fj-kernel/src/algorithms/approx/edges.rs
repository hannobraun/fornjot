use crate::{geometry, topology::EdgeVertex};

pub fn approximate_edge(
    vertices: Option<[EdgeVertex; 2]>,
    mut points: Vec<geometry::Point<1>>,
) -> Vec<geometry::Point<1>> {
    // Insert the exact vertices of this edge into the approximation. This means
    // we don't rely on the curve approximation to deliver accurate
    // representations of these vertices, which they might not be able to do.
    //
    // If we used inaccurate representations of those vertices here, then that
    // would lead to bugs in the approximation, as points that should refer to
    // the same vertex would be understood to refer to very close, but distinct
    // vertices.
    let vertices = vertices.map(|vertices| vertices.map(|vertex| vertex.local));
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

    use crate::{
        geometry,
        shape::Shape,
        topology::{EdgeVertex, Vertex},
    };

    #[test]
    fn approximate_edge() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);
        let d = Point::from([5., 8., 13.]);

        let a = geometry::Point::new([0.0], a);
        let b = geometry::Point::new([0.25], b);
        let c = geometry::Point::new([0.75], c);
        let d = geometry::Point::new([1.0], d);

        let v1 = Vertex::builder(&mut shape).build_from_point(a.canonical())?;
        let v2 = Vertex::builder(&mut shape).build_from_point(d.canonical())?;

        let vertices = [
            EdgeVertex {
                handle: v1,
                local: geometry::Point::new([0.], a.canonical()),
            },
            EdgeVertex {
                handle: v2,
                local: geometry::Point::new([1.], d.canonical()),
            },
        ];

        // Regular edge
        assert_eq!(
            super::approximate_edge(Some(vertices), vec![b, c]),
            vec![a, b, c, d],
        );

        // Continuous edge
        assert_eq!(super::approximate_edge(None, vec![b, c]), vec![b, c, b],);

        Ok(())
    }
}
