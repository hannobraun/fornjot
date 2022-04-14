use std::{fs, process::Command};

use anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    for model in fs::read_dir("models")? {
        let model = model?;
        let model = model.file_name().into_string().map_err(|err| {
            anyhow!("Failed to convert directory name to `String`: {:?}", err)
        })?;

        let export_file = format!("{model}.3mf");

        let exit_status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(["--model", &model])
            .args(["--export", &export_file])
            .status()?;

        if !exit_status.success() {
            bail!(
                "Exporting model `{model}` failed with error code:\
                {exit_status}"
            );
        }
    }

    Ok(())
}
