use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};
use fj_operations::shape_processor::ShapeProcessor;
use winit::event_loop::EventLoopProxy;

use crate::{HostCommand, HostHandle, Model, ModelEvent, Watcher};

// Use a zero-sized error type to silence `#[warn(clippy::result_large_err)]`.
// The only error from `EventLoopProxy::send_event` is `EventLoopClosed<T>`,
// so don't need the actual value, just to know that there was an error.
/// Error type for sending on a channel to a closed event loop
pub struct EventLoopClosed;

/// A Fornjot model host that runs in the background
pub struct Host {
    shape_processor: ShapeProcessor,
    event_loop_proxy: EventLoopProxy<ModelEvent>,
    command_tx: Sender<HostCommand>,
    command_rx: Receiver<HostCommand>,
}

impl Host {
    ///
    pub fn new(
        shape_processor: ShapeProcessor,
        event_loop_proxy: EventLoopProxy<ModelEvent>,
    ) -> Self {
        let (command_tx, command_rx) = unbounded();
        Self {
            shape_processor,
            event_loop_proxy,
            command_tx,
            command_rx,
        }
    }

    /// Run a background thread to watch for updates and evaluate a model.
    pub fn spawn(mut self) -> HostHandle {
        let command_tx = self.command_tx.clone();

        let host_thread = thread::Builder::new()
            .name("host".to_string())
            .spawn(move || -> Result<(), EventLoopClosed> {
                let mut _watcher: Option<Watcher> = None;
                let mut model: Option<Model> = None;

                while let Ok(command) = self.command_rx.recv() {
                    match command {
                        HostCommand::LoadModel(new_model) => {
                            self.send_event(ModelEvent::StartWatching)?;

                            _watcher = Some(
                                Watcher::watch_model(
                                    new_model.watch_path(),
                                    self.command_tx.clone(),
                                )
                                .unwrap(),
                            );

                            self.evaluate_model(&new_model)?;

                            model = Some(new_model);
                        }
                        HostCommand::TriggerEvaluation => {
                            self.send_event(ModelEvent::ChangeDetected)?;

                            if let Some(model) = &model {
                                self.evaluate_model(model)?;
                            }
                        }
                    }
                }

                Ok(())
            })
            .expect("Cannot create OS thread for host");

        HostHandle::new(command_tx, host_thread)
    }

    fn evaluate_model(&mut self, model: &Model) -> Result<(), EventLoopClosed> {
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
