mod args;
mod geometry;
mod graphics;
mod input;
mod math;
mod mesh;
mod model;

use std::{collections::HashMap, sync::mpsc, time::Instant};

use futures::executor::block_on;
use notify::Watcher as _;
use parry3d_f64::query::{Ray, RayCast as _};
use tracing::trace;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    args::Args,
    geometry::Shape as _,
    graphics::{DrawConfig, Renderer, Transform, FIELD_OF_VIEW_IN_X, NEAR_PLANE},
    math::{Point, Vector},
    mesh::{HashVector, MeshMaker},
    model::Model,
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let model = Model::new(args.model);

    let mut parameters = HashMap::new();
    for parameter in args.parameters {
        let mut parameter = parameter.splitn(2, "=");

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

    // TASK: Since we're loading the model before setting up the watcher below,
    //       there's a race condition, and a modification could be missed
    //       between those two events.
    //
    //       This can't be addressed with the current structure, since the
    //       watcher closure takes ownership of the model.
    let shape = model.load(&parameters)?;

    let (watcher_tx, watcher_rx) = mpsc::sync_channel(0);

    let watch_path = model.src_path();
    let mut watcher = notify::recommended_watcher(
        move |event: notify::Result<notify::Event>| {
            // TASK: Figure out when this error can happen, find a better way to
            //       handle it.
            let event = event.expect("Error handling watch event");

            if let notify::EventKind::Access(
                notify::event::AccessKind::Close(
                    notify::event::AccessMode::Write,
                ),
            ) = event.kind
            {
                let shape = match model.load(&parameters) {
                    Ok(shape) => shape,
                    Err(model::Error::Compile) => {
                        // TASK: Display error message in graphics window.
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

    let aabb = shape.bounding_volume();

    let tolerance = aabb.size().min() / 1000.;
    let mut triangles = shape.faces(tolerance);

    if let Some(path) = args.export {
        let mut mesh_maker = MeshMaker::new();

        for triangle in triangles.0 {
            for vertex in triangle.vertices() {
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
            furthest_point / (FIELD_OF_VIEW_IN_X as f64 / 2.).atan();

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

    let mut previous_time = Instant::now();

    let mut input_handler = input::Handler::new(previous_time);
    let mut renderer = block_on(Renderer::new(&window))?;
    renderer.update_geometry((&triangles.0).into());

    let mut draw_config = DrawConfig::default();
    let mut transform = Transform::new(initial_distance);

    event_loop.run(move |event, _, control_flow| {
        trace!("Handling event: {:?}", event);

        let mut actions = input::Actions::new();

        let now = Instant::now();

        match watcher_rx.try_recv() {
            Ok(shape) => {
                triangles = shape.faces(tolerance);
                renderer.update_geometry((&triangles.0).into());
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
                input_handler.handle_keyboard_input(
                    input,
                    &mut actions,
                    &mut transform,
                );
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                input_handler.handle_cursor_moved(position, &mut transform);

                if let Some(cursor) = input_handler.cursor() {
                    let [width, height] = renderer.surface_size();
                    let aspect_ratio = renderer.aspect_ratio();

                    // Cursor position in normalized coordinates (-1 to +1) with
                    // aspect ratio taken into account.
                    let x = cursor.x / width as f64 * 2. - 1.;
                    let y =
                        -(cursor.y / height as f64 * 2. - 1.) / aspect_ratio;

                    // Cursor position in camera space.
                    let fov = FIELD_OF_VIEW_IN_X.tan();
                    let cursor = Point::origin()
                        + Vector::new(x * fov, y * fov, -NEAR_PLANE);

                    // Transform camera and cursor positions to model space.
                    let camera_to_model = transform.view_transform().inverse();
                    let origin =
                        camera_to_model.transform_point(&Point::origin());
                    let cursor = camera_to_model.transform_point(&cursor);
                    let dir = (cursor - origin).normalize();

                    // TASK: Ray direction behaves weirdly, when looking at it
                    //       more closely. It changes a lot when moving the
                    //       cursor near the center of the screen, but barely at
                    //       the edges.
                    let ray = Ray { origin, dir };

                    let mut min_t = None;

                    for triangle in &triangles.0 {
                        let t = triangle.cast_local_ray(
                            &ray,
                            // TASK: This is the maximum time of impact. Come up
                            //       with a better value.
                            f64::INFINITY,
                            true,
                        );

                        // TASK: Intersections are only detected at the very
                        //       center of the screen, even though the triangles
                        //       cover a much larger area on screen.
                        if let Some(t) = t {
                            if t <= min_t.unwrap_or(t) {
                                min_t = Some(t);
                            }
                        }
                    }

                    if let Some(t) = min_t {
                        // TASK: The detected point is implausible. Even though
                        //       intersections are only detected near the center
                        //       of the screen, x and y coordinates of the
                        //       intersection point are too large.
                        //
                        //       There must be a problem with the perspective.
                        let point = ray.point_at(t);
                        dbg!(point);
                    }
                }
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
                input_handler.handle_mouse_wheel(delta, now);
            }
            Event::MainEventsCleared => {
                let delta_t = now.duration_since(previous_time);
                previous_time = now;

                input_handler.update(
                    delta_t.as_secs_f64(),
                    now,
                    &mut transform,
                );

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
