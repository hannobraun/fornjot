use std::{path::PathBuf, str::FromStr as _};

use anyhow::anyhow;
use fj_host::Parameters;
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
    #[clap(short, long, parse(try_from_str = parse_parameters))]
    pub parameters: Option<Parameters>,

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

fn parse_parameters(input: &str) -> anyhow::Result<Parameters> {
    let mut parameters = Parameters::empty();

    for parameter in input.split(',') {
        let mut parameter = parameter.splitn(2, '=');

        let key = parameter
            .next()
            .ok_or_else(|| anyhow!("Expected model parameter key"))?
            .to_owned();
        let value = parameter
            .next()
            .ok_or_else(|| anyhow!("Expected model parameter value"))?
            .to_owned();

        parameters.0.insert(key, value);
    }

    Ok(parameters)
}

fn parse_tolerance(input: &str) -> anyhow::Result<Tolerance> {
    let tolerance = f64::from_str(input)?;
    let tolerance = Scalar::from_f64(tolerance);
    let tolerance = Tolerance::from_scalar(tolerance)?;

    Ok(tolerance)
}
