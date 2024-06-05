use fj_math::Winding;

use crate::{
    geometry::Geometry,
    storage::Handle,
    topology::{Cycle, HalfEdge, Region, Sketch},
    validation::{
        checks::{AdjacentHalfEdgesNotConnected, MultipleReferencesToObject},
        ValidationCheck,
    },
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Sketch {
    fn validate(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
        geometry: &Geometry,
    ) {
        errors.extend(
            AdjacentHalfEdgesNotConnected::check(self, geometry, config)
                .map(Into::into),
        );
        errors.extend(
            MultipleReferencesToObject::<Cycle, Region>::check(
                self, geometry, config,
            )
            .map(Into::into),
        );
        errors.extend(
            MultipleReferencesToObject::<HalfEdge, Cycle>::check(
                self, geometry, config,
            )
            .map(Into::into),
        );
        SketchValidationError::check_exterior_cycles(
            self, geometry, config, errors,
        );
        SketchValidationError::check_interior_cycles(
            self, geometry, config, errors,
        );
    }
}

/// [`Sketch`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum SketchValidationError {
    /// Region within sketch has exterior cycle with clockwise winding
    #[error(
        "Exterior cycle within sketch region has clockwise winding\n
        Cycle: {cycle:#?}"
    )]
    ClockwiseExteriorCycle {
        /// Cycle with clockwise winding
        cycle: Handle<Cycle>,
    },

    /// Region within sketch has interior cycle with counter-clockwise winding
    #[error(
        "Interior cycle within sketch region has counter-clockwise winding\n
        Cycle: {cycle:#?}"
    )]
    CounterClockwiseInteriorCycle {
        /// Cycle with counter-clockwise winding
        cycle: Handle<Cycle>,
    },
}

impl SketchValidationError {
    fn check_exterior_cycles(
        sketch: &Sketch,
        geometry: &Geometry,
        _config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        sketch.regions().iter().for_each(|region| {
            let cycle = region.exterior();
            if cycle.winding(geometry) == Winding::Cw {
                errors.push(ValidationError::Sketch(
                    SketchValidationError::ClockwiseExteriorCycle {
                        cycle: cycle.clone(),
                    },
                ))
            }
        });
    }

    fn check_interior_cycles(
        sketch: &Sketch,
        geometry: &Geometry,
        _config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        sketch.regions().iter().for_each(|region| {
            region
                .interiors()
                .iter()
                .filter(|interior| interior.winding(geometry) == Winding::Ccw)
                .for_each(|cycle| {
                    errors.push(ValidationError::Sketch(
                        SketchValidationError::CounterClockwiseInteriorCycle {
                            cycle: cycle.clone(),
                        },
                    ));
                })
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
        operations::{
            build::{BuildHalfEdge, BuildSketch},
            insert::Insert,
        },
        topology::{Cycle, HalfEdge, Region, Sketch, Vertex},
        validate::{SketchValidationError, Validate, ValidationError},
        Core,
    };

    #[test]
    fn should_find_clockwise_exterior_cycle() -> anyhow::Result<()> {
        let mut core = Core::new();

        let surface = core.layers.topology.surfaces.space_2d();

        let valid_outer_circle =
            HalfEdge::circle([0., 0.], 1., surface.clone(), &mut core);
        let valid_sketch = Sketch::circle([0., 0.], 1., &mut core);
        valid_sketch.validate_and_return_first_error(&core.layers.geometry)?;

        let invalid_outer_circle = HalfEdge::from_sibling(
            &valid_outer_circle,
            Vertex::new().insert(&mut core),
            &mut core,
        );
        let invalid_exterior =
            Cycle::new(vec![invalid_outer_circle.clone()]).insert(&mut core);
        let invalid_sketch = Sketch::new(
            surface,
            vec![
                Region::new(invalid_exterior.clone(), vec![]).insert(&mut core)
            ],
        );
        assert_contains_err!(
            core,
            invalid_sketch,
            ValidationError::Sketch(
                SketchValidationError::ClockwiseExteriorCycle { cycle: _ }
            )
        );

        Ok(())
    }

    #[test]
    fn should_find_counterclockwise_interior_cycle() -> anyhow::Result<()> {
        let mut core = Core::new();

        let surface = core.layers.topology.surfaces.space_2d();

        let outer_circle =
            HalfEdge::circle([0., 0.], 2., surface.clone(), &mut core);
        let inner_circle =
            HalfEdge::circle([0., 0.], 1., surface.clone(), &mut core);
        let cw_inner_circle = HalfEdge::from_sibling(
            &inner_circle,
            Vertex::new().insert(&mut core),
            &mut core,
        );
        let exterior = Cycle::new(vec![outer_circle.clone()]).insert(&mut core);

        let valid_interior =
            Cycle::new(vec![cw_inner_circle.clone()]).insert(&mut core);
        let valid_sketch = Sketch::new(
            surface.clone(),
            vec![Region::new(exterior.clone(), vec![valid_interior])
                .insert(&mut core)],
        );
        valid_sketch.validate_and_return_first_error(&core.layers.geometry)?;

        let invalid_interior =
            Cycle::new(vec![inner_circle.clone()]).insert(&mut core);
        let invalid_sketch = Sketch::new(
            surface,
            vec![Region::new(exterior.clone(), vec![invalid_interior])
                .insert(&mut core)],
        );
        assert_contains_err!(
            core,
            invalid_sketch,
            ValidationError::Sketch(
                SketchValidationError::CounterClockwiseInteriorCycle {
                    cycle: _
                }
            )
        );
        Ok(())
    }
}
