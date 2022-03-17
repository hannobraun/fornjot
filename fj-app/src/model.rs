use std::{collections::HashMap, io, path::PathBuf, process::Command};

use thiserror::Error;

pub struct Model {
    src_path: PathBuf,
    lib_path: PathBuf,
    manifest_path: PathBuf,
}

impl Model {
    pub fn from_path(
        path: PathBuf,
        target_dir: Option<PathBuf>,
    ) -> io::Result<Self> {
        let name = {
            // Can't panic. It only would, if the path ends with "..", and we
            // are canonicalizing it here to prevent that.
            let canonical = path.canonicalize()?;
            let file_name = canonical.file_name().unwrap();

            file_name.to_string_lossy().replace('-', "_")
        };

        let src_path = path.join("src");

        let lib_path = {
            let file = if cfg!(windows) {
                format!("{}.dll", name)
            } else if cfg!(target_os = "macos") {
                format!("lib{}.dylib", name)
            } else {
                //Unix
                format!("lib{}.so", name)
            };

            let target_dir = target_dir.unwrap_or_else(|| path.join("target"));
            target_dir.join("debug").join(file)
        };

        let manifest_path = path.join("Cargo.toml");

        Ok(Self {
            src_path,
            lib_path,
            manifest_path,
        })
    }

    pub fn src_path(&self) -> PathBuf {
        self.src_path.clone()
    }

    pub fn load(
        &self,
        arguments: &HashMap<String, String>,
    ) -> Result<fj::Shape, Error> {
        let manifest_path = self.manifest_path.display().to_string();

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
            let lib = libloading::Library::new(&self.lib_path)?;
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
