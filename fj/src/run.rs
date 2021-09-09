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
    geometry::isosurface::grid,
    graphics::{DrawError, Renderer, Transform},
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

    let (mesh, grid) = geometry.into_mesh();
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
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    Args::parse()
}

fn run_inner(
    mesh: Mesh,
    grid: Option<grid::Descriptor>,
    export: Option<PathBuf>,
) -> anyhow::Result<()> {
    info!("Converting geometry to triangle mesh...");

    let start_of_conversion = Instant::now();
    let mesh = mesh.into();
    let conversion_duration = start_of_conversion.elapsed();

    info!(
        "Converted geometry in {}.{:03}s",
        conversion_duration.as_secs(),
        conversion_duration.subsec_millis()
    );

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

    trace!("Initializing transform...");
    let mut transform = Transform::new();

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
                input_handler.handle_mouse_wheel(delta, &mut transform);
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                match renderer.draw(&transform) {
                    Ok(()) => {}
                    err @ Err(DrawError(wgpu::SurfaceError::Outdated)) => {
                        // I'm getting this from time to time when resizing the
                        // window. It's not catastrophic.
                        println!("Draw error: {:?}", err);
                    }
                    Err(err) => {
                        panic!("Draw error: {:?}", err);
                    }
                }
            }
            _ => {}
        }

        if actions.exit {
            *control_flow = ControlFlow::Exit;
        }
        if actions.toggle_model {
            renderer.toggle_model();
        }
        if actions.toggle_mesh {
            renderer.toggle_mesh();
        }
        if actions.toggle_grid {
            renderer.toggle_grid();
        }

        trace!("Event handled: {:?}", event);
    })
}
