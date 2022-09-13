use anyhow::Context;
use octocrab::Octocrab;

use crate::{announcement::create_release_announcement, args::Args};

pub async fn run() -> anyhow::Result<()> {
    let octocrab = Octocrab::builder().build()?;

    match Args::parse() {
        Args::CreateReleaseAnnouncement(_) => {
            create_release_announcement(&octocrab)
                .await
                .context("Failed to create release announcement")?;
        }
        Args::Sponsors => {
            todo!("Querying sponsors is not supported yet.")
        }
    }

    Ok(())
}
