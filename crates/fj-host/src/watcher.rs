use std::{collections::HashSet, ffi::OsStr, path::Path};

use crossbeam_channel::Sender;
use notify::Watcher as _;

use crate::{Error, HostCommand};

/// Watches a model for changes, reloading it continually
pub struct Watcher {
    _watcher: Box<dyn notify::Watcher>,
}

impl Watcher {
    /// Watch the provided model for changes
    pub fn watch_model(
        watch_path: impl AsRef<Path>,
        host_tx: Sender<HostCommand>,
    ) -> Result<Self, Error> {
        let watch_path = watch_path.as_ref();

        let mut watcher = notify::recommended_watcher(
            move |event: notify::Result<notify::Event>| {
                // Unfortunately the `notify` documentation doesn't say when
                // this might happen, so no idea if it needs to be handled.
                let event = event.expect("Error handling watch event");

                // Various acceptable ModifyKind kinds. Varies across platforms
                // (e.g. MacOs vs. Windows10)
                if let notify::EventKind::Modify(
                    notify::event::ModifyKind::Any
                    | notify::event::ModifyKind::Data(
                        notify::event::DataChange::Any
                        | notify::event::DataChange::Content,
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
                    // Either way, not much we can do about it here.
                    host_tx
                        .send(HostCommand::TriggerEvaluation)
                        .expect("Channel is disconnected");
                }
            },
        )?;

        watcher.watch(watch_path, notify::RecursiveMode::Recursive)?;

        Ok(Self {
            _watcher: Box::new(watcher),
        })
    }
}
