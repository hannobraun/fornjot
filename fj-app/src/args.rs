use std::{path::PathBuf, str::FromStr as _};

use fj_kernel::algorithms::Tolerance;
use fj_math::Scalar;

/// Fornjot - Experimental CAD System
#[derive(clap::Parser)]
pub struct Args {
    /// The model to open
    #[clap(short, long)]
    pub model: Option<PathBuf>,

    /// Export model to this path
    #[clap(short, long)]
    pub export: Option<PathBuf>,

    /// Parameters for the model, each in the form `key=value`
    #[clap(short, long)]
    pub parameters: Vec<String>,

    /// Model deviation tolerance
    #[clap[short, long, parse(try_from_str = parse_tolerance)]]
    pub tolerance: Option<Tolerance>,
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

fn parse_tolerance(input: &str) -> anyhow::Result<Tolerance> {
    let tolerance = f64::from_str(input)?;
    let tolerance = Scalar::from_f64(tolerance);
    let tolerance = Tolerance::from_scalar(tolerance)?;

    Ok(tolerance)
}
