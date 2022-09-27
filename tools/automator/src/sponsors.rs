use std::{cmp::Ordering, fmt::Write};

use chrono::{DateTime, Utc};
use octocrab::Octocrab;

#[derive(Debug)]
pub struct Sponsors {
    pub inner: Vec<Sponsor>,
}

impl Sponsors {
    pub async fn query(octocrab: &Octocrab) -> anyhow::Result<Sponsors> {
        let response: QueryResult = octocrab
            .graphql(
                "query {
                    viewer {
                        sponsors(first: 100) {
                            nodes {
                                __typename
                                ... on User {
                                    login
                                    sponsorshipForViewerAsSponsorable {
                                        createdAt
                                        tier {
                                            monthlyPriceInDollars
                                        }
                                    }
                                }
                                ... on Organization {
                                    login
                                    sponsorshipForViewerAsSponsorable {
                                        createdAt
                                        tier {
                                            monthlyPriceInDollars
                                        }
                                    }
                                }
                            }
                        }
                    }
                }",
            )
            .await?;

        let mut sponsors = response
            .data
            .viewer
            .sponsors
            .nodes
            .into_iter()
            .map(|node| {
                let login = node.login;
                let since =
                    node.sponsorship_for_viewer_as_sponsorable.created_at;
                let dollars = node
                    .sponsorship_for_viewer_as_sponsorable
                    .tier
                    .monthly_price_in_dollars;

                Sponsor {
                    login,
                    since,
                    dollars,
                }
            })
            .collect::<Vec<_>>();

        if sponsors.len() >= 100 {
            todo!(
                "Number of sponsors has reached max page size, but query does \
                not support pagination."
            )
        }

        sponsors.sort();

        Ok(Sponsors { inner: sponsors })
    }

    pub fn as_markdown(&self, min_dollars: u32) -> anyhow::Result<String> {
        let mut output = String::from("Fornjot is supported by ");

        for sponsor in &self.inner {
            if sponsor.dollars < min_dollars {
                continue;
            }

            let login = &sponsor.login;
            let url = format!("https://github.com/{login}");

            write!(output, "[@{login}]({url}), ")?;
        }

        output.push_str(
            "and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!"
        );

        Ok(output)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Sponsor {
    pub login: String,
    pub since: DateTime<Utc>,
    pub dollars: u32,
}

impl Ord for Sponsor {
    fn cmp(&self, other: &Self) -> Ordering {
        let by_dollars = other.dollars.cmp(&self.dollars);
        let by_date = self.since.cmp(&other.since);
        let by_login = self.login.cmp(&other.login);

        if by_dollars.is_ne() {
            return by_dollars;
        }

        if by_date.is_ne() {
            return by_date;
        }

        by_login
    }
}

impl PartialOrd for Sponsor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct QueryResult {
    pub data: QueryResultData,
}

#[derive(Debug, serde::Deserialize)]
pub struct QueryResultData {
    pub viewer: QueryResultViewer,
}

#[derive(Debug, serde::Deserialize)]
pub struct QueryResultViewer {
    pub sponsors: QueryResultSponsorsNodes,
}

#[derive(Debug, serde::Deserialize)]
pub struct QueryResultSponsorsNodes {
    pub nodes: Vec<QueryResultSponsorsNode>,
}

#[derive(Debug, serde::Deserialize)]
pub struct QueryResultSponsorsNode {
    pub login: String,

    #[serde(rename = "sponsorshipForViewerAsSponsorable")]
    pub sponsorship_for_viewer_as_sponsorable: QueryResultSponsorable,
}

#[derive(Debug, serde::Deserialize)]
pub struct QueryResultSponsorable {
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    pub tier: QueryResultSponsorableTier,
}

#[derive(Debug, serde::Deserialize)]
pub struct QueryResultSponsorableTier {
    #[serde(rename = "monthlyPriceInDollars")]
    pub monthly_price_in_dollars: u32,
}
