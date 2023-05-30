use std::{ops::Deref, path::PathBuf};

use fj_kernel::algorithms::{approx::Tolerance, triangulate::Triangulate};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let cuboid = cuboid::cuboid(3., 2., 1.);

    // The tolerance makes no difference for this model, as there aren't any
    // curves.
    let tolerance = Tolerance::from_scalar(1.)?;

    let mesh = (cuboid.deref(), tolerance).triangulate();

    if let Some(path) = args.export {
        fj_export::export(&mesh, &path)?;
    } else {
        fj_window::run(mesh, false)?;
    }

    Ok(())
}

/// Standardized CLI for Fornjot models
#[derive(clap::Parser)]
pub struct Args {
    /// Export model to this path
    #[arg(short, long, value_name = "PATH")]
    pub export: Option<PathBuf>,
}

impl Args {
    /// Parse the command-line arguments
    ///
    /// Convenience method that saves the caller from having to import the
    /// `clap::Parser` trait.
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
