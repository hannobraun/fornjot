use std::path::PathBuf;

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
