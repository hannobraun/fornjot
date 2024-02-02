use crate::{objects::Sketch, validate_references};

use super::{
    references::{ReferenceCountError, ReferenceCounter},
    Validate, ValidationConfig, ValidationError,
};

impl Validate for Sketch {
    fn validate_with_config(
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
        operations::{build::BuildHalfEdge, insert::Insert},
        validate::{
            references::ReferenceCountError, SketchValidationError, Validate,
            ValidationError,
        },
        Instance,
    };

    #[test]
    fn should_find_cycle_multiple_references() -> anyhow::Result<()> {
        let mut core = Instance::new();

        let shared_cycle = Cycle::new(vec![]).insert(&mut core.services);

        let invalid_sketch = Sketch::new(vec![
            Region::new(
                Cycle::new(vec![]).insert(&mut core.services),
                vec![shared_cycle.clone()],
                None,
            )
            .insert(&mut core.services),
            Region::new(shared_cycle, vec![], None).insert(&mut core.services),
        ]);
        assert_contains_err!(
            invalid_sketch,
            ValidationError::Sketch(SketchValidationError::MultipleReferences(
                ReferenceCountError::Cycle { references: _ }
            ))
        );

        let valid_sketch = Sketch::new(vec![Region::new(
            Cycle::new(vec![]).insert(&mut core.services),
            vec![],
            None,
        )
        .insert(&mut core.services)]);
        valid_sketch.validate_and_return_first_error()?;

        Ok(())
    }

    #[test]
    fn should_find_half_edge_multiple_references() -> anyhow::Result<()> {
        let mut core = Instance::new();

        let half_edge = HalfEdge::line_segment(
            [[0., 0.], [1., 0.]],
            None,
            &mut core.services,
        )
        .insert(&mut core.services);
        let sibling_edge = HalfEdge::from_sibling(
            &half_edge,
            Vertex::new().insert(&mut core.services),
        )
        .insert(&mut core.services);

        let exterior =
            Cycle::new(vec![half_edge.clone(), sibling_edge.clone()])
                .insert(&mut core.services);

        let interior =
            Cycle::new(vec![half_edge.clone(), sibling_edge.clone()])
                .insert(&mut core.services);

        let invalid_sketch = Sketch::new(vec![Region::new(
            exterior.clone(),
            vec![interior],
            None,
        )
        .insert(&mut core.services)]);
        assert_contains_err!(
            invalid_sketch,
            ValidationError::Sketch(SketchValidationError::MultipleReferences(
                ReferenceCountError::HalfEdge { references: _ }
            ))
        );

        let valid_sketch =
            Sketch::new(vec![
                Region::new(exterior, vec![], None).insert(&mut core.services)
            ]);
        valid_sketch.validate_and_return_first_error()?;

        Ok(())
    }
}
