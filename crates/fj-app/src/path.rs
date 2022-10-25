use std::{fmt::Write, path::PathBuf};

use anyhow::{anyhow, Context};
use fj_host::{Model, Parameters};

use crate::{args::Args, config::Config};

pub struct ModelPath {
    default_path: Option<PathBuf>,
    model_path: PathBuf,
}

impl ModelPath {
    pub fn from_args_and_config(
        args: &Args,
        config: &Config,
    ) -> anyhow::Result<Self> {
        let default_path = config.default_path.clone();
        let model_path = args
            .model
            .as_ref()
            .or(config.default_model.as_ref())
            .ok_or_else(no_model_error)?
            .clone();

        Ok(Self {
            default_path,
            model_path,
        })
    }

    pub fn load_model(&self, parameters: Parameters) -> anyhow::Result<Model> {
        let default_path = self
            .default_path
            .as_ref()
            .map(|path| {
                path.canonicalize().with_context(|| {
                    format!(
                        "Converting `default-path` from `fj.toml` (`{}`) into \
                        absolute path",
                        path.display(),
                    )
                })
            })
            .transpose()?;

        let path = default_path
            .clone()
            .unwrap_or_else(PathBuf::new)
            .join(&self.model_path);

        let mut error = String::new();
        write!(
            error,
            "Failed to load model: `{}`",
            self.model_path.display()
        )?;
        write!(error, "\n- Path of model: {}", path.display())?;

        let mut suggestions = String::new();
        write!(suggestions, "Suggestions:")?;
        write!(
            suggestions,
            "\n- Did you mis-type the model path `{}`?",
            self.model_path.display()
        )?;

        if let Some(default_path) = &default_path {
            write!(
                error,
                "\n- Searching inside default path from configuration: {}",
                default_path.display(),
            )?;

            write!(suggestions, "\n- Did you mis-type the default path?")?;
            write!(
                suggestions,
                "\n- Did you accidentally pick up a local configuration file?"
            )?;
        }

        let model = Model::new(&path, parameters)
            .with_context(|| format!("{error}\n\n{suggestions}"))?;
        Ok(model)
    }
}

fn no_model_error() -> anyhow::Error {
    anyhow!(
        "You must specify a model to start Fornjot.\n\
        - Pass a model as a command-line argument. See `fj-app --help`.\n\
        - Specify a default model in the configuration file."
    )
}
