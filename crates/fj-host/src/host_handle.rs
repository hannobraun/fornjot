use crossbeam_channel::Sender;

use crate::{Error, Model};

/// A handle for sending commands to a spawned host
pub struct HostHandle {
    command_tx: Sender<HostCommand>,
    model_loaded: bool,
}

impl HostHandle {
    /// Create a `HostHandle` with a channel to send commands to.
    pub fn new(command_tx: Sender<HostCommand>) -> Self {
        Self {
            command_tx,
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
}

/// Commands that can be sent to a host
pub enum HostCommand {
    /// Load a model to be evaluated and processed
    LoadModel(Model),
    /// Used by a `Watcher` to trigger evaluation when a model is edited
    TriggerEvaluation,
}
