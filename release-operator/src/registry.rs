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

    pub fn publish(&self) -> anyhow::Result<()> {
        for c in &self.crates {
            c.validate()?.preflight()?.submit()?;
        }

        Ok(())
    }
}

impl Crate {
    fn validate(&self) -> anyhow::Result<&Self> {
        // todo check if the path is readable
        // todo check if there is a Cargo.toml
        Ok(self)
    }

    fn preflight(&self) -> anyhow::Result<&Self> {
        // todo run `cargo search` to determine which version the registry knows
        //      compare the version to the local one and abort if they match
        Ok(self)
    }

    fn submit(&self) -> anyhow::Result<&Self> {
        // todo run `cargo publish`
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
