use std::process::Command;

pub struct Model {
    name: String,
}

impl Model {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> String {
        format!("models/{}", self.name)
    }

    pub fn build(&self) -> anyhow::Result<()> {
        // This can be made a bit more contact using `ExitStatus::exit_ok`, once
        // that is stable.
        let status = Command::new("cargo")
            .arg("build")
            .args(["--manifest-path", &format!("{}/Cargo.toml", self.path())])
            .status()?;
        assert!(status.success());

        Ok(())
    }
}
