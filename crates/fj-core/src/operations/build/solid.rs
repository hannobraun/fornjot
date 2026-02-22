use crate::{
    Core,
    math::Point,
    operations::{
        build::{BuildShell, TetrahedronShell},
        insert::{Insert, IsInsertedYes},
        update::UpdateSolid,
    },
    topology::{Shell, Solid},
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
        core: &mut Core,
    ) -> Tetrahedron {
        let shell = Shell::tetrahedron(points, core).insert(core);
        let solid = Solid::empty().add_shells([shell.shell.clone()], core);

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
