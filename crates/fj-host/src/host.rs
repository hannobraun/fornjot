use std::thread::JoinHandle;

use fj_operations::shape_processor::ShapeProcessor;
use winit::event_loop::EventLoopProxy;

use crate::{spawn_evaluator, Error, Model, ModelEvent, Watcher};

/// A Fornjot model host
pub struct Host {
    evaluator_thread: Option<JoinHandle<()>>,
    _watcher: Watcher,
}

impl Host {
    /// Create a new instance of `Host`
    ///
    /// This is only useful, if you want to continuously watch the model for
    /// changes. If you don't, just keep using `Model`.
    pub fn new(
        model: Model,
        shape_processor: ShapeProcessor,
        event_loop_proxy: EventLoopProxy<ModelEvent>,
    ) -> Result<Self, Error> {
        let watch_path = model.watch_path();
        let (evaluator_thread, trigger_tx) =
            spawn_evaluator(model, shape_processor, event_loop_proxy);
        let watcher = Watcher::watch_model(watch_path, trigger_tx)?;

        Ok(Self {
            evaluator_thread: Some(evaluator_thread),
            _watcher: watcher,
        })
    }

    /// Check if the evaluator thread has exited with a panic.
    ///
    /// # Panics
    ///
    /// Panics if the evaluator thread has panicked.
    pub fn propagate_panic(&mut self) {
        if self.evaluator_thread.is_none() {
            unreachable!("Constructor requires host thread")
        }
        if let Some(evaluator_thread) = &self.evaluator_thread {
            // The host thread should not finish while this handle holds the
            // `command_tx` channel open, so an exit means the thread panicked.
            if evaluator_thread.is_finished() {
                let evaluator_thread = self.evaluator_thread.take().unwrap();
                match evaluator_thread.join() {
                    Ok(()) => {
                        unreachable!(
                            "Evaluator thread cannot exit until host handle disconnects"
                        )
                    }
                    // The error value has already been reported by the panic
                    // in the host thread, so just ignore it here.
                    Err(_) => {
                        panic!("Evaluator thread panicked")
                    }
                }
            }
        }
    }
}
