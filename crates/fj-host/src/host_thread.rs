use std::thread::{self, JoinHandle};

use crossbeam_channel::{self, Receiver, Sender};
use fj_interop::processed_shape::ProcessedShape;
use fj_operations::shape_processor::ShapeProcessor;

use crate::{Error, HostCommand, Model, Watcher};

// Use a zero-sized error type to silence `#[warn(clippy::result_large_err)]`.
// The only error from `EventLoopProxy::send_event` is `EventLoopClosed<T>`,
// so we don't need the actual value. We just need to know there was an error.
pub(crate) struct EventLoopClosed;

pub(crate) struct HostThread {
    shape_processor: ShapeProcessor,
    model_event_tx: Sender<ModelEvent>,
    command_tx: Sender<HostCommand>,
    command_rx: Receiver<HostCommand>,
}

impl HostThread {
    // Spawn a background thread that will process models for an event loop.
    pub(crate) fn spawn(
        shape_processor: ShapeProcessor,
        event_loop_proxy: Sender<ModelEvent>,
    ) -> (Sender<HostCommand>, JoinHandle<Result<(), EventLoopClosed>>) {
        let (command_tx, command_rx) = crossbeam_channel::unbounded();
        let command_tx_2 = command_tx.clone();

        let host_thread = Self {
            shape_processor,
            model_event_tx: event_loop_proxy,
            command_tx,
            command_rx,
        };

        let join_handle = host_thread.spawn_thread();

        (command_tx_2, join_handle)
    }

    fn spawn_thread(mut self) -> JoinHandle<Result<(), EventLoopClosed>> {
        thread::Builder::new()
            .name("host".to_string())
            .spawn(move || -> Result<(), EventLoopClosed> {
                let mut model: Option<Model> = None;
                let mut _watcher: Option<Watcher> = None;

                while let Ok(command) = self.command_rx.recv() {
                    match command {
                        HostCommand::LoadModel(new_model) => {
                            // Right now, `fj-app` will only load a new model
                            // once. The gui does not have a feature to load a
                            // new model after the initial load. If that were
                            // to change, there would be a race condition here
                            // if the prior watcher sent `TriggerEvaluation`
                            // before it and the model were replaced.
                            match Watcher::watch_model(
                                new_model.watch_path(),
                                self.command_tx.clone(),
                            ) {
                                Ok(watcher) => {
                                    _watcher = Some(watcher);
                                    self.send_event(ModelEvent::StartWatching)?;
                                }

                                Err(err) => {
                                    self.send_event(ModelEvent::Error(err))?;
                                    continue;
                                }
                            }
                            self.process_model(&new_model)?;
                            model = Some(new_model);
                        }
                        HostCommand::TriggerEvaluation => {
                            self.send_event(ModelEvent::ChangeDetected)?;
                            if let Some(model) = &model {
                                self.process_model(model)?;
                            }
                        }
                    }
                }

                Ok(())
            })
            .expect("Cannot create OS thread for host")
    }

    // Evaluate and process a model.
    fn process_model(&mut self, model: &Model) -> Result<(), EventLoopClosed> {
        let evaluation = match model.evaluate() {
            Ok(evaluation) => evaluation,

            Err(err) => {
                self.send_event(ModelEvent::Error(err))?;
                return Ok(());
            }
        };

        self.send_event(ModelEvent::Evaluated)?;

        if let Some(warn) = evaluation.warning {
            self.send_event(ModelEvent::Warning(warn))?;
        }

        match self.shape_processor.process(&evaluation.shape) {
            Ok(shape) => self.send_event(ModelEvent::ProcessedShape(shape))?,

            Err(err) => {
                self.send_event(ModelEvent::Error(err.into()))?;
            }
        }

        Ok(())
    }

    // Send a message to the event loop.
    fn send_event(&mut self, event: ModelEvent) -> Result<(), EventLoopClosed> {
        self.model_event_tx
            .send(event)
            .map_err(|_| EventLoopClosed)?;

        Ok(())
    }
}

/// An event emitted by the host thread
#[derive(Debug)]
pub enum ModelEvent {
    /// A new model is being watched
    StartWatching,

    /// A change in the model has been detected
    ChangeDetected,

    /// The model has been evaluated
    Evaluated,

    /// The model has been processed
    ProcessedShape(ProcessedShape),

    /// A warning
    Warning(String),

    /// An error
    Error(Error),
}
