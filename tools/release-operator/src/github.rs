use crate::release::Outputs;
use cmd_lib::run_fun;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PullRequest {
    number: u32,
}

#[derive(Deserialize, Debug)]
pub struct Labels {
    labels: Vec<Label>,
}

#[derive(Deserialize, Debug)]
pub struct Label {
    name: String,
}

pub struct GitHub;

impl GitHub {
    /// Shells out to GitHub's CLI `gh` to try and determine if the commit belongs to any pull-request
    pub fn find_pull_request_by(
        sha: &str,
        marker_label: &str,
    ) -> anyhow::Result<Option<PullRequest>> {
        log::trace!("listing pull-requests that contain the commit:");
        let pulls = run_fun!(gh pr list --state merged --search ${sha} --limit 1 --json number)?;
        log::trace!("{pulls}");

        let pulls: Vec<PullRequest> = serde_json::from_str(&pulls)?;

        if pulls.is_empty() {
            log::debug!("the commit sha {sha} is not part of any pull-request");
            return Ok(None);
        }

        let pr = pulls.first().unwrap().clone();
        log::trace!("extracted {pr:?}");

        log::trace!("getting labels for the possibly qualified pull-request:");
        let labels = {
            let pr_number = pr.number;
            run_fun!(gh pr view ${pr_number} --json labels)?
        };
        log::trace!("{labels}");

        let labels: Labels = serde_json::from_str(&labels)?;

        if labels.labels.is_empty() {
            log::debug!(
            "the commit sha {sha} is not part of any pull-request with the {marker_label} label"
        );
            return Ok(None);
        }

        for label in labels.labels {
            if label.name == marker_label {
                return Ok(Some(pr));
            }
        }

        Ok(None)
    }
}

pub struct Actions;

impl Actions {
    // Set an "output" in GitHub Actions
    pub fn set_output(key: Outputs, value: &str) {
        log::debug!("setting output name={key} value={value}");
        println!("{key}={value} >> $GITHUB_OUTPUT");
    }
}
