//! Model viewer initialization and event processing
//!
//! Provides the functionality to create a window and perform basic viewing
//! with programmed models.

use std::{
    error,
    fmt::{self, Write},
};

use fj_host::{Host, Model};
use fj_operations::shape_processor::ShapeProcessor;
use fj_viewer::{RendererInitError, StatusReport, Viewer};
use futures::executor::block_on;
use tracing::trace;
use winit::event_loop::EventLoop;

use crate::{
    event_loop_handler::{self, EventLoopHandler},
    window::{self, Window},
};

/// Initializes a model viewer for a given model and enters its process loop.
pub fn run(
    model: Option<Model>,
    shape_processor: ShapeProcessor,
    invert_zoom: bool,
) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;
    let viewer = block_on(Viewer::new(&window))?;

    let egui_winit_state = egui_winit::State::new(&event_loop);

    let host = model
        .map(|model| Host::new(model, shape_processor.clone()))
        .transpose()?;

    let mut handler = EventLoopHandler {
        invert_zoom,
        shape_processor,
        window,
        viewer,
        egui_winit_state,
        host,
        status: StatusReport::new(),
        held_mouse_button: None,
        new_size: None,
        stop_drawing: false,
    };

    event_loop.run(move |event, _, control_flow| {
        trace!("Handling event: {:?}", event);

        if let Err(err) = handler.handle_event(event, control_flow) {
            handle_error(err, &mut handler.status)
                .expect("Expected error handling not to fail");
        }
    });
}

fn handle_error(
    err: event_loop_handler::Error,
    status: &mut StatusReport,
) -> Result<(), fmt::Error> {
    // Can be cleaned up, once `Report` is stable:
    // https://doc.rust-lang.org/std/error/struct.Report.html

    let mut msg = String::new();

    writeln!(msg, "Shape processing error: {err}")?;

    let mut current_err = &err as &dyn error::Error;
    while let Some(err) = current_err.source() {
        writeln!(msg)?;
        writeln!(msg, "Caused by:")?;
        writeln!(msg, "    {err}")?;

        current_err = err;
    }

    status.update_status(&msg);

    Ok(())
}

/// Error in main loop
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error loading model
    #[error("Error loading model")]
    Model(#[from] fj_host::Error),

    /// Error initializing window
    #[error("Error initializing window")]
    WindowInit(#[from] window::Error),

    /// Error initializing graphics
    #[error("Error initializing graphics")]
    GraphicsInit(#[from] RendererInitError),
}
