use fj_math::Point;

use crate::objects::VerticesOfEdge;

use super::Local;

pub fn approx_edge(
    vertices: VerticesOfEdge,
    points: &mut Vec<Local<Point<1>>>,
) {
    // Insert the exact vertices of this edge into the approximation. This means
    // we don't rely on the curve approximation to deliver accurate
    // representations of these vertices, which they might not be able to do.
    //
    // If we used inaccurate representations of those vertices here, then that
    // would lead to bugs in the approximation, as points that should refer to
    // the same vertex would be understood to refer to very close, but distinct
    // vertices.
    let vertices = vertices.convert(|vertex| {
        Local::new(vertex.position(), vertex.global().position())
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
}

#[cfg(test)]
mod test {
    use fj_math::Point;

    use crate::{
        algorithms::approx::Local,
        objects::{GlobalVertex, Vertex, VerticesOfEdge},
    };

    #[test]
    fn approx_edge() {
        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);
        let d = Point::from([5., 8., 13.]);

        let v1 = GlobalVertex::from_position(a);
        let v2 = GlobalVertex::from_position(d);

        let vertices = VerticesOfEdge::from_vertices([
            Vertex::new(Point::from([0.]), v1),
            Vertex::new(Point::from([1.]), v2),
        ]);

        let a = Local::new([0.0], a);
        let b = Local::new([0.25], b);
        let c = Local::new([0.75], c);
        let d = Local::new([1.0], d);

        // Regular edge
        let mut points = vec![b, c];
        super::approx_edge(vertices, &mut points);
        assert_eq!(points, vec![a, b, c, d]);

        // Continuous edge
        let mut points = vec![b, c];
        super::approx_edge(VerticesOfEdge::none(), &mut points);
        assert_eq!(points, vec![b, c, b]);
    }
}
