use std::{collections::HashMap, path::PathBuf};

use anyhow::anyhow;

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
    #[clap(short, long, parse(try_from_str = Parameters::parse))]
    pub parameters: Parameters,

    /// Model deviation tolerance
    #[clap[short, long]]
    pub tolerance: Option<f64>,
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

pub struct Parameters(pub HashMap<String, String>);

impl Parameters {
    fn parse(input: &str) -> Result<Self, anyhow::Error> {
        let mut output = HashMap::new();

        for parameter in input.split(',') {
            let mut parameter = parameter.splitn(2, '=');

            let key = parameter
                .next()
                .ok_or_else(|| anyhow!("key not found"))?
                .to_owned();
            let value = parameter
                .next()
                .ok_or_else(|| anyhow!("value not found"))?
                .to_owned();

            output.insert(key, value);
        }

        Ok(Self(output))
    }
}
