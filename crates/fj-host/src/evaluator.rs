use std::thread::{self, JoinHandle};

use crossbeam_channel::Sender;
use fj_interop::processed_shape::ProcessedShape;
use fj_operations::shape_processor::ShapeProcessor;
use winit::event_loop::{EventLoopClosed, EventLoopProxy};

use crate::{Error, Model};

/// Start a background thread for evaluating a model
pub fn spawn_evaluator(
    model: Model,
    shape_processor: ShapeProcessor,
    event_loop_proxy: EventLoopProxy<ModelEvent>,
) -> (JoinHandle<()>, Sender<TriggerEvaluation>) {
    let (trigger_tx, trigger_rx) = crossbeam_channel::bounded(0);

    let join_handle = thread::Builder::new()
        .name("evaluator".to_string())
        .spawn(move || {
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
        })
        .expect("Cannot create thread in evaluator");

    (join_handle, trigger_tx)
}

fn evaluate_model(
    model: &Model,
    shape_processor: &ShapeProcessor,
    event_loop_proxy: &EventLoopProxy<ModelEvent>,
) {
    let evaluation = match model.evaluate() {
        Ok(evaluation) => evaluation,
        Err(err) => {
            event_loop_proxy
                .send_event(ModelEvent::Error(err))
                .expect("Event loop proxy closed");
            return;
        }
    };

    event_loop_proxy
        .send_event(ModelEvent::Evaluated)
        .expect("Event loop proxy closed");

    let shape = shape_processor.process(&evaluation.shape).unwrap();

    event_loop_proxy
        .send_event(ModelEvent::ProcessedShape(shape))
        .expect("Event loop proxy closed");
}

/// Command received by an evaluator thread through its channel
pub struct TriggerEvaluation;

/// An event emitted by an evaluator thread
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
