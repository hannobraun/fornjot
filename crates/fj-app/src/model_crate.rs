use include_dir::{include_dir, Dir};
use std::{fs, path::Path};

static MODEL_TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/model-template");

pub fn create(model_name: &str) -> anyhow::Result<()> {
    let path = Path::new(model_name);
    fs::create_dir_all(path)?;
    MODEL_TEMPLATE.extract(path)?;
    post_process_model_files(path, model_name)?;
    println!("Model '{model_name}' created");
    Ok(())
}

fn post_process_model_files(
    path: &Path,
    model_name: &str,
) -> anyhow::Result<()> {
    replace_in_file(
        &path.join("Cargo.toml"),
        [
            (
                "name = \"model-template\"".to_string(),
                format!("name = \"{model_name}\""),
            ),
            (
                r#"path = "../../fj""#.to_owned(),
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
