use fj_interop::ext::ArrayExt;
use itertools::Itertools;

use crate::{
    geometry::curve::Curve,
    objects::Cycle,
    partial::{MaybeCurve, Partial, PartialFace},
};

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Add an interior cycle
    fn add_interior(&mut self) -> Partial<Cycle>;

    /// Infer any undefined curves in the face
    fn infer_curves(&mut self);
}

impl FaceBuilder for PartialFace {
    fn add_interior(&mut self) -> Partial<Cycle> {
        let cycle = Partial::new();
        self.interiors.push(cycle.clone());
        cycle
    }

    fn infer_curves(&mut self) {
        for (mut half_edge, next_half_edge) in self
            .exterior
            .read()
            .half_edges
            .iter()
            .cloned()
            .circular_tuple_windows()
        {
            let mut half_edge = half_edge.write();

            let mut curve = half_edge.curve;

            if let Some(path) = &mut curve {
                match path {
                    MaybeCurve::Defined(_) => {
                        // Path is already defined. Nothing to infer.
                    }
                    MaybeCurve::UndefinedCircle { .. } => todo!(
                        "Inferring undefined circles is not supported yet"
                    ),
                    MaybeCurve::UndefinedLine => {
                        let points_surface = [
                            &half_edge.start_vertex,
                            &next_half_edge.read().start_vertex,
                        ]
                        .map(|vertex| {
                            vertex.read().position.expect(
                                "Can't infer curve without surface points",
                            )
                        });
                        let (line, points_curve) =
                            Curve::line_from_points(points_surface);

                        *path = MaybeCurve::Defined(line);
                        for (vertex, point) in half_edge
                            .boundary
                            .each_mut_ext()
                            .zip_ext(points_curve)
                        {
                            *vertex = Some(point);
                        }
                    }
                }
            }
        }
    }
}
