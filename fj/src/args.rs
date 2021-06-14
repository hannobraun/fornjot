use std::path::PathBuf;

use clap::Clap;

#[derive(Clap)]
pub struct Args {
    /// Export model to this path
    #[clap(short, long)]
    pub export: Option<PathBuf>,
}

impl Args {
    /// Parse the command-line arguments
    ///
    /// Convenience method that saves the caller from having to import the
    /// `Clap` trait.
    pub fn parse() -> Self {
        <Self as Clap>::parse()
    }
}
