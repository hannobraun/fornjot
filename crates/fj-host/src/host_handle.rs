use std::thread::JoinHandle;

use crossbeam_channel::Sender;

use crate::{Error, Model};

/// A handle for sending commands to a spawned host
pub struct HostHandle {
    command_tx: Sender<HostCommand>,
    host_thread: Option<JoinHandle<()>>,
    model_loaded: bool,
}

impl HostHandle {
    /// Create a `HostHandle` with a channel to send commands to.
    pub fn new(
        command_tx: Sender<HostCommand>,
        host_thread: JoinHandle<()>,
    ) -> Self {
        Self {
            command_tx,
            host_thread: Some(host_thread),
            model_loaded: false,
        }
    }

    /// Send a model to the host for evluation.
    pub fn load_model(&mut self, model: Model) -> Result<(), Error> {
        self.command_tx
            .try_send(HostCommand::LoadModel(model))
            .map_err(|_| Error::HostChannel)?;
        self.model_loaded = true;

        Ok(())
    }

    /// Whether a model has been sent to a host for watching and evaluation
    pub fn is_model_loaded(&self) -> bool {
        self.model_loaded
    }

    /// Panic if the host thread has panicked.
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
                    Ok(()) => {
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
