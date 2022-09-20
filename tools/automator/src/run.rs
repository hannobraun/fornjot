use std::env;

use anyhow::Context;
use octocrab::Octocrab;

use crate::{announcement::create_release_announcement, args::Args};

pub async fn run() -> anyhow::Result<()> {
    let token = env::var("GITHUB_TOKEN")
        .context("Loading env variable `GITHUB_TOKEN`")?;
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    match Args::parse() {
        Args::Announcement => {
            create_release_announcement(&octocrab)
                .await
                .context("Failed to create release announcement")?;
        }
        Args::Sponsors => {
            let response: serde_json::Value =
                octocrab.graphql("query { viewer { login }}").await?;
            println!("{response}");

            todo!("Querying sponsors is not supported yet.")
        }
    }

    Ok(())
}
