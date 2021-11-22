use std::{collections::HashMap, io, path::PathBuf, process::Command};

use thiserror::Error;

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

    pub fn src_path(&self) -> PathBuf {
        format!("{}/src", self.path()).into()
    }

    pub fn load(
        &self,
        arguments: &HashMap<String, String>,
    ) -> Result<fj::Shape, Error> {
        // This can be made a bit more compact using `ExitStatus::exit_ok`, once
        // that is stable.
        let status = Command::new("cargo")
            .arg("build")
            .args(["--manifest-path", &format!("{}/Cargo.toml", self.path())])
            .status()?;
        assert!(status.success());

        // TASK: Read up why those calls are unsafe. Make sure calling them is
        //       sound, and document why that is.
        let shape = unsafe {
            let lib = libloading::Library::new(format!(
                "{}/target/debug/lib{}.so",
                self.path(),
                self.name(),
            ))?;
            let model: libloading::Symbol<ModelFn> = lib.get(b"model")?;
            model(&arguments)
        };

        Ok(shape)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error while loading model")]
    Io(#[from] io::Error),

    #[error("Error loading model from dynamic library")]
    LibLoading(#[from] libloading::Error),
}

type ModelFn =
    unsafe extern "C" fn(args: &HashMap<String, String>) -> fj::Shape;
