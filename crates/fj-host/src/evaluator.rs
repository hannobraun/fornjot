use std::thread;

use crossbeam_channel::Sender;
use fj_interop::processed_shape::ProcessedShape;
use fj_operations::shape_processor::ShapeProcessor;
use winit::event_loop::{EventLoopClosed, EventLoopProxy};

use crate::{Error, Model};

/// Evaluates a model in a background thread
pub struct Evaluator {
    trigger_tx: Sender<TriggerEvaluation>,
}

impl Evaluator {
    /// Create an `Evaluator` from a model
    pub fn new(
        model: Model,
        shape_processor: ShapeProcessor,
        event_loop_proxy: EventLoopProxy<ModelEvent>,
    ) -> Self {
        let (trigger_tx, trigger_rx) = crossbeam_channel::bounded(0);

        thread::spawn(move || {
            if let Err(EventLoopClosed(..)) =
                event_loop_proxy.send_event(ModelEvent::StartWatching)
            {
                return;
            }
            evaluate_model(&model, &shape_processor, &event_loop_proxy);

            while matches!(trigger_rx.recv(), Ok(TriggerEvaluation)) {
                if let Err(EventLoopClosed(..)) =
                    event_loop_proxy.send_event(ModelEvent::ChangeDetected)
                {
                    return;
                }

                evaluate_model(&model, &shape_processor, &event_loop_proxy);
            }

            // The channel is disconnected, which means this instance of
            // `Evaluator`, as well as all `Sender`s created from it, have been
            // dropped. We're done.
        });

        Self {
            trigger_tx,
        }
    }

    /// Access a channel for triggering evaluations
    pub fn trigger(&self) -> Sender<TriggerEvaluation> {
        self.trigger_tx.clone()
    }
}

pub fn evaluate_model(
    model: &Model,
    shape_processor: &ShapeProcessor,
    event_loop_proxy: &EventLoopProxy<ModelEvent>,
) {
    let evaluation = match model.evaluate() {
        Ok(evaluation) => evaluation,

        Err(err) => {
            if let Err(EventLoopClosed(..)) =
                event_loop_proxy.send_event(ModelEvent::Error(err))
            {
                panic!();
            }
            return;
        }
    };

    event_loop_proxy.send_event(ModelEvent::Evaluated).unwrap();

    let shape = shape_processor.process(&evaluation.shape).unwrap();

    if let Err(EventLoopClosed(..)) =
        event_loop_proxy.send_event(ModelEvent::ProcessedShape(shape))
    {
        panic!();
    }
}

/// Command received by [`Evaluator`] through its channel
pub struct TriggerEvaluation;

/// An event emitted by [`Evaluator`]
#[derive(Debug)]
pub enum ModelEvent {
    /// A new model is being watched
    StartWatching,

    /// A change in the model has been detected
    ChangeDetected,

    /// The model has been evaluated
    Evaluated,

    /// The model has been processed into a `Shape`
    ProcessedShape(ProcessedShape),

    /// An error
    Error(Error),
}
