use anyhow::Context;

use crate::{announcement::create_release_announcement, args::Args};

pub async fn run() -> anyhow::Result<()> {
    match Args::parse() {
        Args::CreateReleaseAnnouncement(_) => {
            create_release_announcement()
                .await
                .context("Failed to create release announcement")?;
        }
    }

    Ok(())
}
