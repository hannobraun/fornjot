use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};
use fj_operations::shape_processor::ShapeProcessor;
use winit::event_loop::{EventLoopClosed, EventLoopProxy};

use crate::{Error, Model, ModelEvent, Watcher};

struct ModelWatcher {
    _watcher: Watcher,
}

impl ModelWatcher {
    fn from_model(
        host_tx: Sender<HostCommand>,
        model: Model,
    ) -> Result<Self, Error> {
        let watch_path = model.watch_path();
        let watcher = Watcher::watch_model(watch_path, host_tx)?;

        Ok(Self {
            _watcher: watcher,
        })
    }
}

/// A Fornjot model host
pub struct Host {
    event_loop_proxy: EventLoopProxy<ModelEvent>,
    shape_processor: ShapeProcessor,
    command_tx: Sender<HostCommand>,
    command_rx: Receiver<HostCommand>,
}

impl Host {
    /// ..
    pub fn new(
        shape_processor: ShapeProcessor,
        proxy: EventLoopProxy<ModelEvent>,
    ) -> Self {
        let (command_tx, command_rx) = unbounded();
        Self {
            event_loop_proxy: proxy,
            shape_processor,
            command_tx,
            command_rx,
        }
    }

    fn evaluate_model(&self, model: &Model) {
        let evaluation = match model.evaluate() {
            Ok(evaluation) => evaluation,

            Err(err) => {
                if let Err(EventLoopClosed(..)) =
                    self.event_loop_proxy.send_event(ModelEvent::Error(err))
                {
                    panic!();
                }
                return;
            }
        };

        self.event_loop_proxy
            .send_event(ModelEvent::Evaluated)
            .unwrap();

        let shape = self.shape_processor.process(&evaluation.shape).unwrap();

        if let Err(EventLoopClosed(..)) = self
            .event_loop_proxy
            .send_event(ModelEvent::ProcessedShape(shape))
        {
            panic!();
        }
    }

    /// ..
    pub fn spawn(self) -> HostHandle {
        let host_handle = HostHandle {
            command_tx: self.command_tx.clone(),
            model_loaded: false,
        };

        thread::spawn(move || {
            let mut _model_watcher: Option<ModelWatcher> = None;
            let mut t_model: Option<Model> = None;

            loop {
                while let Ok(command) = self.command_rx.recv() {
                    match command {
                        HostCommand::LoadModel(ref model) => {
                            tracing::warn!("LoadModel");
                            if let Err(EventLoopClosed(..)) = self
                                .event_loop_proxy
                                .send_event(ModelEvent::StartWatching)
                            {
                                break;
                            }

                            _model_watcher = Some(
                                ModelWatcher::from_model(
                                    self.command_tx.clone(),
                                    model.clone(),
                                )
                                .unwrap(),
                            );

                            self.evaluate_model(model);

                            t_model = Some(model.clone());
                        }
                        HostCommand::TriggerEvaluation => {
                            tracing::warn!("TriggerEvaluation");
                            if let Err(EventLoopClosed(..)) = self
                                .event_loop_proxy
                                .send_event(ModelEvent::ChangeDetected)
                            {
                                break;
                            }

                            if let Some(model) = &t_model {
                                self.evaluate_model(model);
                            }
                        }
                    }
                }
            }
        });

        host_handle
    }
}

/// Commands that can be sent to a host
pub enum HostCommand {
    LoadModel(Model),
    TriggerEvaluation,
}

/// A handle to send commands to a host
pub struct HostHandle {
    command_tx: Sender<HostCommand>,
    model_loaded: bool,
}

impl HostHandle {
    /// Send a model to the host for evluation.
    pub fn load_model(&mut self, model: Model) -> Result<(), Error> {
        self.command_tx
            .try_send(HostCommand::LoadModel(model))
            .map_err(|_| Error::HostChannel)?;
        self.model_loaded = true;

        Ok(())
    }

    /// Whether a model has been successfully sent to the host
    pub fn is_model_loaded(&self) -> bool {
        self.model_loaded
    }
}
