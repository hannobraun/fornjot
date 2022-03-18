use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    io,
    path::PathBuf,
    process::Command,
    sync::mpsc,
};

use notify::Watcher as _;
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

    pub fn watch(
        self,
        parameters: HashMap<String, String>,
    ) -> Result<Watcher, Error> {
        let (tx, rx) = mpsc::sync_channel(0);
        let watch_path = self.src_path.clone();

        let mut watcher = notify::recommended_watcher(
            move |event: notify::Result<notify::Event>| {
                // Unfortunately the `notify` documentation doesn't say when
                // this might happen, so no idea if it needs to be handled.
                let event = event.expect("Error handling watch event");

                // Various acceptable ModifyKind kinds. Varies across platforms
                // (e.g. MacOs vs. Windows10)
                if let notify::EventKind::Modify(
                    notify::event::ModifyKind::Any,
                )
                | notify::EventKind::Modify(
                    notify::event::ModifyKind::Data(
                        notify::event::DataChange::Any,
                    ),
                )
                | notify::EventKind::Modify(
                    notify::event::ModifyKind::Data(
                        notify::event::DataChange::Content,
                    ),
                ) = event.kind
                {
                    let file_ext = event
                        .paths
                        .get(0)
                        .expect("File path missing in watch event")
                        .extension();

                    let black_list = HashSet::from([
                        OsStr::new("swp"),
                        OsStr::new("tmp"),
                        OsStr::new("swx"),
                    ]);

                    if let Some(ext) = file_ext {
                        if black_list.contains(ext) {
                            return;
                        }
                    }

                    // This will panic, if the other end is disconnected, which
                    // is probably the result of a panic on that thread, or the
                    // application is being shut down.
                    //
                    // Either way, not much we can do about it here, except
                    // maybe to provide a better error message in the future.
                    tx.send(()).unwrap();
                }
            },
        )?;

        watcher.watch(&watch_path, notify::RecursiveMode::Recursive)?;

        Ok(Watcher {
            _watcher: Box::new(watcher),
            channel: rx,
            model: self,
            parameters,
        })
    }
}

pub struct Watcher {
    _watcher: Box<dyn notify::Watcher>,
    channel: mpsc::Receiver<()>,
    model: Model,
    parameters: HashMap<String, String>,
}

impl Watcher {
    pub fn receive(&self) -> Option<fj::Shape> {
        match self.channel.try_recv() {
            Ok(()) => {
                let shape = match self.model.load(&self.parameters) {
                    Ok(shape) => shape,
                    Err(Error::Compile) => {
                        // It would be better to display an error in the UI,
                        // where the user can actually see it. Issue:
                        // https://github.com/hannobraun/fornjot/issues/30
                        println!("Error compiling model");
                        return None;
                    }
                    Err(err) => {
                        panic!("Error reloading model: {:?}", err);
                    }
                };

                Some(shape)
            }
            Err(mpsc::TryRecvError::Empty) => {
                // Nothing to receive from the channel.
                None
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                // The other end has disconnected. This is probably the result
                // of a panic on the other thread, or a program shutdown in
                // progress. In any case, not much we can do here.
                panic!();
            }
        }
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

    #[error("Error watching model for changes")]
    Notify(#[from] notify::Error),
}

type ModelFn =
    unsafe extern "C" fn(args: &HashMap<String, String>) -> fj::Shape;
