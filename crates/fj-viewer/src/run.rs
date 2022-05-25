//! Model viewer initialization and event processing
//!
//! Provides the functionality to create a window and perform basic viewing
//! with programmed models.

use std::{error, time::Instant};

use fj_host::Watcher;
use fj_operations::shape_processor::ShapeProcessor;
use futures::executor::block_on;
use tracing::{trace, warn};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    camera::Camera,
    graphics::{self, DrawConfig, Renderer},
    input,
    window::{self, Window},
};

/// Initializes a model viewer for a given model and enters its process loop.
pub fn run(
    watcher: Watcher,
    shape_processor: ShapeProcessor,
) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;

    let mut previous_time = Instant::now();

    let mut input_handler = input::Handler::new(previous_time);
    let mut renderer = block_on(Renderer::new(&window))?;

    let mut draw_config = DrawConfig::default();

    let mut shape = None;
    let mut camera = None;

    event_loop.run(move |event, _, control_flow| {
        trace!("Handling event: {:?}", event);

        let mut actions = input::Actions::new();

        let now = Instant::now();

        if let Some(new_shape) = watcher.receive() {
            match shape_processor.process(&new_shape) {
                Ok(new_shape) => {
                    renderer.update_geometry(
                        (&new_shape.mesh).into(),
                        (&new_shape.debug_info).into(),
                        new_shape.aabb,
                    );

                    if camera.is_none() {
                        camera = Some(Camera::new(&new_shape.aabb));
                    }

                    shape = Some(new_shape);
                }
                Err(err) => {
                    // Can be cleaned up, once `Report` is stable:
                    // https://doc.rust-lang.org/std/error/struct.Report.html

                    println!("Shape processing error: {}", err);

                    let mut current_err = &err as &dyn error::Error;
                    while let Some(err) = current_err.source() {
                        println!();
                        println!("Caused by:");
                        println!("    {}", err);

                        current_err = err;
                    }
                }
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                renderer.handle_resize(size);
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                input_handler.handle_keyboard_input(input, &mut actions);
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                if let Some(camera) = &mut camera {
                    input_handler
                        .handle_cursor_moved(position, camera, &window);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if let (Some(shape), Some(camera)) = (&shape, &camera) {
                    let focus_point = camera.focus_point(
                        &window,
                        input_handler.cursor(),
                        &shape.mesh,
                    );

                    input_handler.handle_mouse_input(
                        button,
                        state,
                        focus_point,
                    );
                }
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                input_handler.handle_mouse_wheel(delta, now);
            }
            Event::MainEventsCleared => {
                let delta_t = now.duration_since(previous_time);
                previous_time = now;

                if let (Some(shape), Some(camera)) = (&shape, &mut camera) {
                    input_handler.update(
                        delta_t.as_secs_f64(),
                        now,
                        camera,
                        &window,
                        &shape.mesh,
                    );
                }

                window.inner().request_redraw();
            }
            Event::RedrawRequested(_) => {
                if let (Some(shape), Some(camera)) = (&shape, &mut camera) {
                    camera.update_planes(&shape.aabb);

                    if let Err(err) = renderer.draw(camera, &draw_config) {
                        warn!("Draw error: {}", err);
                    }
                }
            }
            _ => {}
        }

        if actions.exit {
            *control_flow = ControlFlow::Exit;
        }
        if actions.toggle_model {
            draw_config.draw_model = !draw_config.draw_model;
        }
        if actions.toggle_mesh {
            draw_config.draw_mesh = !draw_config.draw_mesh;
        }
        if actions.toggle_debug {
            draw_config.draw_debug = !draw_config.draw_debug;
        }
    });
}

/// Error in main loop
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error initializing window
    #[error("Error initializing window")]
    WindowInit(#[from] window::Error),

    /// Error initializing graphics
    #[error("Error initializing graphics")]
    GraphicsInit(#[from] graphics::InitError),
}
