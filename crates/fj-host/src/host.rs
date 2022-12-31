use fj_operations::shape_processor::ShapeProcessor;
use winit::event_loop::EventLoopProxy;

use crate::{Error, Evaluator, Model, ModelEvent, Watcher};

/// A Fornjot model host
pub struct Host {
    _evaluator: Evaluator,
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
        let evaluator =
            Evaluator::new(model, shape_processor, event_loop_proxy);
        let watcher = Watcher::watch_model(watch_path, &evaluator)?;

        Ok(Self {
            _evaluator: evaluator,
            _watcher: watcher,
        })
    }
}
