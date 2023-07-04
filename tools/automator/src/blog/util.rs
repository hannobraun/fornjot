use std::path::PathBuf;

use anyhow::Context;
use chrono::{Datelike, Utc};
use tokio::fs::{self, File};

pub fn date() -> String {
    let now = Utc::now();
    let year = now.year();
    format!("{year}-{:02}-{:02}", now.month(), now.day())
}

pub async fn create_blog_post_file(
    category: &str,
    title: &str,
) -> anyhow::Result<File> {
    let dir = PathBuf::from(format!("content/blog/{category}/{title}"));
    let file = dir.join("index.md");

    // VS Code (and probably other editors/IDEs) renders the path in the output
    // as a clickable link, so the user can open the file easily.
    println!("Generating `{category}` blog post at {}", file.display());

    fs::create_dir_all(&dir).await.with_context(|| {
        format!("Failed to create directory `{}`", dir.display())
    })?;
    let file = File::create(&file).await.with_context(|| {
        format!("Failed to create file `{}`", file.display())
    })?;

    Ok(file)
}
