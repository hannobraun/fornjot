use std::env;

use anyhow::Context;
use octocrab::Octocrab;

use crate::{
    announcement::create_release_announcement, args::Args, sponsors::Sponsors,
};

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
        Args::Sponsors(args) => {
            let sponsors = Sponsors::query(&octocrab)
                .await
                .context("Failed to query sponsors")?
                .as_markdown(8, args.for_readme)
                .context("Failed to format sponsors")?;

            println!("{sponsors:#?}");

            todo!("Querying sponsors is not supported yet.")
        }
    }

    Ok(())
}
