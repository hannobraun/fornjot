use crate::{objects::Sketch, validate_references};

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
    }
}

/// [`Sketch`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum SketchValidationError {
    /// Object within sketch referenced by more than one other object
    #[error("Object within sketch referenced by more than one other Object")]
    MultipleReferences(#[from] ReferenceCountError),
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
}
