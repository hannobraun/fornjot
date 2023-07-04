use std::{collections::HashSet, fmt::Write};

use map_macro::hash_set;
use octocrab::Octocrab;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{
    pull_requests::{Author, PullRequest, PullRequestsSinceLastRelease},
    sponsors::Sponsors,
};

use super::util;

pub async fn create_release_announcement(
    octocrab: &Octocrab,
) -> anyhow::Result<()> {
    let date = util::date();

    let pull_requests_since_last_release =
        PullRequestsSinceLastRelease::fetch(octocrab).await?;

    let pull_requests =
        pull_requests_since_last_release.pull_requests.into_values();

    // For now, it's good enough to just release a new minor version every time.
    // We could also determine whether there were breaking changes to make sure
    // we actually need it, but as of now, breaking changes are pretty much a
    // given.
    let mut version = pull_requests_since_last_release.version_of_last_release;
    version.minor += 1;
    let version = version.to_string();

    let min_dollars = 32;
    let for_readme = false;
    let sponsors = Sponsors::query(octocrab)
        .await?
        .as_markdown(min_dollars, for_readme)?;

    let mut file = util::create_blog_post_file("release", &version).await?;
    generate_announcement(date, version, sponsors, pull_requests, &mut file)
        .await?;

    Ok(())
}

async fn generate_announcement(
    date: String,
    version: String,
    sponsors: String,
    pull_requests: impl IntoIterator<Item = PullRequest>,
    file: &mut File,
) -> anyhow::Result<()> {
    let mut pull_request_list = String::new();
    let mut pull_request_links = String::new();
    let mut author_links = String::new();

    let author_blacklist = hash_set! {
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
            ..
        } = pull_request;

        let author = if author_blacklist.contains(author.name.as_str()) {
            None
        } else {
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
            if !authors.contains(&name) {
                let author_link = format!("[@{name}]: {profile}\n");
                author_links.push_str(&author_link);

                authors.insert(name.clone());
            }
        }
    }

    let mut buf = String::new();
    write!(
        buf,
        "\
+++
title = \"Fornjot {version}\"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = {date}

# Uncomment to generate the HTML for the email newsletter.
# template = \"newsletter/release.html\"

[extra]
version = \"{version}\"
# TASK: Choose a descriptive subtitle.
subtitle = \"This is a subtitle\"
+++

**TASK: Write introduction.**


### Sponsors

{sponsors}

<strong class=\"call-to-action\">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href=\"https://github.com/sponsors/hannobraun\">supporting me</a> too.
    </p>
</strong>


### Library improvements

Improvements to Fornjot libraries.

#### `fj-core`

**TASK: Add library improvements.**


### Other changes

Improvements that are not associated with a specific Fornjot library.

**TASK: Add internal improvements.**


### Unsorted pull requests

**TASK: Sort into the categories above; update/merge as appropriate.**

{pull_request_list}
{pull_request_links}
{author_links}\
    "
    )?;

    file.write_all(buf.as_bytes()).await?;

    Ok(())
}
