use crate::objects::Sketch;

use super::{
    references::{ReferenceCounter, ValidateReferences},
    Validate, ValidationConfig, ValidationError,
};

impl Validate for Sketch {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        SketchValidationError::check_obect_references(self, config, errors);
    }
}

/// [`Sketch`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum SketchValidationError {
    /// [`HalfEdge`] referenced by more than one [`Cycle`]
    #[error("[`HalfEdge`] referenced by more than one [`Cycle`]")]
    HalfEdgeMultipleReferences,
    /// [`Cycle`] referenced by more than one [`crate::objects::Region`]
    #[error("[`Cycle`] referenced by more than one [`Region`]")]
    CycleMultipleReferences,
}

impl SketchValidationError {
    fn check_obect_references(
        sketch: &Sketch,
        _config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        // todo: store referencing objects instead of just a reference count so that we can surface
        // them in the error message
        let mut referenced_edges = ReferenceCounter::new();
        let mut referenced_cycles = ReferenceCounter::new();

        sketch.regions().iter().for_each(|r| {
            r.all_cycles().for_each(|c| {
                referenced_cycles.add_count(c.clone());
                c.half_edges().into_iter().for_each(|e| {
                    referenced_edges.add_count(e.clone());
                })
            })
        });

        referenced_cycles.validate(errors);
        referenced_edges.validate(errors);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
        objects::{Cycle, HalfEdge, Region, Sketch, Vertex},
        operations::{build::BuildHalfEdge, insert::Insert},
        services::Services,
        validate::{
            references::ReferenceCountError, Validate, ValidationError,
        },
    };

    #[test]
    fn should_find_cycle_multiple_references() -> anyhow::Result<()> {
        let mut services = Services::new();

        let shared_cycle = Cycle::new(vec![]).insert(&mut services);

        let invalid_sketch = Sketch::new(vec![
            Region::new(
                Cycle::new(vec![]).insert(&mut services),
                vec![shared_cycle.clone()],
                None,
            )
            .insert(&mut services),
            Region::new(shared_cycle, vec![], None).insert(&mut services),
        ]);
        assert_contains_err!(
            invalid_sketch,
            ValidationError::ReferenceCount(ReferenceCountError::Cycle)
        );

        let valid_sketch = Sketch::new(vec![Region::new(
            Cycle::new(vec![]).insert(&mut services),
            vec![],
            None,
        )
        .insert(&mut services)]);
        valid_sketch.validate_and_return_first_error()?;

        Ok(())
    }

    #[test]
    fn should_find_half_edge_multiple_references() -> anyhow::Result<()> {
        let mut services = Services::new();

        let half_edge =
            HalfEdge::line_segment([[0., 0.], [1., 0.]], None, &mut services)
                .insert(&mut services);
        let sibling_edge = HalfEdge::from_sibling(
            &half_edge,
            Vertex::new().insert(&mut services),
        )
        .insert(&mut services);

        let exterior =
            Cycle::new(vec![half_edge.clone(), sibling_edge.clone()])
                .insert(&mut services);

        let interior =
            Cycle::new(vec![half_edge.clone(), sibling_edge.clone()])
                .insert(&mut services);

        let invalid_sketch = Sketch::new(vec![Region::new(
            exterior.clone(),
            vec![interior],
            None,
        )
        .insert(&mut services)]);
        assert_contains_err!(
            invalid_sketch,
            ValidationError::ReferenceCount(ReferenceCountError::HalfEdge)
        );

        let valid_sketch =
            Sketch::new(vec![
                Region::new(exterior, vec![], None).insert(&mut services)
            ]);
        valid_sketch.validate_and_return_first_error()?;

        Ok(())
    }
}
