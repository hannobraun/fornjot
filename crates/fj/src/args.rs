use std::{num::ParseFloatError, path::PathBuf, str::FromStr};

use fj_core::algorithms::approx::{InvalidTolerance, Tolerance};
use fj_math::Scalar;

/// Standardized CLI for Fornjot models
#[derive(clap::Parser)]
pub struct Args {
    /// Export model to this path
    #[arg(short, long, value_name = "PATH")]
    pub export: Option<PathBuf>,

    /// How much the export can deviate from the original model
    #[arg(short, long, value_parser = parse_tolerance)]
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

fn parse_tolerance(input: &str) -> Result<Tolerance, ArgsError> {
    let tolerance = f64::from_str(input)?;
    let tolerance = Scalar::from_f64(tolerance);
    let tolerance = Tolerance::from_scalar(tolerance)?;

    Ok(tolerance)
}

#[derive(Debug, thiserror::Error)]
pub enum ArgsError {
    #[error("Error parsing tolerance")]
    ParseTolerance(#[from] ParseFloatError),

    #[error(transparent)]
    InvalidTolerance(#[from] InvalidTolerance),
}
