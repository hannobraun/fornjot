use crate::{
    objects::{HalfEdge, Sketch},
    storage::Handle,
};

use super::{Validate, ValidationConfig, ValidationError};

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
    /// [`Cycle`] referenced by more than one [`Region`]
    #[error("[`Cycle`] referenced by more than one [`Region`]")]
    CycleMultipleReferences,
}

impl SketchValidationError {
    fn check_obect_references(
        sketch: &Sketch,
        _config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let mut referenced_edges =
            std::collections::HashMap::<Handle<HalfEdge>, i32>::new();

        // Do we care about how many times each edge is used, or should we just return as soon as
        // we find one that is used more than once?
        sketch.regions().iter().for_each(|r| {
            r.all_cycles().for_each(|c| {
                c.half_edges().into_iter().for_each(|e| {
                    referenced_edges.insert(e.clone(), {
                        if let Some(count) = referenced_edges.get(e) {
                            count + 1
                        } else {
                            1
                        }
                    });
                })
            })
        });

        referenced_edges.iter().for_each(|(_, count)| {
            if count > &1 {
                errors.push(Self::HalfEdgeMultipleReferences.into());
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
        objects::{Cycle, HalfEdge, Region, Sketch},
        operations::build::BuildHalfEdge,
        services::Services,
        storage::Store,
        validate::{SketchValidationError, Validate, ValidationError},
    };

    #[test]
    fn should_find_half_edge_multiple_references() -> anyhow::Result<()> {
        let mut services = Services::new();

        let mut cycle_store: Store<Cycle> = Store::new();
        let mut region_store: Store<Region> = Store::new();
        let mut half_edge_store: Store<HalfEdge> = Store::new();

        let half_edge = half_edge_store.reserve();
        half_edge_store.insert(
            half_edge.clone(),
            HalfEdge::line_segment([[0., 0.], [1., 0.]], None, &mut services),
        );

        let exterior = cycle_store.reserve();
        cycle_store
            .insert(exterior.clone(), Cycle::new(vec![half_edge.clone()]));
        let interior = cycle_store.reserve();
        cycle_store
            .insert(interior.clone(), Cycle::new(vec![half_edge.clone()]));

        let region = region_store.reserve();
        region_store.insert(
            region.clone(),
            Region::new(exterior, vec![interior], None),
        );

        let sketch = Sketch::new(vec![region]);

        assert_contains_err!(
            sketch,
            ValidationError::Sketch(
                SketchValidationError::HalfEdgeMultipleReferences
            )
        );

        Ok(())
    }
}
