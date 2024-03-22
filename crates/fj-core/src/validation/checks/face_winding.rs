use fj_math::Winding;

use crate::{
    objects::{Cycle, Face},
    storage::Handle,
    validation::ValidationCheck,
};

/// Interior [`Cycle`] of [`Face`] has invalid winding
///
/// The winding of a face's exterior cycle is part of what defines the
/// orientation of that face. The winding of the interior cycle has no such
/// meaning attached to it, but it can't be arbitrary either. Triangulation, for
/// example, might need to assume that it is the opposite of the exterior
/// winding.
///
/// This validation check ensures just that: that the winding of the interior
/// cycles of a face is the opposite of the winding of that face's exterior
/// cycle.
#[derive(Clone, Debug, thiserror::Error)]
#[error(
    "Interior of `Face` has invalid winding; must be opposite of exterior\n\
    - Winding of exterior cycle: {exterior_winding:#?}\n\
    - Interior cycle with invalid winding: {interior_cycle:#?}\n\
    - Winding of invalid interior cycle: {interior_winding:#?}"
)]
pub struct InteriorCycleHasInvalidWinding {
    /// The winding of the [`Face`]'s exterior cycle
    pub exterior_winding: Winding,

    /// The interior cycle with invalid winding
    pub interior_cycle: Handle<Cycle>,

    /// The winding of the invalid interior cycle
    pub interior_winding: Winding,
}

impl ValidationCheck<Face> for InteriorCycleHasInvalidWinding {
    fn check(
        object: &Face,
        geometry: &crate::geometry::Geometry,
        _: &crate::validation::ValidationConfig,
    ) -> impl Iterator<Item = Self> {
        object.region().interiors().iter().filter_map(|interior| {
            let exterior = object.region().exterior();

            if exterior.half_edges().is_empty()
                || interior.half_edges().is_empty()
            {
                // Can't determine winding, if the cycle has no edges. Sounds
                // like a job for a different validation check.
                return None;
            }

            let exterior_winding = exterior.winding(geometry);
            let interior_winding = interior.winding(geometry);

            if exterior_winding == interior_winding {
                return Some(InteriorCycleHasInvalidWinding {
                    exterior_winding,
                    interior_cycle: interior.clone(),
                    interior_winding,
                });
            }

            None
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        objects::{Cycle, Face, Region},
        operations::{
            build::{BuildCycle, BuildFace},
            derive::DeriveFrom,
            insert::Insert,
            reverse::Reverse,
            update::{UpdateFace, UpdateRegion},
        },
        validation::{checks::InteriorCycleHasInvalidWinding, ValidationCheck},
        Core,
    };

    #[test]
    fn interior_winding() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Face::polygon(
            core.layers.objects.surfaces.xy_plane(),
            [[0., 0.], [3., 0.], [0., 3.]],
            &mut core,
        )
        .update_region(
            |region, core| {
                region.add_interiors(
                    [Cycle::polygon([[1., 1.], [1., 2.], [2., 1.]], core)],
                    core,
                )
            },
            &mut core,
        );
        InteriorCycleHasInvalidWinding::check_and_return_first_error(
            &valid,
            &core.layers.geometry,
        )?;

        let invalid = {
            let interiors = valid
                .region()
                .interiors()
                .iter()
                .cloned()
                .map(|cycle| {
                    cycle
                        .reverse(&mut core)
                        .insert(&mut core)
                        .derive_from(&cycle, &mut core)
                })
                .collect::<Vec<_>>();

            let region =
                Region::new(valid.region().exterior().clone(), interiors)
                    .insert(&mut core);

            Face::new(valid.surface().clone(), region)
        };
        InteriorCycleHasInvalidWinding::check_and_expect_one_error(
            &invalid,
            &core.layers.geometry,
        );

        Ok(())
    }
}