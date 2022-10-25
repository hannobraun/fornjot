use std::thread;

use crossbeam_channel::{Receiver, Sender};

use crate::{Error, Evaluation, Model};

/// Evaluates a model in a background thread
pub struct Evaluator {
    trigger_tx: Sender<()>,
    event_rx: Receiver<ModelEvent>,
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
                        .send(ModelEvent::StatusUpdate(format!(
                            "Failed to compile model:\n{}",
                            output
                        )))
                        .expect("Expected channel to never disconnect");

                    return;
                }
                Err(err) => {
                    event_tx
                        .send(ModelEvent::Error(err))
                        .expect("Expected channel to never disconnect");
                    return;
                }
            };

            event_tx
                .send(ModelEvent::Evaluation(evaluation))
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
    pub fn events(&self) -> Receiver<ModelEvent> {
        self.event_rx.clone()
    }
}

/// An event emitted by [`Evaluator`]
pub enum ModelEvent {
    /// A status update about the model
    StatusUpdate(String),

    /// The model has been evaluated
    Evaluation(Evaluation),

    /// An error
    Error(Error),
}
