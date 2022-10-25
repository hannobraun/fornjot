use std::thread;

use crossbeam_channel::{Receiver, Sender};

use crate::{Error, Model, WatcherEvent};

/// Evaluates a model in a background thread
pub struct Evaluator {
    trigger_tx: Sender<()>,
    event_rx: Receiver<WatcherEvent>,
}

impl Evaluator {
    /// Create an `Evaluator` from a model
    pub fn from_model(model: Model) -> Self {
        let (event_tx, event_rx) = crossbeam_channel::bounded(0);
        let (trigger_tx, trigger_rx) = crossbeam_channel::bounded(0);

        thread::spawn(move || loop {
            let () = trigger_rx
                .recv()
                .expect("Expected channel to never disconnect");

            let evaluation = match model.evaluate() {
                Ok(evaluation) => evaluation,
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
                .send(WatcherEvent::Evaluation(evaluation))
                .expect("Expected channel to never disconnect");
        });

        Self {
            event_rx,
            trigger_tx,
        }
    }

    /// Access a channel for triggering evaluations
    pub fn trigger(&self) -> Sender<()> {
        self.trigger_tx.clone()
    }

    /// Access a channel for receiving status updates
    pub fn events(&self) -> Receiver<WatcherEvent> {
        self.event_rx.clone()
    }
}
