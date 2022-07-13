use fj_math::{Scalar, Segment};
use parry2d_f64::query::{Ray, RayCast};

use crate::objects::{Curve, Face};

/// The intersections between a [`Curve`] and a [`Face`], in curve coordinates
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveFaceIntersections(Vec<[Scalar; 2]>);

/// Determine the intersection between a [`Curve`] and a [`Face`]
///
/// Returns a list of intersections in curve coordinates.
pub fn curve_face(curve: &Curve<2>, face: &Face) -> CurveFaceIntersections {
    let line = match curve {
        Curve::Line(line) => line,
        _ => todo!("Curve-face intersection only supports lines"),
    };

    let face_as_polygon = face
        .exteriors()
        .chain(face.interiors())
        .flat_map(|cycle| {
            let edges: Vec<_> = cycle.edges().collect();
            edges
        })
        .map(|edge| {
            let line = match edge.curve.local() {
                Curve::Line(line) => line,
                _ => todo!("Curve-face intersection only supports polygons"),
            };

            let vertices = match edge.vertices() {
                Some(vertices) => vertices,
                None => todo!(
                    "Curve-face intersection does not support faces with \
                    continuous edges"
                ),
            };

            (line, vertices)
        });

    let mut intersections = Vec::new();

    for (edge_line, vertices) in face_as_polygon {
        let vertices = vertices
            .map(|vertex| edge_line.point_from_line_coords(vertex.position()));
        let segment = Segment::from_points(vertices);

        let ray = Ray {
            origin: line.origin.to_na(),
            dir: line.direction.to_na(),
        };
        let ray_inv = Ray {
            origin: line.origin.to_na(),
            dir: -line.direction.to_na(),
        };

        let result =
            segment
                .to_parry()
                .cast_local_ray(&ray, f64::INFINITY, false);
        let result_inv =
            segment
                .to_parry()
                .cast_local_ray(&ray_inv, f64::INFINITY, false);

        if let Some(result) = result {
            intersections.push(Scalar::from(result));
        }
        if let Some(result_inv) = result_inv {
            intersections.push(-Scalar::from(result_inv));
        }
    }

    assert!(intersections.len() % 2 == 0);

    intersections.sort();

    // Can be cleaned up, once `array_chunks` is stable:
    // https://doc.rust-lang.org/std/primitive.slice.html#method.array_chunks
    let intersections = intersections
        .chunks(2)
        .map(|chunk| {
            // Can't panic, as we passed `2` to `windows`.
            [chunk[0], chunk[1]]
        })
        .collect();

    CurveFaceIntersections(intersections)
}

#[cfg(test)]
mod tests {
    use fj_math::{Line, Point, Scalar, Vector};

    use crate::objects::{Curve, Face, Surface};

    use super::CurveFaceIntersections;

    #[test]
    fn curve_face() {
        let curve = Curve::Line(Line {
            origin: Point::from([-3., 0.]),
            direction: Vector::from([1., 0.]),
        });

        #[rustfmt::skip]
        let exterior = [
            [-2., -2.],
            [ 2., -2.],
            [ 2.,  2.],
            [-2.,  2.],
        ];
        #[rustfmt::skip]
        let interior = [
            [-1., -1.],
            [ 1., -1.],
            [ 1.,  1.],
            [-1.,  1.],
        ];

        let face = Face::builder(Surface::xy_plane())
            .with_exterior_polygon(exterior)
            .with_interior_polygon(interior)
            .build();

        let expected: Vec<_> = [[1., 2.], [4., 5.]]
            .into_iter()
            .map(|interval: [f64; 2]| interval.map(Scalar::from))
            .collect();
        let expected = CurveFaceIntersections(expected);
        assert_eq!(super::curve_face(&curve, &face), expected);
    }
}
