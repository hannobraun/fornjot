use crossbeam_channel::Receiver;

use crate::{Error, Evaluator, Model, ModelEvent, Watcher};

/// A Fornjot model host
pub struct Host {
    evaluator: Evaluator,
    _watcher: Watcher,
}

impl Host {
    /// Create a new instance of `Host`
    ///
    /// This is only useful, if you want to continuously watch the model for
    /// changes. If you don't just keep using `Model`.
    pub fn from_model(model: Model) -> Result<Self, Error> {
        let watch_path = model.watch_path();
        let evaluator = Evaluator::from_model(model);
        let _watcher = match Watcher::watch_model(&watch_path, &evaluator) {
            Ok(_watcher) => _watcher,
            Err(e) => return Err(e),
        };

        return Ok(Self {
            evaluator,
            _watcher,
        });
    }

    /// Access a channel with evaluation events
    pub fn events(&self) -> Receiver<ModelEvent> {
        self.evaluator.events()
    }
}
