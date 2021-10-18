use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
    /// Export model to this path
    #[clap(short, long)]
    pub export: Option<PathBuf>,

    /// The model parameters as a JSON object
    #[clap(short, long)]
    pub model_params: Option<String>,
}

impl Args {
    /// Parse the command-line arguments
    ///
    /// Convenience method that saves the caller from having to import the
    /// `Clap` trait.
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
