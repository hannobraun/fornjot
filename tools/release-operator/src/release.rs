use crate::{Actions, GitHub};
use regex::Regex;
use std::fmt::{Display, Formatter};

pub struct Release {
    sha: String,
    label: String,
}

pub enum Outputs {
    ReleaseDetected,
    TagName,
}

impl Display for Outputs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Outputs::ReleaseDetected => write!(f, "release-detected"),
            Outputs::TagName => write!(f, "tag-name"),
        }
    }
}

impl Release {
    pub fn new(sha: String, label: String) -> Self {
        Self { sha, label }
    }

    pub fn detect(&self) -> anyhow::Result<()> {
        let sha = &self.sha;
        let label = &self.label;

        // Try and find the pull-request that the commit was part of to examine it.
        // A release can only ever be triggered by a pull-request being merged.
        if GitHub::find_pull_request_by(sha, label)?.is_none() {
            log::info!(
                "Could not find a pull request with hash {sha} and label \
                {label}",
            );
            return self.miss();
        }

        let commit: String = cmd_lib::run_fun!(git log -n 1 "${sha}")?;

        // A release commits need to contain a semver version number.
        let version = Regex::new(r"(v?\d+.\d+.\d+)")?
            .find_iter(&commit)
            .find(|m| semver::Version::parse(m.as_str()).is_ok());

        match version {
            Some(v) => self.hit(v.as_str()),
            None => {
                log::info!(
                    "Commit message is missing version number:\n\
                    {commit}",
                );
                self.miss()
            }
        }
    }

    fn hit(&self, tag: &str) -> anyhow::Result<()> {
        log::info!("detected release of {tag}");
        Actions::set_output(Outputs::ReleaseDetected, "true");
        Actions::set_output(Outputs::TagName, tag);
        Ok(())
    }

    fn miss(&self) -> anyhow::Result<()> {
        log::info!("no release detected");
        Actions::set_output(Outputs::ReleaseDetected, "false");
        Ok(())
    }
}
