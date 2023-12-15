use std::env;

use anyhow::Context;
use octocrab::Octocrab;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    args::{Args, Blog},
    blog,
    sponsors::Sponsors,
};

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let token = env::var("GITHUB_TOKEN")
        .context("Loading env variable `GITHUB_TOKEN`")?;
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    match Args::parse() {
        Args::Blog(Blog::Release) => {
            blog::create_release_announcement(&octocrab)
                .await
                .context("Failed to create release announcement")?;
        }
        Args::Blog(Blog::SponsorUpdate) => {
            blog::create_sponsor_update()
                .await
                .context("Failed to create sponsor update")?;
        }
        Args::Sponsors(args) => {
            let min_dollars = 8;
            let sponsors = Sponsors::query(&octocrab)
                .await
                .context("Failed to query sponsors")?
                .as_markdown(min_dollars, args.for_readme)
                .context("Failed to format sponsors")?;

            println!("{sponsors}");
        }
    }

    Ok(())
}
