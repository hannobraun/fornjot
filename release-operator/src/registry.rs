use crate::Crate;
use secstr::SecStr;

pub struct Registry {
    token: SecStr,
    crates: Vec<Crate>,
}

impl Registry {
    pub fn new(token: &SecStr, crates: &[Crate]) -> Self {
        Self {
            token: token.to_owned(),
            crates: crates.to_owned(),
        }
    }

    pub fn publish(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
