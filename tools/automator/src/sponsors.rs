use chrono::{DateTime, Utc};
use octocrab::Octocrab;

#[derive(Debug)]
pub struct Sponsor {
    pub login: String,
    pub since: DateTime<Utc>,
    pub dollars: u32,
}

pub async fn query_sponsors(octocrab: &Octocrab) -> anyhow::Result<()> {
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

    let sponsors = response
        .data
        .viewer
        .sponsors
        .nodes
        .into_iter()
        .map(|node| {
            let login = node.login;
            let since = node.sponsorship_for_viewer_as_sponsorable.created_at;
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

    println!("{sponsors:#?}");

    Ok(())
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
