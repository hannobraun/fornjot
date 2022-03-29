mod github;
mod release;

use crate::github::{Actions, GitHub};

use crate::release::Release;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
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
    log::debug!("got arguments: {args:#?}");

    Release::new(cli.sha, cli.label).detect()?;

    log::trace!(
        "finished release-operator process, took {:?}",
        start.elapsed()
    );

    Ok(())
}
