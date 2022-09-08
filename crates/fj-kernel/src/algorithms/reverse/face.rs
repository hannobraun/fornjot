use fj_math::{Circle, Line, Point, Vector};

use crate::objects::{
    Curve, CurveKind, Cycle, Edge, Face, SurfaceVertex, Vertex,
};

use super::Reverse;

impl Reverse for Face {
    fn reverse(self) -> Self {
        let surface = self.surface().reverse();

        let exteriors = reverse_local_coordinates_in_cycles(self.exteriors());
        let interiors = reverse_local_coordinates_in_cycles(self.interiors());

        Face::new(surface)
            .with_exteriors(exteriors)
            .with_interiors(interiors)
            .with_color(self.color())
    }
}

/// Reverse local coordinates within the cycle, leaving global ones as-is
///
/// # Implementation Note
///
/// This is probably overly complicated. If the orientation of a face were
/// defined by the direction of the half-edges that bound it, we could reverse
/// the whole cycle with no weird distinction. The `Reverse` implementation of
/// `Face` could just use the `Reverse` implementation of `Cycle` then.
///
/// Please note that, as of this writing, half-edges don't really exist as a
/// concept in the kernel. We kind of treat `Edge` as a half-edge, but in an
/// inconsistent way that causes problems. This issue has some context on that:
/// <https://github.com/hannobraun/Fornjot/issues/993>
fn reverse_local_coordinates_in_cycles<'r>(
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

                Curve::new(
                    edge.curve().surface().reverse(),
                    local,
                    *edge.curve().global_form(),
                )
            };

            let vertices = edge.vertices().map(|vertex| {
                let surface_vertex = {
                    let vertex = vertex.surface_form();

                    let position = Point::from([
                        vertex.position().u,
                        -vertex.position().v,
                    ]);

                    SurfaceVertex::new(
                        position,
                        vertex.surface().reverse(),
                        *vertex.global_form(),
                    )
                };

                Vertex::new(
                    vertex.position(),
                    curve,
                    surface_vertex,
                    *vertex.global_form(),
                )
            });

            Edge::from_curve_and_vertices(curve, vertices)
        });

        Cycle::new(surface, edges)
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
