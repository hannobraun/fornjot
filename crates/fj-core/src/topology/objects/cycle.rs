use fj_math::{Scalar, Winding};

use crate::{
    geometry::{Geometry, Path},
    storage::Handle,
    topology::{HalfEdge, ObjectSet},
};

use super::surface::Surface;

/// A cycle of connected edges
#[derive(Clone, Debug)]
pub struct Cycle {
    half_edges: ObjectSet<HalfEdge>,
}

impl Cycle {
    /// Create an instance of `Cycle`
    pub fn new(half_edges: impl IntoIterator<Item = Handle<HalfEdge>>) -> Self {
        let half_edges = half_edges.into_iter().collect();
        Self { half_edges }
    }

    /// Access the edges that make up the cycle
    pub fn half_edges(&self) -> &ObjectSet<HalfEdge> {
        &self.half_edges
    }

    /// Indicate the cycle's winding, assuming a right-handed coordinate system
    ///
    /// Please note that this is not *the* winding of the cycle, only one of the
    /// two possible windings, depending on the direction you look at the
    /// surface that the cycle is defined on from.
    pub fn winding(
        &self,
        geometry: &Geometry,
        surface: &Handle<Surface>,
    ) -> Winding {
        // The cycle could be made up of one or two circles. If that is the
        // case, the winding of the cycle is determined by the winding of the
        // first circle.
        if self.half_edges.len() < 3 {
            let first = self
                .half_edges()
                .iter()
                .next()
                .expect("Invalid cycle: expected at least one edge");

            let curve_geom = geometry
                .of_curve(first.curve())
                .unwrap()
                .local_on(surface)
                .unwrap()
                .clone();

            let [a, b] = [
                curve_geom.path.point_from_path_coords(
                    geometry
                        .of_vertex(first.start_vertex())
                        .unwrap()
                        .local_on(first.curve())
                        .unwrap()
                        .position,
                ),
                curve_geom.path.point_from_path_coords(
                    geometry
                        .of_vertex(
                            self.half_edges()
                                .after(first)
                                .expect("Just got half-edge from this cycle")
                                .start_vertex(),
                        )
                        .unwrap()
                        .local_on(first.curve())
                        .unwrap()
                        .position,
                ),
            ];
            let edge_direction_positive = a < b;

            let circle = match curve_geom.path {
                Path::Circle(circle) => circle,
                Path::Line(_) => unreachable!(
                    "Invalid cycle: less than 3 edges, but not all are circles"
                ),
            };
            let cross_positive = circle.a().cross2d(&circle.b()) > Scalar::ZERO;

            if edge_direction_positive == cross_positive {
                return Winding::Ccw;
            } else {
                return Winding::Cw;
            }
        }

        // Now that we got the special case out of the way, we can treat the
        // cycle as a polygon:
        // https://stackoverflow.com/a/1165943

        let mut sum = Scalar::ZERO;

        for (a, b) in self.half_edges().pairs() {
            let [a, b] = [a, b].map(|half_edge| {
                geometry
                    .of_curve(half_edge.curve())
                    .unwrap()
                    .local_on(surface)
                    .unwrap()
                    .path
                    .point_from_path_coords(
                        geometry
                            .of_vertex(half_edge.start_vertex())
                            .unwrap()
                            .local_on(half_edge.curve())
                            .unwrap()
                            .position,
                    )
            });

            sum += (b.u - a.u) * (b.v + a.v);
        }

        if sum > Scalar::ZERO {
            return Winding::Cw;
        }
        if sum < Scalar::ZERO {
            return Winding::Ccw;
        }

        unreachable!("Encountered invalid cycle: {self:#?}");
    }
}
