use crate::{objects::Sketch, validate_references};
use fj_math::Winding;

use super::{
    references::{ReferenceCountError, ReferenceCounter},
    Validate, ValidationConfig, ValidationError,
};

impl Validate for Sketch {
    fn validate(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        SketchValidationError::check_object_references(self, config, errors);
        SketchValidationError::check_exterior_cycle(self, config, errors);
        SketchValidationError::check_interior_cycles(self, config, errors);
    }
}

/// [`Sketch`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum SketchValidationError {
    /// Object within sketch referenced by more than one other object
    #[error("Object within sketch referenced by more than one other Object")]
    MultipleReferences(#[from] ReferenceCountError),
    /// Region within sketch has exterior cycle with clockwise winding
    #[error("Exterior cycle within sketch region has clockwise winding")]
    ClockwiseExteriorCycle(),
    /// Region within sketch has interior cycle with counter-clockwise winding
    #[error(
        "Interior cycle within sketch region has counter-clockwise winding"
    )]
    CounterClockwiseInteriorCycle(),
}

impl SketchValidationError {
    fn check_object_references(
        sketch: &Sketch,
        _config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let mut referenced_edges = ReferenceCounter::new();
        let mut referenced_cycles = ReferenceCounter::new();

        sketch.regions().iter().for_each(|r| {
            r.all_cycles().for_each(|c| {
                referenced_cycles.add_reference(c.clone(), r.clone());
                c.half_edges().into_iter().for_each(|e| {
                    referenced_edges.add_reference(e.clone(), c.clone());
                })
            })
        });

        validate_references!(
            errors, SketchValidationError;
            referenced_edges, HalfEdge;
            referenced_cycles, Cycle;
        );
    }

    fn check_exterior_cycle(
        sketch: &Sketch,
        _config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        sketch.regions().iter().for_each(|region| {
            if region.exterior().winding() == Winding::Cw {
                errors.push(ValidationError::Sketch(
                    SketchValidationError::ClockwiseExteriorCycle(),
                ))
            }
        });
    }

    fn check_interior_cycles(
        sketch: &Sketch,
        _config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        sketch.regions().iter().for_each(|region| {
            region
                .interiors()
                .iter()
                .filter(|interior| interior.winding() == Winding::Ccw)
                .for_each(|_interior| {
                    errors.push(ValidationError::Sketch(
                        SketchValidationError::CounterClockwiseInteriorCycle(),
                    ));
                })
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
        objects::{Cycle, HalfEdge, Region, Sketch, Vertex},
        operations::{
            build::BuildHalfEdge, build::BuildRegion, insert::Insert,
        },
        validate::{
            references::ReferenceCountError, SketchValidationError, Validate,
            ValidationError,
        },
        Core,
    };

    #[test]
    fn should_find_cycle_multiple_references() -> anyhow::Result<()> {
        let mut core = Core::new();

        let region = <Region as BuildRegion>::circle([0., 0.], 1., &mut core)
            .insert(&mut core);
        let valid_sketch = Sketch::new(vec![region.clone()]).insert(&mut core);
        valid_sketch.validate_and_return_first_error()?;

        let shared_cycle = region.exterior();
        let invalid_sketch = Sketch::new(vec![
            Region::new(shared_cycle.clone(), vec![]).insert(&mut core),
            Region::new(shared_cycle.clone(), vec![]).insert(&mut core),
        ]);
        assert_contains_err!(
            invalid_sketch,
            ValidationError::Sketch(SketchValidationError::MultipleReferences(
                ReferenceCountError::Cycle { references: _ }
            ))
        );

        Ok(())
    }

    #[test]
    fn should_find_half_edge_multiple_references() -> anyhow::Result<()> {
        let mut core = Core::new();

        let region = <Region as BuildRegion>::polygon(
            [[0., 0.], [1., 1.], [0., 1.]],
            &mut core,
        )
        .insert(&mut core);
        let valid_sketch = Sketch::new(vec![region.clone()]).insert(&mut core);
        valid_sketch.validate_and_return_first_error()?;

        let exterior = region.exterior();
        let cloned_edges: Vec<_> =
            exterior.half_edges().iter().map(|e| e.clone()).collect();
        let interior = Cycle::new(cloned_edges).insert(&mut core);

        let invalid_sketch =
            Sketch::new(vec![
                Region::new(exterior.clone(), vec![interior]).insert(&mut core)
            ]);
        assert_contains_err!(
            invalid_sketch,
            ValidationError::Sketch(SketchValidationError::MultipleReferences(
                ReferenceCountError::HalfEdge { references: _ }
            ))
        );

        Ok(())
    }

    #[test]
    fn should_find_clockwise_exterior_cycle() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid_outer_circle =
            HalfEdge::circle([0., 0.], 1., &mut core).insert(&mut core);
        let valid_exterior =
            Cycle::new(vec![valid_outer_circle.clone()]).insert(&mut core);
        let valid_sketch =
            Sketch::new(vec![
                Region::new(valid_exterior.clone(), vec![]).insert(&mut core)
            ]);
        valid_sketch.validate_and_return_first_error()?;

        let invalid_outer_circle = HalfEdge::from_sibling(
            &valid_outer_circle,
            Vertex::new().insert(&mut core),
        )
        .insert(&mut core);
        let invalid_exterior =
            Cycle::new(vec![invalid_outer_circle.clone()]).insert(&mut core);
        let invalid_sketch =
            Sketch::new(vec![
                Region::new(invalid_exterior.clone(), vec![]).insert(&mut core)
            ]);
        assert_contains_err!(
            invalid_sketch,
            ValidationError::Sketch(
                SketchValidationError::ClockwiseExteriorCycle()
            )
        );

        Ok(())
    }

    #[test]
    fn should_find_counterclockwise_interior_cycle() -> anyhow::Result<()> {
        let mut core = Core::new();

        let outer_circle =
            HalfEdge::circle([0., 0.], 2., &mut core).insert(&mut core);
        let inner_circle =
            HalfEdge::circle([0., 0.], 1., &mut core).insert(&mut core);
        let cw_inner_circle = HalfEdge::from_sibling(
            &inner_circle,
            Vertex::new().insert(&mut core),
        )
        .insert(&mut core);
        let exterior = Cycle::new(vec![outer_circle.clone()]).insert(&mut core);

        let valid_interior =
            Cycle::new(vec![cw_inner_circle.clone()]).insert(&mut core);
        let valid_sketch = Sketch::new(vec![Region::new(
            exterior.clone(),
            vec![valid_interior],
        )
        .insert(&mut core)]);
        valid_sketch.validate_and_return_first_error()?;

        let invalid_interior =
            Cycle::new(vec![inner_circle.clone()]).insert(&mut core);
        let invalid_sketch = Sketch::new(vec![Region::new(
            exterior.clone(),
            vec![invalid_interior],
        )
        .insert(&mut core)]);
        assert_contains_err!(
            invalid_sketch,
            ValidationError::Sketch(
                SketchValidationError::CounterClockwiseInteriorCycle()
            )
        );
        Ok(())
    }
}
