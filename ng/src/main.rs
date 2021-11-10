mod graphics;
mod input;
mod mesh;

use std::{process::Command, time::Instant};

use futures::executor::block_on;
use tracing::trace;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use self::{
    graphics::{DrawConfig, Renderer, Transform},
    mesh::MeshMaker,
};

fn main() -> anyhow::Result<()> {
    // This can be made a bit more contact using `ExitStatus::exit_ok`, once
    // that is stable.
    let status = Command::new("cargo")
        .arg("build")
        .args(["--manifest-path", "model/Cargo.toml"])
        .status()?;
    assert!(status.success());

    // TASK: Read up why those calls are unsafe. Make sure calling them is
    //       sound, and document why that is.
    let model = unsafe {
        let lib = libloading::Library::new("model/target/debug/libmodel.so")?;
        let func: libloading::Symbol<ModelFn> = lib.get(b"model")?;
        func()
    };

    let mut mesh = MeshMaker::new();
    let s = model.cube_size / 2.;

    // Define a cube
    let v0 = [-s, -s, -s];
    let v1 = [-s, -s, s];
    let v2 = [-s, s, -s];
    let v3 = [-s, s, s];
    let v4 = [s, -s, -s];
    let v5 = [s, -s, s];
    let v6 = [s, s, -s];
    let v7 = [s, s, s];

    // left
    mesh.triangle([v0, v1, v2]);
    mesh.triangle([v2, v1, v3]);

    // right
    mesh.triangle([v4, v6, v5]);
    mesh.triangle([v6, v7, v5]);

    // front
    mesh.triangle([v0, v4, v1]);
    mesh.triangle([v4, v5, v1]);

    // back
    mesh.triangle([v2, v3, v6]);
    mesh.triangle([v6, v3, v7]);

    // bottom
    mesh.triangle([v0, v2, v6]);
    mesh.triangle([v0, v6, v4]);

    // top
    mesh.triangle([v1, v5, v7]);
    mesh.triangle([v1, v7, v3]);

    let mesh = mesh.make();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Fornjot")
        .with_maximized(true)
        .with_decorations(true)
        .with_transparent(false)
        .build(&event_loop)
        .unwrap();

    let mut input_handler = input::Handler::new();
    let mut renderer = block_on(Renderer::new(&window, mesh.into()))?;

    let mut draw_config = DrawConfig::default();
    let mut transform = Transform::new();

    let mut previous_time = Instant::now();

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
                let now = Instant::now();
                let delta_t = now.duration_since(previous_time);
                previous_time = now;

                input_handler.update(delta_t.as_secs_f32(), &mut transform);

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
    });
}

type ModelFn = unsafe extern "C" fn() -> fj::Shape;
