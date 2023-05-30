use std::ops::Deref;

use fj::{
    core::algorithms::{approx::Tolerance, triangulate::Triangulate},
    Args,
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let cuboid = cuboid::cuboid(3., 2., 1.);

    // The tolerance makes no difference for this model, as there aren't any
    // curves.
    let tolerance = Tolerance::from_scalar(1.)?;

    let mesh = (cuboid.deref(), tolerance).triangulate();

    if let Some(path) = args.export {
        fj::export::export(&mesh, &path)?;
    } else {
        fj::window::run(mesh, false)?;
    }

    Ok(())
}
