use std::env;

use anyhow::Context;
use octocrab::Octocrab;

use crate::{
    args::{Args, Blog},
    blog,
    sponsors::Sponsors,
};

pub async fn run() -> anyhow::Result<()> {
    let token = env::var("GITHUB_TOKEN")
        .context("Loading env variable `GITHUB_TOKEN`")?;
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    match Args::parse() {
        Args::Blog(Blog::Release) => {
            blog::create_release_announcement(&octocrab)
                .await
                .context("Failed to create release announcement")?;
        }
        Args::Sponsors(args) => {
            let sponsors = Sponsors::query(&octocrab)
                .await
                .context("Failed to query sponsors")?
                .as_markdown(8, args.for_readme)
                .context("Failed to format sponsors")?;

            println!("{sponsors}");
        }
    }

    Ok(())
}
