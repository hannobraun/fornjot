use fj_math::{Circle, Line, Point, Vector};

use crate::{
    local::Local,
    objects::{Curve, Cycle, Edge, Face},
};

/// Reverse the direction of a face
pub fn reverse_face(face: &Face) -> Face {
    if face.triangles().is_some() {
        panic!("Reversing tri-rep faces is not supported");
    }

    let surface = face.surface().reverse();

    let exteriors = reverse_local_coordinates_in_cycle(face.exteriors());
    let interiors = reverse_local_coordinates_in_cycle(face.interiors());

    Face::new(surface)
        .with_exteriors(exteriors)
        .with_interiors(interiors)
        .with_color(face.color())
}

fn reverse_local_coordinates_in_cycle<'r>(
    cycles: impl IntoIterator<Item = &'r Cycle> + 'r,
) -> impl Iterator<Item = Cycle> + 'r {
    cycles.into_iter().map(|cycle| {
        let edges = cycle.edges().map(|edge| {
            let curve = {
                let local = match edge.curve().local_form() {
                    Curve::Circle(Circle { center, a, b }) => {
                        let center = Point::from([center.u, -center.v]);

                        let a = Vector::from([a.u, -a.v]);
                        let b = Vector::from([b.u, -b.v]);

                        Curve::Circle(Circle { center, a, b })
                    }
                    Curve::Line(Line { origin, direction }) => {
                        let origin = Point::from([origin.u, -origin.v]);
                        let direction =
                            Vector::from([direction.u, -direction.v]);

                        Curve::Line(Line { origin, direction })
                    }
                };

                Local::new(local, *edge.curve().global_form())
            };

            Edge::new(curve, *edge.vertices())
        });

        Cycle::new().with_edges(edges)
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::objects::{Cycle, Face, Surface};

    #[test]
    fn reverse_face() {
        let surface = Surface::xy_plane();
        let original =
            Face::new(surface).with_exteriors([Cycle::polygon_from_points(
                &surface,
                [[0., 0.], [1., 0.], [0., 1.]],
            )]);

        let reversed = super::reverse_face(&original);

        let surface = Surface::xy_plane().reverse();
        let expected =
            Face::new(surface).with_exteriors([Cycle::polygon_from_points(
                &surface,
                [[0., 0.], [1., 0.], [0., -1.]],
            )]);

        assert_eq!(expected, reversed);
    }
}
