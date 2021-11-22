mod args;
mod geometry;
mod graphics;
mod input;
mod math;
mod mesh;
mod model;

use std::{collections::HashMap, time::Instant};

use futures::executor::block_on;
use notify::Watcher as _;
use tracing::trace;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    args::Args,
    geometry::Shape as _,
    graphics::{DrawConfig, Renderer, Transform, FIELD_OF_VIEW},
    mesh::{HashVector, MeshMaker},
    model::Model,
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let model = Model::new(args.model);

    let mut arguments = HashMap::new();
    for argument in args.arguments {
        let mut argument = argument.splitn(2, "=");

        let key = argument
            .next()
            .expect("model argument: key not found")
            .to_owned();
        let value = argument
            .next()
            .expect("model argument: value not found")
            .to_owned();

        arguments.insert(key, value);
    }

    let shape = model.load(&arguments)?;

    let watch_path = model.src_path();
    let mut watcher = notify::recommended_watcher(
        move |event: notify::Result<notify::Event>| {
            // TASK: Figure out when this can happen, find a better way to
            //       handle it.
            let event = event.expect("Error handling watch event");

            let event = match event.kind {
                notify::EventKind::Access(
                    notify::event::AccessKind::Close(
                        notify::event::AccessMode::Write,
                    ),
                ) => event,
                _ => {
                    // irrelevant event
                    return;
                }
            };

            // TASK: Render the reloaded model.
            model.load(&arguments).expect("Error loading model");

            println!("{:?}", event);
        },
    )?;
    watcher.watch(&watch_path, notify::RecursiveMode::Recursive)?;

    let aabb = shape.bounding_volume();

    let tolerance = aabb.size().min() / 1000.;
    let triangles = shape.faces(tolerance);

    if let Some(path) = args.export {
        let mut mesh_maker = MeshMaker::new();

        for triangle in triangles.0 {
            for vertex in triangle.0 {
                mesh_maker.push(HashVector::from(vertex));
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

    let initial_distance = {
        // Let's make sure we choose a distance, so that the model fills most of
        // the screen.
        //
        // To do that, first compute the model's highest point, as well as the
        // furthers point from the origin, in x and y.
        let highest_point = aabb.max.z;
        let furthest_point =
            [aabb.min.x.abs(), aabb.max.x, aabb.min.y.abs(), aabb.max.y]
                .into_iter()
                .reduce(|a, b| f64::max(a, b))
                // `reduce` can only return `None`, if there are no items in the
                // iterator. And since we're creating an array full of items
                // above, we know this can't panic.
                .unwrap();

        // The actual furthest point is not far enough. We don't want the model
        // to fill the whole screen.
        let furthest_point = furthest_point * 2.;

        // Having computed those points, figuring out how far the camera needs
        // to be from the model is just a bit of trigonometry.
        let distance_from_model =
            furthest_point / (FIELD_OF_VIEW as f64 / 2.).atan();

        // An finally, the distance from the origin is trivial now.
        highest_point + distance_from_model
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Fornjot")
        .with_maximized(true)
        .with_decorations(true)
        .with_transparent(false)
        .build(&event_loop)
        .unwrap();

    let mut input_handler = input::Handler::new();
    let mut renderer = block_on(Renderer::new(&window, triangles.0.into()))?;

    let mut draw_config = DrawConfig::default();
    let mut transform = Transform::new(initial_distance as f32);

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
