use std::{fs, path::Path};
use tar::Archive;

static NEW_MODEL_TEMPLATE: &str = "star";

static NEW_MODEL_TAR: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/new_model.tar"));

pub fn create(model_name: &str) -> anyhow::Result<()> {
    let path = Path::new(model_name);
    Archive::new(NEW_MODEL_TAR).unpack(path)?;
    postprocess_model_files(path, model_name)?;
    println!("Model '{model_name}' created");
    Ok(())
}

fn postprocess_model_files(
    path: &Path,
    model_name: &str,
) -> anyhow::Result<()> {
    replace_in_file(
        &path.join("Cargo.toml"),
        [
            (
                format!("name = \"{NEW_MODEL_TEMPLATE}\""),
                format!("name = \"{model_name}\""),
            ),
            (
                r#"path = "../../crates/fj""#.to_owned(),
                ["version = \"", &fj::version::VERSION_PKG.to_string(), "\""]
                    .concat(),
            ),
        ],
    )?;
    fs::write(path.join("README.md"), format!("# {model_name}\n"))?;
    Ok(())
}

fn replace_in_file(
    path: &Path,
    replacements: impl IntoIterator<Item = (String, String)>,
) -> anyhow::Result<()> {
    let mut content = fs::read_to_string(path)?;
    for (from, to) in replacements {
        content = content.replace(&from, &to);
    }
    fs::write(path, content)?;
    Ok(())
}
