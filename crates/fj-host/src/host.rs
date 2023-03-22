use std::thread::JoinHandle;

use crossbeam_channel::Sender;
use fj_operations::shape_processor::ShapeProcessor;

use crate::{EventLoopClosed, HostThread, Model, ModelEvent};

/// A host for watching models and responding to model updates
pub struct Host {
    command_tx: Sender<HostCommand>,
    host_thread: Option<JoinHandle<Result<(), EventLoopClosed>>>,
    model_loaded: bool,
}

impl Host {
    /// Create a host with a shape processor and a send channel to the event
    /// loop.
    pub fn new(
        shape_processor: ShapeProcessor,
        model_event_tx: Sender<ModelEvent>,
    ) -> Self {
        let (command_tx, host_thread) =
            HostThread::spawn(shape_processor, model_event_tx);

        Self {
            command_tx,
            host_thread: Some(host_thread),
            model_loaded: false,
        }
    }

    /// Send a model to the host for evaluation and processing.
    pub fn load_model(&mut self, model: Model) {
        self.command_tx
            .try_send(HostCommand::LoadModel(model))
            .expect("Host channel disconnected unexpectedly");
        self.model_loaded = true;
    }

    /// Whether a model has been sent to the host yet
    pub fn is_model_loaded(&self) -> bool {
        self.model_loaded
    }

    /// Check if the host thread has exited with a panic. This method runs at
    /// each tick of the event loop. Without an explicit check, an operation
    /// will appear to hang forever (e.g. processing a model).  An error
    /// will be printed to the terminal, but the gui will not notice until
    /// a new `HostCommand` is issued on the disconnected channel.
    ///
    /// # Panics
    ///
    /// This method panics on purpose so the main thread can exit on an
    /// unrecoverable error.
    pub fn propagate_panic(&mut self) {
        if self.host_thread.is_none() {
            unreachable!("Constructor requires host thread")
        }
        if let Some(host_thread) = &self.host_thread {
            // The host thread should not finish while this handle holds the
            // `command_tx` channel open, so an exit means the thread panicked.
            if host_thread.is_finished() {
                let host_thread = self.host_thread.take().unwrap();
                match host_thread.join() {
                    Ok(_) => {
                        unreachable!(
                            "Host thread cannot exit until host handle disconnects"
                        )
                    }
                    // The error value has already been reported by the panic
                    // in the host thread, so just ignore it here.
                    Err(_) => {
                        panic!("Host thread panicked")
                    }
                }
            }
        }
    }
}

/// Commands that can be sent to a host
pub enum HostCommand {
    /// Load a model to be evaluated and processed
    LoadModel(Model),
    /// Used by a `Watcher` to trigger evaluation when a model is edited
    TriggerEvaluation,
}
