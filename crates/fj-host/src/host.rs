use std::thread;

use crossbeam_channel::{self, Receiver, Sender};
use fj_interop::processed_shape::ProcessedShape;
use fj_operations::shape_processor::ShapeProcessor;
use winit::event_loop::EventLoopProxy;

use crate::{Error, HostCommand, HostHandle, Model, Watcher};

// Use a zero-sized error type to silence `#[warn(clippy::result_large_err)]`.
// The only error from `EventLoopProxy::send_event` is `EventLoopClosed<T>`,
// so we don't need the actual value. We just need to know there was an error.
/// Error type for sending on a channel with a closed event loop
pub struct EventLoopClosed;

/// A Fornjot model host that runs in the background
pub struct Host {
    shape_processor: ShapeProcessor,
    event_loop_proxy: EventLoopProxy<ModelEvent>,
    command_tx: Sender<HostCommand>,
    command_rx: Receiver<HostCommand>,
}

impl Host {
    /// Create a new `Host` that will process models for an event loop.
    pub fn new(
        shape_processor: ShapeProcessor,
        event_loop_proxy: EventLoopProxy<ModelEvent>,
    ) -> Self {
        let (command_tx, command_rx) = crossbeam_channel::unbounded();
        Self {
            shape_processor,
            event_loop_proxy,
            command_tx,
            command_rx,
        }
    }

    /// Run a background thread to watch for updates and process a model.
    pub fn spawn(mut self) -> HostHandle {
        let command_tx = self.command_tx.clone();

        let host_thread = thread::Builder::new()
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
            .expect("Cannot create OS thread for host");

        HostHandle::new(command_tx, host_thread)
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
        self.event_loop_proxy
            .send_event(event)
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

    /// An error
    Error(Error),
}
