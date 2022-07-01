use fj_math::{Circle, Line, Point, Vector};

use crate::{
    local::Local,
    objects::{Curve, Cycle, CyclesInFace, Edge, Face},
};

/// Reverse the direction of a face
pub fn reverse_face(face: &Face) -> Face {
    let face = match face {
        Face::Face(face) => face,
        Face::Triangles(_) => {
            panic!("Reversing tri-rep faces is not supported")
        }
    };

    let surface = face.surface().reverse();

    let exteriors = reverse_local_coordinates_in_cycle(&face.exteriors);
    let interiors = reverse_local_coordinates_in_cycle(&face.interiors);

    Face::new(surface, exteriors, interiors, face.color)
}

fn reverse_local_coordinates_in_cycle(
    cycles: &CyclesInFace,
) -> impl Iterator<Item = Cycle> + '_ {
    let cycles = cycles.as_local().map(|cycle| {
        let edges = cycle
            .edges
            .iter()
            .map(|edge| {
                let curve = {
                    let local = match *edge.curve.local() {
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

                    let canonical = *edge.curve.global();
                    Local::new(local, canonical)
                };
                let vertices = edge.vertices.clone();

                Edge { curve, vertices }
            })
            .collect();

        Cycle { edges }
    });

    cycles
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::objects::{Face, Surface};

    #[test]
    fn reverse_face() {
        let original = Face::builder(Surface::xy_plane())
            .with_exterior_polygon([[0., 0.], [1., 0.], [0., 1.]])
            .build();

        let reversed = super::reverse_face(&original);

        let expected = Face::builder(Surface::xy_plane().reverse())
            .with_exterior_polygon([[0., 0.], [1., 0.], [0., -1.]])
            .build();

        assert_eq!(expected, reversed);
    }
}
