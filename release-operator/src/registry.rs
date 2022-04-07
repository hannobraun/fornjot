use anyhow::{anyhow, Context};
use secstr::SecStr;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

pub struct Registry {
    token: SecStr,
    crates: Vec<Crate>,
}

#[derive(Clone, Debug)]
pub struct Crate {
    path: PathBuf,
}

impl Registry {
    pub fn new(token: &SecStr, crates: &[Crate]) -> Self {
        Self {
            token: token.to_owned(),
            crates: crates.to_vec(),
        }
    }

    pub fn publish_crates(&self) -> anyhow::Result<()> {
        for c in &self.crates {
            c.validate()?;

            if c.already_published()? {
                continue;
            }

            c.submit(&self.token)?;
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

    fn already_published(&self) -> anyhow::Result<bool> {
        let theirs = {
            let name = format!("{self}");
            let search_result = cmd_lib::run_fun!(cargo search "${name}")
                .context("search for crate on crates.io")?;

            if search_result.is_empty() {
                log::info!("{self} has not been published yet");
                return Ok(true);
            }

            let version = cmd_lib::run_fun!(cargo search "${name}" | head -n1 | awk r#"{print $3}"# | tr -d '"')
                .context("search crates.io for published crate version")?;
            log::trace!("{self} found as {version} on their side");

            version
        };

        let ours = {
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
                .find(|p| p.name == name)
                .ok_or_else(|| anyhow!("could not find package"))?;

            let version = package.version.to_string();
            log::trace!("{self} found as {version} on our side");

            version
        };

        if ours == theirs {
            log::info!("{self} has already been published as {ours}");
            return Ok(false);
        }

        Ok(true)
    }

    fn submit(&self, token: &SecStr) -> anyhow::Result<&Self> {
        // todo run `cargo publish`
        // todo support a dry-run mode
        let _ = token;
        Ok(self)
    }
}

impl FromStr for Crate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Crate {
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
