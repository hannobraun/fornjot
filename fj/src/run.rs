use std::{path::PathBuf, time::Instant};

use futures::executor::block_on;
use tracing::{info, trace};
use tracing_subscriber::EnvFilter;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    args::Args,
    draw_config::DrawConfig,
    geometry::isosurface::grid::Grid,
    graphics::{Renderer, Transform},
    input,
    model::IntoMesh,
    threemf, Mesh, Model,
};

pub fn run_model(model: impl Model) -> anyhow::Result<()> {
    let args = init();

    let params = serde_json::from_str(
        &args.model_params.unwrap_or_else(|| String::from("{}")),
    )?;
    let geometry = model.instantiate(params);

    info!("Converting geometry to triangle mesh...");

    let start_of_conversion = Instant::now();
    let (mesh, grid) = geometry.into_mesh();
    let conversion_duration = start_of_conversion.elapsed();

    info!(
        "Converted geometry in {}.{:03}s",
        conversion_duration.as_secs(),
        conversion_duration.subsec_millis()
    );

    run_inner(mesh, Some(grid), args.export)?;

    Ok(())
}

pub fn run_mesh(mesh: impl Into<Mesh>) -> anyhow::Result<()> {
    let args = init();
    run_inner(mesh.into(), None, args.export)?;
    Ok(())
}

fn init() -> Args {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("fj=info".parse().unwrap()),
        )
        .pretty()
        .init();

    Args::parse()
}

fn run_inner(
    mesh: Mesh,
    grid: Option<Grid>,
    export: Option<PathBuf>,
) -> anyhow::Result<()> {
    if let Some(path) = export {
        info!("Exporting to `{}`", path.display());
        threemf::export(&mesh, path)?;
        return Ok(());
    }

    trace!("Initializing event loop...");
    let event_loop = EventLoop::new();

    trace!("Initializing window...");
    let window = WindowBuilder::new()
        .with_title("Fornjot")
        .with_maximized(true)
        .with_decorations(true)
        .with_transparent(false)
        .build(&event_loop)
        .unwrap();

    trace!("Initializing event handler...");
    let mut input_handler = input::Handler::new();

    trace!("Initializing state...");
    let mut transform = Transform::new();
    let mut draw_config = DrawConfig::default();

    trace!("Initializing renderer...");
    let mut renderer = block_on(Renderer::new(
        &window,
        mesh.into(),
        grid.map(|grid| grid.into()),
    ))?;

    trace!("Finished initialization.");

    event_loop.run(move |event, _, control_flow| {
        trace!("Handling event: {:?}", event);

        let mut actions = input::Actions::new();

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
                input_handler.handle_cursor_moved(position, &mut transform);
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                input_handler.handle_mouse_input(button, state);
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                input_handler.handle_mouse_wheel(delta);
            }
            Event::MainEventsCleared => {
                // TASK: Create a proper main loop and call this at a fixed
                //       frequency instead of whenever this event pops up.
                input_handler.update(&mut transform);

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                match renderer.draw(&transform, &draw_config) {
                    Ok(()) => {}
                    Err(err) => {
                        panic!("Draw error: {}", err);
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
        if actions.toggle_grid {
            draw_config.draw_grid = !draw_config.draw_grid;
        }

        trace!("Event handled: {:?}", event);
    })
}
