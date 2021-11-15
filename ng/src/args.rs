use std::path::PathBuf;

/// Fornjot - Experimental CAD System - Host Application
#[derive(clap::Parser)]
pub struct Args {
    /// The model to open
    #[clap(short, long, default_value = "cube")]
    pub model: String,

    /// Export model to this path
    #[clap(short, long)]
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
