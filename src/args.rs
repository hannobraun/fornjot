use std::path::PathBuf;

/// Fornjot - Experimental CAD System
#[derive(clap::Parser)]
pub struct Args {
    /// The model to open
    #[clap(short, long, default_value = "cuboid")]
    pub model: PathBuf,

    /// Export model to this path
    #[clap(short, long)]
    pub export: Option<PathBuf>,

    /// Parameters for the model, each in the form `key=value`
    #[clap(short, long)]
    pub parameters: Vec<String>,

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
