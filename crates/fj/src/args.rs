use std::{num::ParseFloatError, path::PathBuf, str::FromStr};

use fj_core::{
    interop::{InvalidTolerance, Tolerance},
    math::Scalar,
};

/// Standardized CLI for Fornjot models
///
/// This is completely optional, as models are just Rust code and don't need any
/// kind of CLI interface. It is useful, however, to provide a standardized
/// interface for viewing and exporting models, and is used for Fornjot's
/// example models and the testing infrastructure they are part of.
///
/// You might not want to use this struct directly. [`Instance::process_model`]
/// provides a more high-level and convenient interface.
///
/// [`Instance::process_model`]: crate::Instance::process_model
#[derive(clap::Parser)]
pub struct Args {
    /// Export model to this path
    #[arg(short, long, value_name = "PATH")]
    pub export: Option<PathBuf>,

    /// How much the export can deviate from the original model
    #[arg(short, long, value_parser = parse_tolerance)]
    pub tolerance: Option<Tolerance>,

    /// Ignore validation errors
    #[arg(short, long)]
    pub ignore_validation: bool,
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
