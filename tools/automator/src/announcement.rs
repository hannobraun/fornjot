use std::{collections::HashSet, fmt::Write, path::PathBuf};

use anyhow::Context;
use chrono::{Date, Datelike, Utc};
use map_macro::set;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::pull_requests::{Author, PullRequest};

pub async fn create_release_announcement(
    last_release_date: Date<Utc>,
    version: String,
) -> anyhow::Result<()> {
    let now = Utc::now();

    let year = now.year();
    let week = now.iso_week().week();

    let pull_requests =
        PullRequest::fetch_since_last_release(last_release_date)
            .await?
            .into_values();

    let mut file = create_file(year, week).await?;
    generate_announcement(week, version, pull_requests, &mut file).await?;

    Ok(())
}

async fn create_file(year: i32, week: u32) -> anyhow::Result<File> {
    let dir =
        PathBuf::from(format!("content/blog/weekly-release/{year}-w{week}"));
    let file = dir.join("index.md");

    fs::create_dir_all(&dir).await.with_context(|| {
        format!("Failed to create directory `{}`", dir.display())
    })?;
    let file = File::create(&file).await.with_context(|| {
        format!("Failed to create file `{}`", file.display())
    })?;

    Ok(file)
}

async fn generate_announcement(
    week: u32,
    version: String,
    pull_requests: impl IntoIterator<Item = PullRequest>,
    file: &mut File,
) -> anyhow::Result<()> {
    let mut pull_request_list = String::new();
    let mut pull_request_links = String::new();
    let mut author_links = String::new();

    let author_blacklist = set! {
        "hannobraun",
        "dependabot[bot]"
    };
    let mut authors = HashSet::new();

    for pull_request in pull_requests {
        let PullRequest {
            number,
            title,
            url,
            author,
        } = pull_request;

        let author = if authors.contains(&author.name)
            || author_blacklist.contains(author.name.as_str())
        {
            None
        } else {
            authors.insert(author.name.clone());
            Some(author)
        };

        let thanks = match author.as_ref() {
            Some(author) => format!("; thank you, [@{}]!", author.name),
            None => String::new(),
        };

        let item = format!("- {title} ([#{number}]{thanks})\n");
        pull_request_list.push_str(&item);

        let link = format!("[#{number}]: {url}\n");
        pull_request_links.push_str(&link);

        if let Some(Author { name, profile }) = author {
            let author_link = format!("[@{name}]: {profile}\n");
            author_links.push_str(&author_link);
        }
    }

    let mut buf = String::new();
    write!(
        buf,
        "\
+++
title = \"Weekly Release - 2022-W{week}\"

[extra]
version = \"{version}\"
+++

**TASK: Write introduction.**


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

**TASK: Add end-user improvements.**


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

**TASK: Add ecosystem improvements.**


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

**TASK: Add internal improvements.**


### Unsorted pull requests

**TASK: Sort into the categories above; update/merge as appropriate.**

{pull_request_list}
{pull_request_links}
{author_links}

### Issue of the Week

**TASK: Write.**


### Outlook

**TASK: Write.**\n\
    "
    )?;

    file.write_all(buf.as_bytes()).await?;

    Ok(())
}
