mod github;
mod release;

use crate::github::{Actions, GitHub};

use crate::release::Release;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(version, propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Detect a release and set respective Action outputs
    Detect(DetectArgs),
}

#[derive(Args, Debug)]
struct DetectArgs {
    /// Commit sha to work on
    #[clap(short, long, env = "GITHUB_SHA")]
    sha: String,

    /// Marker label to look for
    #[clap(short, long, env = "RELEASE_LABEL", default_value = "autorelease")]
    label: String,
}

fn main() -> anyhow::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    );
    env_logger::init();

    let start = std::time::Instant::now();
    log::trace!("starting release-operator process");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Detect(args) => {
            log::debug!("got arguments: {args:#?}");
            Release::new(args.sha.to_owned(), args.label.to_owned())
                .detect()?;
        }
    }

    log::trace!(
        "finished release-operator process, took {:?}",
        start.elapsed()
    );

    Ok(())
}
