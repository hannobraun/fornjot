use std::{collections::HashSet, ffi::OsStr, thread};

use crossbeam_channel::{Receiver, TryRecvError};
use fj_interop::status_report::StatusReport;
use notify::Watcher as _;

use crate::{Error, Model};

/// Watches a model for changes, reloading it continually
pub struct Watcher {
    _watcher: Box<dyn notify::Watcher>,
    channel: Receiver<()>,
    model: Model,
}

impl Watcher {
    /// Watch the provided model for changes
    pub fn watch_model(model: Model) -> Result<Self, Error> {
        let (tx, rx) = crossbeam_channel::bounded(0);
        let tx2 = tx.clone();

        let watch_path = model.src_path();

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
                    // Either way, not much we can do about it here.
                    tx.send(()).expect("Channel is disconnected");
                }
            },
        )?;

        watcher.watch(&watch_path, notify::RecursiveMode::Recursive)?;

        // To prevent a race condition between the initial load and the start of
        // watching, we'll trigger the initial load here, after having started
        // watching.
        //
        // Will panic, if the receiving end has panicked. Not much we can do
        // about that, if it happened.
        thread::spawn(move || tx2.send(()).expect("Channel is disconnected"));

        Ok(Self {
            _watcher: Box::new(watcher),
            channel: rx,
            model,
        })
    }

    /// Receive an updated shape that the reloaded model created
    ///
    /// Returns `None`, if the model has not changed since the last time this
    /// method was called.
    pub fn receive_shape(
        &self,
        status: &mut StatusReport,
    ) -> Result<Option<fj::Shape>, Error> {
        match self.channel.try_recv() {
            Ok(()) => {
                let shape = match self.model.load(status) {
                    Ok(shape) => shape,
                    Err(Error::Compile { output }) => {
                        status.clear_status();
                        status.update_status(&format!(
                            "Failed to compile model:\n{}",
                            output
                        ));

                        return Ok(None);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                };

                Ok(Some(shape))
            }
            Err(TryRecvError::Empty) => {
                // Nothing to receive from the channel.
                Ok(None)
            }
            Err(TryRecvError::Disconnected) => {
                // The other end has disconnected. This is probably the result
                // of a panic on the other thread, or a program shutdown in
                // progress. In any case, not much we can do here.
                panic!();
            }
        }
    }
}
