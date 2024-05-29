use std::{any::type_name_of_val, collections::HashMap, fmt};

use crate::{
    geometry::Geometry,
    storage::Handle,
    topology::{Cycle, Face, HalfEdge, Region, Shell, Sketch, Solid},
    validation::{ValidationCheck, ValidationConfig},
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
        _: &'r Geometry,
        _: &'r ValidationConfig,
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
        _: &'r Geometry,
        _: &'r ValidationConfig,
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

impl ValidationCheck<Solid> for MultipleReferencesToObject<Face, Shell> {
    fn check<'r>(
        object: &'r Solid,
        _: &'r Geometry,
        _: &'r ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        let mut faces = ReferenceCounter::new();

        for shell in object.shells() {
            for face in shell.faces() {
                faces.count(face.clone(), shell.clone());
            }
        }

        faces.multiples()
    }
}

impl ValidationCheck<Solid> for MultipleReferencesToObject<Region, Face> {
    fn check<'r>(
        object: &'r Solid,
        _: &'r Geometry,
        _: &'r ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        let mut regions = ReferenceCounter::new();

        for shell in object.shells() {
            for face in shell.faces() {
                regions.count(face.region().clone(), face.clone());
            }
        }

        regions.multiples()
    }
}

impl ValidationCheck<Solid> for MultipleReferencesToObject<Cycle, Region> {
    fn check<'r>(
        object: &'r Solid,
        _: &'r Geometry,
        _: &'r ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        let mut cycles = ReferenceCounter::new();

        for shell in object.shells() {
            for face in shell.faces() {
                for cycle in face.region().all_cycles() {
                    cycles.count(cycle.clone(), face.region().clone());
                }
            }
        }

        cycles.multiples()
    }
}

impl ValidationCheck<Solid> for MultipleReferencesToObject<HalfEdge, Cycle> {
    fn check<'r>(
        object: &'r Solid,
        _: &'r Geometry,
        _: &'r ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        let mut half_edges = ReferenceCounter::new();

        for shell in object.shells() {
            for face in shell.faces() {
                for cycle in face.region().all_cycles() {
                    for half_edge in cycle.half_edges() {
                        half_edges.count(half_edge.clone(), cycle.clone());
                    }
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
        geometry::GlobalPath,
        operations::{
            build::{
                BuildHalfEdge, BuildShell, BuildSketch, BuildSolid,
                BuildSurface,
            },
            insert::Insert,
            update::{UpdateRegion, UpdateShell, UpdateSketch, UpdateSolid},
        },
        topology::{
            Cycle, Face, HalfEdge, Region, Shell, Sketch, Solid, Surface,
        },
        validate::Validate,
        validation::{
            checks::MultipleReferencesToObject, ValidationCheck,
            ValidationError,
        },
        Core,
    };

    #[test]
    fn multiple_references_to_cycle_within_sketch() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Sketch::circle([0., 0.], 1., &mut core);
        MultipleReferencesToObject::<
            Cycle,
            Region
        >::check_and_return_first_error(
            &valid,
            &core.layers.geometry,
        )?;

        let invalid = valid.add_regions(
            [Region::new(
                valid.regions().first().exterior().clone(),
                vec![],
            )],
            &mut core,
        );
        MultipleReferencesToObject::<Cycle, Region>::check_and_expect_one_error(
            &invalid,
            &core.layers.geometry,
        );

        Ok(())
    }

    #[test]
    fn multiple_references_to_half_edge_within_sketch() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Sketch::polygon([[0., 0.], [1., 1.], [0., 1.]], &mut core);
        MultipleReferencesToObject::<
            HalfEdge,
            Cycle
        >::check_and_return_first_error(
            &valid,
            &core.layers.geometry,
        )?;

        let invalid = valid.update_region(
            valid.regions().first(),
            |region, core| {
                [region.add_interiors(
                    [Cycle::new(
                        region.exterior().half_edges().iter().cloned(),
                    )],
                    core,
                )]
            },
            &mut core,
        );
        assert!(
            MultipleReferencesToObject::<
                HalfEdge,
                Cycle
            >::check_and_return_first_error(
                &invalid,
                &core.layers.geometry,
            )
            .is_err()
        );

        Ok(())
    }

    #[test]
    fn multiple_references_to_face_within_solid() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Solid::tetrahedron(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            &mut core,
        );
        MultipleReferencesToObject::<
            Face,
            Shell
        >::check_and_return_first_error(
            &valid.solid,
            &core.layers.geometry,
        )?;

