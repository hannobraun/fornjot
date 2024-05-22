use crate::{
    geometry::Geometry,
    topology::Shell,
    validation::{
        checks::{
            CoincidentHalfEdgesAreNotSiblings, CurveGeometryMismatch,
            HalfEdgeHasNoSibling,
        },
        ValidationCheck,
    },
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Shell {
    fn validate(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
        geometry: &Geometry,
    ) {
        errors.extend(
            CurveGeometryMismatch::check(self, geometry, config)
                .map(Into::into),
        );
        errors.extend(
            HalfEdgeHasNoSibling::check(self, geometry, config).map(Into::into),
        );
        errors.extend(
            CoincidentHalfEdgesAreNotSiblings::check(self, geometry, config)
                .map(Into::into),
        );
    }
}

/// [`Shell`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum ShellValidationError {}

impl ShellValidationError {}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains_err,
        operations::{
            build::BuildShell,
            geometry::{UpdateCurveGeometry, UpdateHalfEdgeGeometry},
            insert::Insert,
            update::{
                UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateRegion,
                UpdateShell,
            },
        },
        topology::{Curve, Shell},
        validate::{Validate, ValidationError},
        Core,
    };

    #[test]
    fn coincident_half_edges_are_not_siblings() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Shell::tetrahedron(
            [[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [0., 0., 1.]],
            &mut core,
        );
        let invalid = valid.shell.update_face(
            &valid.abc.face,
            |face, core| {
                [face.update_region(
                    |region, core| {
                        region.update_exterior(
                            |cycle, core| {
                                cycle.update_half_edge(
                                    cycle.half_edges().nth_circular(0),
                                    |half_edge, core| {
                                        let curve = Curve::new()
                                            .insert(core)
                                            .copy_geometry_from(
                                                half_edge.curve(),
                                                &mut core.layers.geometry,
                                            );

                                        [half_edge
                                            .update_curve(|_, _| curve, core)
                                            .insert(core)
                                            .set_geometry(
                                                *core
                                                    .layers
                                                    .geometry
                                                    .of_half_edge(half_edge),
                                                &mut core.layers.geometry,
                                            )]
                                    },
                                    core,
                                )
                            },
                            core,
                        )
                    },
                    core,
                )]
            },
            &mut core,
        );

        valid
            .shell
            .validate_and_return_first_error(&core.layers.geometry)?;
        assert_contains_err!(
            core,
            invalid,
            ValidationError::CoincidentHalfEdgesAreNotSiblings { .. }
        );

        Ok(())
    }
}
