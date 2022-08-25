//! Reverse the direction/orientation of objects

use fj_math::{Circle, Line, Point, Vector};

use crate::objects::{Curve, CurveKind, Cycle, Edge, Face};

/// Reverse the direction/orientation of an object
pub trait Reverse {
    /// Reverse the direction/orientation of the object
    #[must_use]
    fn reverse(self) -> Self;
}

impl Reverse for Face {
    fn reverse(self) -> Self {
        if self.triangles().is_some() {
            panic!("Reversing tri-rep faces is not supported");
        }

        let surface = self.surface().reverse();

        let exteriors = reverse_local_coordinates_in_cycle(self.exteriors());
        let interiors = reverse_local_coordinates_in_cycle(self.interiors());

        Face::new(surface)
            .with_exteriors(exteriors)
            .with_interiors(interiors)
            .with_color(self.color())
    }
}

fn reverse_local_coordinates_in_cycle<'r>(
    cycles: impl IntoIterator<Item = &'r Cycle> + 'r,
) -> impl Iterator<Item = Cycle> + 'r {
    cycles.into_iter().map(|cycle| {
        let surface = cycle.surface().reverse();

        let edges = cycle.edges().map(|edge| {
            let curve = {
                let local = match edge.curve().kind() {
                    CurveKind::Circle(circle) => {
                        let center = Point::from([
                            circle.center().u,
                            -circle.center().v,
                        ]);

                        let a = Vector::from([circle.a().u, -circle.a().v]);
                        let b = Vector::from([circle.b().u, -circle.b().v]);

                        CurveKind::Circle(Circle::new(center, a, b))
                    }
                    CurveKind::Line(line) => {
                        let origin =
                            Point::from([line.origin().u, -line.origin().v]);
                        let direction = Vector::from([
                            line.direction().u,
                            -line.direction().v,
                        ]);

                        CurveKind::Line(Line::from_origin_and_direction(
                            origin, direction,
                        ))
                    }
                };

                Curve::new(local, *edge.curve().global())
            };

            Edge::from_curve_and_vertices(curve, *edge.vertices())
        });

        Cycle::new(surface).with_edges(edges)
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::reverse::Reverse,
        objects::{Face, Surface},
    };

    #[test]
    fn reverse_face() {
        let surface = Surface::xy_plane();
        let original = Face::build(surface)
            .polygon_from_points([[0., 0.], [1., 0.], [0., 1.]])
            .into_face();

        let reversed = original.reverse();

        let surface = Surface::xy_plane().reverse();
        let expected = Face::build(surface)
            .polygon_from_points([[0., 0.], [1., 0.], [0., -1.]])
            .into_face();

        assert_eq!(expected, reversed);
    }
}
