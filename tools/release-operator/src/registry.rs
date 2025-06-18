use anyhow::{anyhow, bail, Context};
use cargo_metadata::MetadataCommand;
use secstr::SecUtf8;
use serde::Deserialize;
use std::collections::{BTreeSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("crate was not found on crates.io")]
    CrateNotFound,
}

pub struct Registry {
    token: SecUtf8,
    dry_run: bool,
}

#[derive(Clone, Debug)]
pub struct Crate {
    path: PathBuf,
}

enum CrateState {
    /// Our crate version is ahead of the registry and should be published
    Ahead,
    /// Our crate version is behind the registry; you'll be warned about this
    Behind,
    /// Our crate version matches the registry version
    Published,
    /// We encountered an unknown state while processing the crate
    Unknown,
}

impl Registry {
    pub fn new(token: &SecUtf8, dry_run: bool) -> Self {
        Self {
            token: token.to_owned(),
            dry_run,
        }
    }

    pub fn publish_crates(&self) -> anyhow::Result<()> {
        let mut crates = VecDeque::new();

        for dir_entry in fs::read_dir("crates")? {
            let dir_entry = dir_entry?;
            let name = dir_entry
                .file_name()
                .into_string()
                .map_err(|err| anyhow!("Error converting string: {err:?}"))?;

            let mut command = MetadataCommand::new();
            command.manifest_path(dir_entry.path().join("Cargo.toml"));

            let metadata = command.exec()?;

            crates.push_back((name, metadata));
        }

        let mut crates_ordered = Vec::new();
        let mut processed_dependencies = BTreeSet::new();

        while let Some((name, metadata)) = crates.pop_front() {
            let mut package = None;
            for p in &metadata.packages {
                if p.name.as_str() == name {
                    package = Some(p.clone());
                }
            }

            let package = package.expect("Could not find package");

            let mut unmet_dependencies = false;

            for dependency in &package.dependencies {
                let unmet_dependency = dependency.path.is_some()
                    && !processed_dependencies.contains(&dependency.name);

                if unmet_dependency {
                    unmet_dependencies = true;
                    break;
                }
            }

            if unmet_dependencies {
                crates.push_back((name, metadata));
            } else {
                let mut path = package.manifest_path.into_std_path_buf();
                path.pop();

                processed_dependencies.insert(name);
                crates_ordered.push(Crate { path });
            }
        }

        for c in crates_ordered {
            c.validate()?;

            match c.determine_state()? {
                CrateState::Published | CrateState::Behind => continue,
                CrateState::Unknown | CrateState::Ahead => {
                    c.submit(&self.token, self.dry_run)?;
                }
            }
        }

        Ok(())
    }
}

impl Crate {
    fn validate(&self) -> anyhow::Result<()> {
        match self.path.exists() {
            true => Ok(()),
            false => Err(anyhow!(
                "given path to the '{self}' crate is either not readable or does not exist"
            )),
        }
    }

    fn get_local_version(&self) -> anyhow::Result<semver::Version> {
        let name = format!("{self}");
        let cargo_toml_location = std::fs::canonicalize(&self.path)
            .context("absolute path to Cargo.toml")?;
        let mut cmd = cargo_metadata::MetadataCommand::new();
        cmd.manifest_path(format!(
            "{}/Cargo.toml",
            cargo_toml_location.to_string_lossy()
        ))
        .no_deps();

        let metadata = cmd.exec()?;
        let package = metadata
            .packages
            .iter()
            .find(|p| p.name.as_str() == name)
            .ok_or_else(|| anyhow!("could not find package"))?;

        let version = package.version.to_owned();
        log::debug!("{self} found as {version} on our side");

        Ok(version)
    }

    fn get_upstream_version(&self) -> anyhow::Result<semver::Version> {
        #[derive(Deserialize)]
        struct CrateVersions {
            versions: Vec<CrateVersion>,
        }

        #[derive(Deserialize)]
        struct CrateVersion {
            #[serde(rename = "num")]
            version: semver::Version,
        }

        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .context("build http client")?;

        let resp = client
            .get(format!("https://crates.io/api/v1/crates/{self}"))
            .send()
            .context("fetching crate versions from the registry")?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(anyhow::Error::new(Error::CrateNotFound));
        }

        if resp.status() != reqwest::StatusCode::OK {
            return Err(anyhow!(
                "{self} request to crates.io failed with {} '{}'",
                resp.status(),
                resp.text().unwrap_or_else(|_| {
                    "[response body could not be read]".to_string()
                })
            ));
        }

        let versions =
            serde_json::from_str::<CrateVersions>(resp.text()?.as_str())
                .context("deserializing crates.io response")?;

        Ok(versions.versions.first().unwrap().version.to_owned())
    }

    fn determine_state(&self) -> anyhow::Result<CrateState> {
        let theirs = match self.get_upstream_version() {
            Ok(version) => version,
            Err(error) => match error.downcast_ref::<Error>() {
                Some(Error::CrateNotFound) => return Ok(CrateState::Unknown),
                None => return Err(error),
            },
        };

        let ours = self.get_local_version()?;

        match theirs.cmp(&ours) {
            std::cmp::Ordering::Less => Ok(CrateState::Ahead),
            std::cmp::Ordering::Equal => {
                log::info!("{self} has already been published as {ours}");
                Ok(CrateState::Published)
            }
            std::cmp::Ordering::Greater => {
                log::warn!("{self} has already been published as {ours}, which is a newer version");
                Ok(CrateState::Behind)
            }
        }
    }

    fn submit(&self, token: &SecUtf8, dry_run: bool) -> anyhow::Result<()> {
        log::info!("{self} publishing new version");

        let ours = self.get_local_version()?;
        let delay = Duration::from_secs(10);
        let start_time = Instant::now();
        let timeout = Duration::from_secs(600);
        let mut attempt = 1;
        let maximum_attempts = 10;

        loop {
            if Instant::now() - start_time > timeout {
                bail!("{self} could not be published to the registry within {timeout:?}");
            }

            if attempt > maximum_attempts {
                bail!("{self} could not be published to the registry; tried {attempt} times");
            }

            let mut command = Command::new("cargo");
            command
                .arg("publish")
                .args(["--token", token.unsecure()])
                .current_dir(&self.path);

            if dry_run {
                command.arg("--dry-run");
            }

            let exit_status = command.status()?;

            if !exit_status.success() {
                match exit_status.code() {
                    Some(code) => {
                        log::warn!("`cargo publish` failed with exit code {code}; trying again in {delay:?}");
                        sleep(delay);
                        attempt += 1;
                        continue;
                    }
                    None => {
                        bail!("`cargo publish` was terminated by signal")
                    }
                }
            }

            log::info!("{self} published as {ours}");
            break;
        }

        Ok(())
    }
}

impl FromStr for Crate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            path: PathBuf::from(s),
        })
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.path.file_name() {
            return write!(f, "{}", name.to_string_lossy());
        }
        write!(f, "{:?}", self.path)
    }
}
