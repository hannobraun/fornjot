use std::{
    fmt::{self, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};
use fj_host::{Model, Parameters};

use crate::{args::Args, config::Config};

pub struct ModelPath {
    default_path: Option<PathBuf>,
    model_path: ModelPathSource,
}

impl ModelPath {
    pub fn from_args_and_config(
        args: &Args,
        config: &Config,
    ) -> anyhow::Result<Self> {
        let default_path = config.default_path.clone();

        let model_path_from_args = args
            .model
            .as_ref()
            .map(|model| ModelPathSource::Args(model.clone()));
        let model_path_from_config = config
            .default_model
            .as_ref()
            .map(|model| ModelPathSource::Config(model.clone()));
        let model_path = model_path_from_args
            .or(model_path_from_config)
            .ok_or_else(no_model_error)?;

        Ok(Self {
            default_path,
            model_path,
        })
    }

    pub fn load_model(&self, parameters: Parameters) -> anyhow::Result<Model> {
        let default_path = self
            .default_path
            .as_ref()
            .map(|path| -> anyhow::Result<_> {
                let rel = path;
                let abs = path.canonicalize().with_context(|| {
                    format!(
                        "Converting `default-path` from `fj.toml` (`{}`) into \
                        absolute path",
                        path.display(),
                    )
                })?;
                Ok((rel, abs))
            })
            .transpose()?;

        let path = default_path
            .clone()
            .map(|(_, abs)| abs)
            .unwrap_or_else(PathBuf::new)
            .join(self.model_path.path());

        let model = Model::new(&path, parameters).with_context(|| {
            load_error_context(default_path, &self.model_path, path)
        })?;
        Ok(model)
    }
}

enum ModelPathSource {
    Args(PathBuf),
    Config(PathBuf),
}

impl ModelPathSource {
    fn path(&self) -> &Path {
        match self {
            ModelPathSource::Args(path) => path,
            ModelPathSource::Config(path) => path,
        }
    }
}

fn load_error_context(
    default_path: Option<(&PathBuf, PathBuf)>,
    model_path: &ModelPathSource,
    path: PathBuf,
) -> String {
    load_error_context_inner(default_path, model_path, path)
        .expect("Expected `write!` to `String` to never fail")
}

fn load_error_context_inner(
    default_path: Option<(&PathBuf, PathBuf)>,
    model_path: &ModelPathSource,
    path: PathBuf,
) -> Result<String, fmt::Error> {
    let mut error = String::new();
    write!(
        error,
        "Failed to load model: `{}`",
        model_path.path().display()
    )?;
    write!(error, "\n- Path of model: {}", path.display())?;

    let mut suggestions = String::new();
    write!(suggestions, "Suggestions:")?;
    write!(
        suggestions,
        "\n- Did you mis-type the model path `{}`?",
        model_path.path().display()
    )?;

    if let Some((default_path_rel, default_path_abs)) = &default_path {
        write!(
            error,
            "\n- Searching inside default path from configuration: {}",
            default_path_abs.display(),
        )?;

        write!(
            suggestions,
            "\n- Did you mis-type the default path `{}`?",
            default_path_rel.display()
        )?;
        write!(
            suggestions,
            "\n- Did you accidentally pick up a local configuration file?"
        )?;
    }

    let context = format!("{error}\n\n{suggestions}");

    Ok(context)
}

fn no_model_error() -> anyhow::Error {
    anyhow!(
        "You must specify a model to start Fornjot.\n\
        - Pass a model as a command-line argument. See `fj-app --help`.\n\
        - Specify a default model in the configuration file."
    )
}
