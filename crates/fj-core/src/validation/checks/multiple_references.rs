use std::{any::type_name_of_val, collections::HashMap, fmt};

use crate::{
    storage::Handle,
    topology::{Cycle, HalfEdge, Region, Sketch},
    validation::ValidationCheck,
};

/// Object that should be exclusively owned by another, is not
///
/// Some objects are expected to be "owned" by a single other object. This means
/// that only one reference to these objects must exist within the topological
/// object graph.
#[derive(Clone, Debug, thiserror::Error)]
pub struct MultipleReferencesToObject<T, U> {
    object: Handle<T>,
    referenced_by: Vec<Handle<U>>,
}

impl<T, U> fmt::Display for MultipleReferencesToObject<T, U>
where
    T: fmt::Debug,
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "`{}` ({:?}) referenced by multiple `{}` objects ({:?})",
            type_name_of_val(&self.object),
            self.object,
            type_name_of_val(&self.referenced_by),
            self.referenced_by
        )
    }
}

impl ValidationCheck<Sketch> for MultipleReferencesToObject<Cycle, Region> {
    fn check<'r>(
        object: &'r Sketch,
        _: &'r crate::geometry::Geometry,
        _: &'r crate::validation::ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        let mut cycles = ReferenceCounter::new();

        for region in object.regions() {
            for cycle in region.all_cycles() {
                cycles.count(cycle.clone(), region.clone());
            }
        }

        cycles.multiples()
    }
}

impl ValidationCheck<Sketch> for MultipleReferencesToObject<HalfEdge, Cycle> {
    fn check<'r>(
        object: &'r Sketch,
        _: &'r crate::geometry::Geometry,
        _: &'r crate::validation::ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        let mut half_edges = ReferenceCounter::new();

        for region in object.regions() {
            for cycle in region.all_cycles() {
                for half_edge in cycle.half_edges() {
                    half_edges.count(half_edge.clone(), cycle.clone());
                }
            }
        }

        half_edges.multiples()
    }
}

// Warnings are temporarily silenced, until this struct can be made private.
// This can happen once this validation check has been fully ported from the old
// infrastructure.
#[allow(missing_docs)]
#[derive(Default)]
pub struct ReferenceCounter<T, U>(HashMap<Handle<T>, Vec<Handle<U>>>);

// Warnings are temporarily silenced, until this struct can be made private.
// This can happen once this validation check has been fully ported from the old
// infrastructure.
#[allow(missing_docs)]
impl<T, U> ReferenceCounter<T, U> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn count(&mut self, to: Handle<T>, from: Handle<U>) {
        self.0.entry(to).or_default().push(from);
    }

    pub fn multiples(
        self,
    ) -> impl Iterator<Item = MultipleReferencesToObject<T, U>> {
        self.0
            .into_iter()
            .filter(|(_, referenced_by)| referenced_by.len() > 1)
            .map(|(object, referenced_by)| MultipleReferencesToObject {
                object: object.clone(),
                referenced_by: referenced_by.to_vec(),
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
        operations::{build::BuildRegion, insert::Insert},
        topology::{Cycle, Region, Sketch},
        validate::Validate,
        validation::ValidationError,
        Core,
    };

    #[test]
    fn should_find_cycle_multiple_references() -> anyhow::Result<()> {
        let mut core = Core::new();

        let surface = core.layers.topology.surfaces.space_2d();

        let region = <Region as BuildRegion>::circle(
            [0., 0.],
            1.,
            surface.clone(),
            &mut core,
        )
        .insert(&mut core);
        let valid = Sketch::new(surface.clone(), vec![region.clone()]);
        valid.validate_and_return_first_error(&core.layers.geometry)?;

        let shared_cycle = region.exterior();
        let invalid = Sketch::new(
            surface,
            vec![
                Region::new(shared_cycle.clone(), vec![]).insert(&mut core),
                Region::new(shared_cycle.clone(), vec![]).insert(&mut core),
            ],
        );
        assert_contains_err!(
            core,
            invalid,
            ValidationError::MultipleReferencesToCycle(_)
        );

        Ok(())
    }

    #[test]
    fn should_find_half_edge_multiple_references() -> anyhow::Result<()> {
        let mut core = Core::new();

        let surface = core.layers.topology.surfaces.space_2d();

        let region = <Region as BuildRegion>::polygon(
            [[0., 0.], [1., 1.], [0., 1.]],
            surface.clone(),
            &mut core,
        )
        .insert(&mut core);
        let valid_sketch = Sketch::new(surface.clone(), vec![region.clone()])
            .insert(&mut core);
        valid_sketch.validate_and_return_first_error(&core.layers.geometry)?;

        let exterior = region.exterior();
        let cloned_edges: Vec<_> =
            exterior.half_edges().iter().cloned().collect();
        let interior = Cycle::new(cloned_edges).insert(&mut core);

        let invalid_sketch = Sketch::new(
            surface,
            vec![
                Region::new(exterior.clone(), vec![interior]).insert(&mut core)
            ],
        );
        assert_contains_err!(
            core,
            invalid_sketch,
            ValidationError::MultipleReferencesToHalfEdge(_)
        );

        Ok(())
    }
}
