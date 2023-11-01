use fj_math::Point;

use crate::{
    objects::{Shell, Solid},
    operations::{
        build::shell::BuildShell, Insert, IsInsertedYes, TetrahedronShell,
        UpdateSolid,
    },
    services::Services,
};

/// Build a [`Solid`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildSolid {
    /// Build an empty solid
    fn empty() -> Solid {
        Solid::new([])
    }

    /// Build a tetrahedron from the provided points
    ///
    /// See [`BuildShell::tetrahedron`] for more information.
    fn tetrahedron(
        points: [impl Into<Point<3>>; 4],
        services: &mut Services,
    ) -> Tetrahedron {
        let shell = Shell::tetrahedron(points, services).insert(services);
        let solid = Solid::empty().add_shells([shell.shell.clone()]);

        Tetrahedron { solid, shell }
    }
}

impl BuildSolid for Solid {}

/// A tetrahedron
///
/// Returned by [`BuildSolid::tetrahedron`].
pub struct Tetrahedron {
    /// The solid that forms the tetrahedron
    pub solid: Solid,

    /// The shell of the tetrahedron
    pub shell: TetrahedronShell<IsInsertedYes>,
}
