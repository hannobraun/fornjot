use octocrab::Octocrab;

pub async fn query_sponsors(octocrab: &Octocrab) -> anyhow::Result<()> {
    let response: serde_json::Value = octocrab
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

    println!("{response}");

    Ok(())
}
