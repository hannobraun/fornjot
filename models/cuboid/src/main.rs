use std::ops::Deref;

use fj_kernel::algorithms::{approx::Tolerance, triangulate::Triangulate};

fn main() -> anyhow::Result<()> {
    let cuboid = cuboid::cuboid(3., 2., 1.);

    // The tolerance makes no difference for this model, as there aren't any
    // curves.
    let tolerance = Tolerance::from_scalar(1.)?;

    let mesh = (cuboid.deref(), tolerance).triangulate();
    fj_window::run(mesh, false)?;

    Ok(())
}
