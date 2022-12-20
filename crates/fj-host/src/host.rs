use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};
use fj_operations::shape_processor::ShapeProcessor;
use winit::event_loop::EventLoopProxy;

use crate::{Error, Evaluator, Model, ModelEvent, Watcher};

/*
/// A Fornjot model host
pub struct Host {
    evaluator: Evaluator,
    _watcher: Watcher,
}

impl Host {
    /// Create a new instance of `Host`
    ///
    /// This is only useful, if you want to continuously watch the model for
    /// changes. If you don't, just keep using `Model`.
    pub fn from_model(model: Model) -> Result<Self, Error> {
        let watch_path = model.watch_path();
        let evaluator = Evaluator::from_model(model);
        let watcher = Watcher::watch_model(watch_path, &evaluator)?;

        Ok(Self {
            evaluator,
            _watcher: watcher,
        })
    }

    /*
    /// Access a channel with evaluation events
    pub fn events(&self) -> Receiver<ModelEvent> {
        self.evaluator.events()
    }
    */
}
*/

/// A Fornjot model host
pub struct ModelWatcher {
    //evaluator: Evaluator,
    model: Model,
    _watcher: Watcher,
}

impl ModelWatcher {
    /// Create a new instance of `Host`
    ///
    /// This is only useful, if you want to continuously watch the model for
    /// changes. If you don't, just keep using `Model`.
    pub fn from_model(
        host_tx: Sender<HostCommand>,
        model: Model,
    ) -> Result<Self, Error> {
        let watch_path = model.watch_path();
        //let evaluator = Evaluator::from_model(model);
        //let watcher = Watcher::watch_model(watch_path, &evaluator)?;
        let watcher = Watcher::watch_model(watch_path, host_tx)?;

        Ok(Self {
            model,
            _watcher: watcher,
        })
    }
}

/// ..
pub struct Host {
    event_loop_proxy: EventLoopProxy<ModelEvent>,
    shape_processor: ShapeProcessor,
    command_tx: Sender<HostCommand>,
    command_rx: Receiver<HostCommand>,
    //model_watcher: Option<ModelWatcher>,
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
            //model_watcher: None,
        }
    }

    fn evaluate_model(&self, model: &Model) {
        //if let Some(model_watcher) = &self.model_watcher {
        //let evaluation = match model_watcher
        let evaluation = match model.evaluate() {
            Ok(evaluation) => evaluation,

            Err(err) => {
                if let Err(EventLoopClosed) =
                    self.event_loop_proxy.send_event(ModelEvent::Error(err))
                {
                    panic!();
                }
                return;
            }
        };

        self.event_loop_proxy.send_event(ModelEvent::Evaluated).unwrap();

        let shape = self.shape_processor.process(&evaluation.shape).unwrap();

        if let Err(EventLoopClosed) = self
            .event_loop_proxy
            .send_event(ModelEvent::ProcessedShape(shape))
        {
            panic!();
        }
        //};
    }

    /// ..
    pub fn spawn(self) -> HostHandle {
        let command_tx_2 = self.command_tx.clone();

        thread::spawn(move || {
            let mut model_watcher: Option<ModelWatcher> = None;
            let mut t_model: Option<Model> = None;

            loop {
                while let Ok(command) = self.command_rx.recv() {
                    match command {
                        HostCommand::LoadModel(ref model) => {
                            tracing::warn!("LoadModel");
                            if let Err(EventLoopClosed) = self
                                .event_loop_proxy
                                .send_event(ModelEvent::StartWatching)
                            {
                                break;
                            }

                            model_watcher = Some(
                                ModelWatcher::from_model(
                                    self.command_tx.clone(),
                                    model.clone() ,
                                )
                                .unwrap(),
                            );

                            self.evaluate_model(&model);

                            t_model = Some(model.clone());
                        }
                        HostCommand::TriggerEvaluation => {
                            tracing::warn!("TriggerEvaluation");
                            if let Err(EventLoopClosed) = self
                                .event_loop_proxy
                                .send_event(ModelEvent::ChangeDetected)
                            {
                                break;
                            }

                            if let Some(model) = &t_model {
                                self.evaluate_model(&model);
                            }
                        }
                    }
                }
            }
        });

        HostHandle {
            command_tx: command_tx_2,
            model_available: false,
        }
    }
}

pub enum HostCommand {
    LoadModel(Model),
    TriggerEvaluation,
}

/// ..
pub struct HostHandle {
    command_tx: Sender<HostCommand>,
    model_available: bool,
}

impl HostHandle {
    /// ..
    pub fn load_model(&mut self, model: Model) {
        self.model_available = true;
        self.command_tx.send(HostCommand::LoadModel(model)).unwrap();
    }

    /// ..
    pub fn is_model_available(&self) -> bool {
        self.model_available
    }
}