        let invalid = valid.solid.add_shells(
            {
                let shell = Shell::tetrahedron(
                    [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
                    &mut core,
                )
                .shell;

                [shell.update_face(
                    shell.faces().first(),
                    |_, _| {
                        [valid.solid.shells().first().faces().first().clone()]
                    },
                    &mut core,
                )]
            },
            &mut core,
        );

        assert_contains_err!(
            core,
            invalid,
            ValidationError::MultipleReferencesToFace(_)
        );

        // Ignore remaining validation errors.
        let _ = core.layers.validation.take_errors();

        Ok(())
    }

    #[test]
    fn multiple_references_to_region_within_solid() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Solid::tetrahedron(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            &mut core,
        );
        MultipleReferencesToObject::<
            Region,
            Face
        >::check_and_return_first_error(
            &valid.solid,
            &core.layers.geometry,
        )?;

        let surface = Surface::from_uv(
            GlobalPath::circle_from_radius(1.),
            [0., 0., 1.],
            &mut core,
        );

        let shared_region = Region::new(
            Cycle::new(vec![HalfEdge::circle(
                [0., 0.],
                1.,
                surface.clone(),
                &mut core,
            )])
            .insert(&mut core),
            vec![],
        )
        .insert(&mut core);

        let invalid = Solid::new(vec![Shell::new(vec![
            Face::new(surface.clone(), shared_region.clone()).insert(&mut core),
            Face::new(surface, shared_region.clone()).insert(&mut core),
        ])
        .insert(&mut core)])
        .insert(&mut core);

        assert_contains_err!(
            core,
            invalid,
            ValidationError::MultipleReferencesToRegion(_)
        );

        // Ignore remaining validation errors.
        let _ = core.layers.validation.take_errors();

        Ok(())
    }

    #[test]
    fn multiple_references_to_cycle_within_solid() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Solid::tetrahedron(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            &mut core,
        );
        MultipleReferencesToObject::<
            Cycle,
            Region
        >::check_and_return_first_error(
            &valid.solid,
            &core.layers.geometry,
        )?;

        let surface = Surface::from_uv(
            GlobalPath::circle_from_radius(1.),
            [0., 0., 1.],
            &mut core,
        );

        let shared_cycle = Cycle::new(vec![HalfEdge::circle(
            [0., 0.],
            1.,
            surface.clone(),
            &mut core,
        )])
        .insert(&mut core);

        let invalid = Solid::new(vec![Shell::new(vec![
            Face::new(
                surface.clone(),
                Region::new(shared_cycle.clone(), vec![]).insert(&mut core),
            )
            .insert(&mut core),
            Face::new(
                surface,
                Region::new(shared_cycle, vec![]).insert(&mut core),
            )
            .insert(&mut core),
        ])
        .insert(&mut core)])
        .insert(&mut core);

        assert_contains_err!(
            core,
            invalid,
            ValidationError::MultipleReferencesToCycle(_)
        );

        // Ignore remaining validation errors.
        let _ = core.layers.validation.take_errors();

        Ok(())
    }

    #[test]
    fn multiple_references_to_half_edge_within_solid() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Solid::tetrahedron(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            &mut core,
        );
        MultipleReferencesToObject::<
            HalfEdge,
            Cycle
        >::check_and_return_first_error(
            &valid.solid,
            &core.layers.geometry,
        )?;

        let surface = Surface::from_uv(
            GlobalPath::circle_from_radius(1.),
            [0., 0., 1.],
            &mut core,
        );

        let shared_edge =
            HalfEdge::circle([0., 0.], 1., surface.clone(), &mut core);

        let invalid = Solid::new(vec![Shell::new(vec![Face::new(
            surface,
            Region::new(
                Cycle::new(vec![shared_edge.clone()]).insert(&mut core),
                vec![Cycle::new(vec![shared_edge.clone()]).insert(&mut core)],
            )
            .insert(&mut core),
        )
        .insert(&mut core)])
        .insert(&mut core)])
        .insert(&mut core);

        assert_contains_err!(
            core,
            invalid,
            ValidationError::MultipleReferencesToHalfEdge(_)
        );

        // Ignore remaining validation errors.
        let _ = core.layers.validation.take_errors();

        Ok(())
    }
}
