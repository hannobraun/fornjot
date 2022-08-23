use std::collections::BTreeMap;

use anyhow::anyhow;
use chrono::{Date, Utc};
use octocrab::params::{pulls::Sort, Direction, State};
use url::Url;

pub struct PullRequest {
    pub number: u64,
    pub html_url: Url,
}

impl PullRequest {
    pub async fn fetch_since_last_release(
        last_release_date: Date<Utc>,
    ) -> anyhow::Result<BTreeMap<u64, Self>> {
        let mut pull_requests = BTreeMap::new();
        let mut page = 1u32;

        'outer: loop {
            const MAX_RESULTS_PER_PAGE: u8 = 100;

            println!("Fetching page {}...", page);
            let pull_request_page = octocrab::instance()
                .pulls("hannobraun", "Fornjot")
                .list()
                .state(State::Closed)
                .sort(Sort::Updated)
                .direction(Direction::Descending)
                .per_page(MAX_RESULTS_PER_PAGE)
                .page(page)
                .send()
                .await?;

            for pull_request in pull_request_page.items {
                if let Some(updated_at) = pull_request.updated_at {
                    if updated_at.date() < last_release_date {
                        // This pull request has been updated before the last
                        // release. Since we sort pull requests by
                        // updated-descending, that means all following pull
                        // requests have been updated before the last release,
                        // and thus couldn't have been merged after.
                        break 'outer;
                    }
                }

                if let Some(merged_at) = pull_request.merged_at {
                    if merged_at.date() >= last_release_date {
                        let number = pull_request.number;
                        let html_url =
                            pull_request.html_url.ok_or_else(|| {
                                anyhow!("Pull request is missing URL")
                            })?;

                        let pull_request = Self { number, html_url };

                        pull_requests.insert(pull_request.number, pull_request);
                    }
                }
            }

            if pull_request_page.next.is_some() {
                page += 1;
            } else {
                break;
            }
        }

        Ok(pull_requests)
    }
}
