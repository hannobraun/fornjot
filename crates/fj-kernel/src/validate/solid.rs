use std::iter::repeat;

use crate::{
    objects::{Solid, Vertex},
    storage::Handle,
};
use fj_math::Point;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Solid {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        SolidValidationError::check_vertices(self, config, errors)
    }
}

/// [`Solid`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum SolidValidationError {
    /// [`Solid`] contains vertices that are coincident, but not identical
    #[error(
        "Solid contains Vertices that are coinciendent but not identical\n
        Vertex 1: {:#?} {:#?}
        Vertex 2: {:#?} {:#?}
        ", .0[0].0, .0[0].1,.0[1].0,.0[1].1
    )]
    DistinctVertsCoincide([(Handle<Vertex>, Point<3>); 2]),

    /// [`Solid`] contains vertices that are identical, but do not coincide
    #[error(
        "Solid contains Vertices that are identical but do not coincide\n
        Vertex 1: {:#?} {:#?}
        Vertex 2: {:#?} {:#?}
        ", .0[0].0, .0[0].1,.0[1].0,.0[1].1
    )]
    IdenticalVertsNotCoincident([(Handle<Vertex>, Point<3>); 2]),
}

impl SolidValidationError {
    fn check_vertices(
        solid: &Solid,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let vertices: Vec<(Point<3>, Handle<Vertex>)> = solid
            .shells()
            .flat_map(|s| s.faces())
            .flat_map(|face| {
                face.all_cycles()
                    .flat_map(|cycle| cycle.half_edges().cloned())
                    .zip(repeat(face.surface().geometry()))
            })
            .map(|(h, s)| {
                (
                    s.point_from_surface_coords(h.start_position()),
                    h.start_vertex().clone(),
                )
            })
            .collect();

        // This is O(N^2) which isn't great, but we can't use a HashMap since we
        // need to deal with float inaccuracies. Maybe we could use some smarter
        // data-structure like an octree.
        for a in &vertices {
            for b in &vertices {
                match a.1.id() == b.1.id() {
                    true => {
                        if a.0.distance_to(&b.0) > config.identical_max_distance
                        {
                            errors.push(
                                Self::IdenticalVertsNotCoincident([
                                    (a.1.clone(), a.0),
                                    (b.1.clone(), b.0),
                                ])
                                .into(),
                            )
                        }
                    }
                    false => {
                        if a.0.distance_to(&b.0) < config.distinct_min_distance
                        {
                            errors.push(
                                Self::DistinctVertsCoincide([
                                    (a.1.clone(), a.0),
                                    (b.1.clone(), b.0),
                                ])
                                .into(),
                            )
                        }
                    }
                }
            }
        }
    }
}
