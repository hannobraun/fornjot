use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};

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
        let default_path = config
            .default_path
            .clone()
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

    pub fn default_path(&self) -> PathBuf {
        self.default_path.clone().unwrap_or_else(PathBuf::new)
    }

    pub fn model_path_without_default(&self) -> &Path {
        &self.model_path
    }

    pub fn path(&self) -> PathBuf {
        self.default_path
            .clone()
            .unwrap_or_else(PathBuf::new)
            .join(&self.model_path)
    }
}

fn no_model_error() -> anyhow::Error {
    anyhow!(
        "You must specify a model to start Fornjot.\n\
        - Pass a model as a command-line argument. See `fj-app --help`.\n\
        - Specify a default model in the configuration file."
    )
}
