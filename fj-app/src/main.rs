mod args;
mod camera;
mod config;
mod graphics;
mod input;
mod mesh;
mod model;
mod operations;
mod window;

use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{collections::HashMap, sync::mpsc, time::Instant};

use fj_debug::DebugInfo;
use fj_math::Scalar;
use futures::executor::block_on;
use notify::Watcher as _;
use tracing::trace;
use tracing_subscriber::fmt::format;
use tracing_subscriber::EnvFilter;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    args::Args,
    camera::Camera,
    config::Config,
    graphics::{DrawConfig, Renderer},
    mesh::MeshMaker,
    model::Model,
    operations::ToShape as _,
    window::Window,
};

fn main() -> anyhow::Result<()> {
    // Respect `RUST_LOG`. If that's not defined or erroneous, log warnings and
    // above.
    //
    // It would be better to fail, if `RUST_LOG` is erroneous, but I don't know
    // how to distinguish between that and the "not defined" case.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("WARN")),
        )
        .event_format(format().pretty())
        .init();

    let args = Args::parse();
    let config = Config::load()?;

    let mut path = config.default_path.unwrap_or_else(|| PathBuf::from(""));
    match args.model.or(config.default_model) {
        Some(model) => {
            path.push(model);
        }
        None => {
            anyhow::bail!(
                "No model specified, and no default model configured.\n\
                Specify a model by passing `--model path/to/model`."
            );
        }
    }

    let model = Model::from_path(path, config.target_dir)?;

    let mut parameters = HashMap::new();
    for parameter in args.parameters {
        let mut parameter = parameter.splitn(2, '=');

        let key = parameter
            .next()
            .expect("model parameter: key not found")
            .to_owned();
        let value = parameter
            .next()
            .expect("model parameter: value not found")
            .to_owned();

        parameters.insert(key, value);
    }

    // Since we're loading the model before setting up the watcher below,
    // there's a race condition, and a modification could be missed between
    // those two events.
    //
    // This can't be addressed with the current structure, since the watcher
    // closure takes ownership of the model.
    //
    // This is being tracked in the following issue:
    // https://github.com/hannobraun/fornjot/issues/32
    let shape = model.load(&parameters)?;

    let mut aabb = shape.bounding_volume();

    let tolerance = match args.tolerance {
        None => {
            // Compute a reasonable default for the tolerance value. To do this, we just
            // look at the smallest non-zero extent of the bounding box and divide that
            // by some value.
            let mut min_extent = Scalar::MAX;
            for extent in aabb.size().components {
                if extent > Scalar::ZERO && extent < min_extent {
                    min_extent = extent;
                }
            }

            // `tolerance` must not be zero, or we'll run into trouble.
            let tolerance = min_extent / Scalar::from_f64(1000.);
            assert!(tolerance > Scalar::ZERO);

            tolerance
        }
        Some(user_defined_tolerance) => {
            if user_defined_tolerance > 0.0 {
                Scalar::from_f64(user_defined_tolerance)
            } else {
                anyhow::bail!("Invalid user defined model deviation tolerance: {}. Tolerance must be larger than zero", 
                user_defined_tolerance)
            }
        }
    };

    let mut debug_info = DebugInfo::new();
    let mut triangles = Vec::new();
    shape
        .to_shape(tolerance, &mut debug_info)
        .topology()
        .triangles(tolerance, &mut triangles, &mut debug_info);

    if let Some(path) = args.export {
        let mut mesh_maker = MeshMaker::new();

        for triangle in triangles {
            for vertex in triangle.points() {
                mesh_maker.push(vertex);
            }
        }

        let vertices =
            mesh_maker.vertices().map(|vertex| vertex.into()).collect();

        let indices: Vec<_> = mesh_maker.indices().collect();
        let triangles = indices
            .chunks(3)
            .map(|triangle| {
                [
                    triangle[0] as usize,
                    triangle[1] as usize,
                    triangle[2] as usize,
                ]
            })
            .collect();

        let mesh = threemf::TriangleMesh {
            vertices,
            triangles,
        };

        threemf::write(path, &mesh)?;

        return Ok(());
    }

    let (watcher_tx, watcher_rx) = mpsc::sync_channel(0);

    let watch_path = model.src_path();
    let mut watcher = notify::recommended_watcher(
        move |event: notify::Result<notify::Event>| {
            // Unfortunately the `notify` documentation doesn't say when this
            // might happen, so no idea if it needs to be handled.
            let event = event.expect("Error handling watch event");

            //Various acceptable ModifyKind kinds. Varies across platforms (e.g. MacOs vs. Windows10)
            if let notify::EventKind::Modify(notify::event::ModifyKind::Any)
            | notify::EventKind::Modify(notify::event::ModifyKind::Data(
                notify::event::DataChange::Any,
            ))
            | notify::EventKind::Modify(notify::event::ModifyKind::Data(
                notify::event::DataChange::Content,
            )) = event.kind
            {
                let file_ext = event
                    .paths
                    .get(0)
                    .expect("File path missing in watch event")
                    .extension();

                let black_list = HashSet::from([
                    OsStr::new("swp"),
                    OsStr::new("tmp"),
                    OsStr::new("swx"),
                ]);

                if let Some(ext) = file_ext {
                    if black_list.contains(ext) {
                        return;
                    }
                }

                let shape = match model.load(&parameters) {
                    Ok(shape) => shape,
                    Err(model::Error::Compile) => {
                        // It would be better to display an error in the UI,
                        // where the user can actually see it. Issue:
                        // https://github.com/hannobraun/fornjot/issues/30
                        println!("Error compiling model");
                        return;
                    }
                    Err(err) => {
                        panic!("Error reloading model: {:?}", err);
                    }
                };

                // This will panic, if the other end is disconnected, which is
                // probably the result of a panic on that thread, or the
                // application is being shut down.
                //
                // Either way, not much we can do about it here, except maybe to
                // provide a better error message in the future.
                watcher_tx.send(shape).unwrap();
            }
        },
    )?;
    watcher.watch(&watch_path, notify::RecursiveMode::Recursive)?;

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop);

    let mut previous_time = Instant::now();

    let mut input_handler = input::Handler::new(previous_time);
    let mut renderer = block_on(Renderer::new(&window))?;

    renderer.update_geometry((&triangles).into(), (&debug_info).into(), aabb);

    let mut draw_config = DrawConfig::default();
    let mut camera = Camera::new(&aabb);

    event_loop.run(move |event, _, control_flow| {
        trace!("Handling event: {:?}", event);

        let mut actions = input::Actions::new();

        let now = Instant::now();

        match watcher_rx.try_recv() {
            Ok(shape) => {
                debug_info.clear();
                triangles.clear();

                aabb = shape.bounding_volume();
                shape
                    .to_shape(tolerance, &mut debug_info)
                    .topology()
                    .triangles(tolerance, &mut triangles, &mut debug_info);

                renderer.update_geometry(
                    (&triangles).into(),
                    (&debug_info).into(),
                    aabb,
                );
            }
            Err(mpsc::TryRecvError::Empty) => {
                // Nothing to receive from the channel. We don't care.
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                // The other end has disconnected. This is probably the result
                // of a panic on the other thread, or a program shutdown in
                // progress. In any case, not much we can do here.
                panic!();
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
                input_handler.handle_cursor_moved(
                    position,
                    &mut camera,
                    &window,
                );
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                let focus_point = camera.focus_point(
                    &window,
                    input_handler.cursor(),
                    &triangles,
                );

                input_handler.handle_mouse_input(button, state, focus_point);
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

                input_handler.update(
                    delta_t.as_secs_f64(),
                    now,
                    &mut camera,
                    &window,
                    &triangles,
                );

                window.inner().request_redraw();
            }
            Event::RedrawRequested(_) => {
                camera.update_planes(&aabb);

                match renderer.draw(&camera, &draw_config) {
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
        if actions.toggle_debug {
            draw_config.draw_debug = !draw_config.draw_debug;
        }
    });
}
