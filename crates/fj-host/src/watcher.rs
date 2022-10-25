use std::{collections::HashSet, ffi::OsStr, thread};

use crossbeam_channel::Receiver;
use notify::Watcher as _;

use crate::{Error, Evaluation, Model};

/// Watches a model for changes, reloading it continually
pub struct Watcher {
    _watcher: Box<dyn notify::Watcher>,
    event_rx: Receiver<WatcherEvent>,
}

impl Watcher {
    /// Watch the provided model for changes
    pub fn watch_model(model: Model) -> Result<Self, Error> {
        let (event_tx, event_rx) = crossbeam_channel::bounded(0);

        let (watch_tx, watch_rx) = crossbeam_channel::bounded(0);
        let watch_tx_2 = watch_tx.clone();

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
                    watch_tx.send(()).expect("Channel is disconnected");
                }
            },
        )?;

        watcher.watch(&watch_path, notify::RecursiveMode::Recursive)?;

        // To prevent a race condition between the initial load and the start of
        // watching, we'll trigger the initial load here, after having started
        // watching.
        //
        // This happens in a separate thread, because the channel is bounded and
        // has no buffer.
        //
        // Will panic, if the receiving end has panicked. Not much we can do
        // about that, if it happened.
        thread::spawn(move || {
            watch_tx_2.send(()).expect("Channel is disconnected")
        });

        // Listen on the watcher channel and rebuild the model. This happens in
        // a separate thread from the watcher to allow us to trigger compiles
        // without the watcher having registered a change, as is done above.
        thread::spawn(move || loop {
            let () = watch_rx
                .recv()
                .expect("Expected channel to never disconnect");

            let shape = match model.evaluate() {
                Ok(shape) => shape,
                Err(Error::Compile { output }) => {
                    event_tx
                        .send(WatcherEvent::StatusUpdate(format!(
                            "Failed to compile model:\n{}",
                            output
                        )))
                        .expect("Expected channel to never disconnect");

                    return;
                }
                Err(err) => {
                    event_tx
                        .send(WatcherEvent::Error(err))
                        .expect("Expected channel to never disconnect");
                    return;
                }
            };

            event_tx
                .send(WatcherEvent::Shape(shape))
                .expect("Expected channel to never disconnect");
        });

        Ok(Self {
            _watcher: Box::new(watcher),
            event_rx,
        })
    }

    /// Access a channel for receiving status updates
    pub fn events(&self) -> Receiver<WatcherEvent> {
        self.event_rx.clone()
    }
}

/// An event emitted by the [`Watcher`]
pub enum WatcherEvent {
    /// A status update about the model
    StatusUpdate(String),

    /// A shape has been loaded from the model
    Shape(Evaluation),

    /// An error
    Error(Error),
}
