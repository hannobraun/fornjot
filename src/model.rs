use std::{collections::HashMap, io, path::PathBuf, process::Command};

use thiserror::Error;

pub struct Model {
    name: String,
}

impl Model {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn path(&self) -> PathBuf {
        let mut path = PathBuf::from("models");
        path.push(self.name());
        path
    }

    pub fn src_path(&self) -> PathBuf {
        self.path().join("src")
    }

    pub fn lib_path(&self) -> PathBuf {
        let name = self.name().replace('-', "_");

        let file = if cfg!(windows) {
            format!("{}.dll", name)
        } else if cfg!(target_os = "macos") {
            format!("lib{}.dylib", name)
        } else {
            //Unix
            format!("lib{}.so", name)
        };

        self.path().join("target/debug").join(file)
    }

    pub fn load(
        &self,
        arguments: &HashMap<String, String>,
    ) -> Result<fj::Shape, Error> {
        let manifest_path =
            self.path().join("Cargo.toml").display().to_string();

        let status = Command::new("cargo")
            .arg("build")
            .args(["--manifest-path", &manifest_path])
            .status()?;

        if !status.success() {
            return Err(Error::Compile);
        }

        // So, strictly speaking this is all unsound:
        // - `Library::new` requires us to abide by the arbitrary requirements
        //   of any library initialization or termination routines.
        // - `Library::get` requires us to specify the correct type for the
        //   model function.
        // - The model function itself is `unsafe`, because it is a function
        //   from across an FFI interface.
        //
        // Typical models won't have initialization or termination routines (I
        // think), should abide by the `ModelFn` signature, and might not do
        // anything unsafe. But we have no way to know that the library the user
        // told us to load actually does (I think).
        //
        // I don't know of a way to fix this. We should take this as motivation
        // to switch to a better technique:
        // https://github.com/hannobraun/Fornjot/issues/71
        let shape = unsafe {
            let lib = libloading::Library::new(self.lib_path())?;
            let model: libloading::Symbol<ModelFn> = lib.get(b"model")?;
            model(arguments)
        };

        Ok(shape)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error compiling model")]
    Compile,

    #[error("I/O error while loading model")]
    Io(#[from] io::Error),

    #[error("Error loading model from dynamic library")]
    LibLoading(#[from] libloading::Error),
}

type ModelFn =
    unsafe extern "C" fn(args: &HashMap<String, String>) -> fj::Shape;
