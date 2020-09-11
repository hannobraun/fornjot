mod graphics;
mod input;
mod transform;
mod vertices;

use futures::executor::block_on;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use self::{
    graphics::{Geometry, Renderer},
    input::InputHandler,
    transform::Transform,
};

fn main() {
    let mut geometry = Geometry::new();
    geometry.vertices.extend_from_slice(vertices::VERTICES);
    geometry.indices.extend_from_slice(vertices::INDICES);

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Fornjot")
        .with_maximized(true)
        .with_decorations(true)
        .with_transparent(false)
        .build(&event_loop)
        .unwrap();

    let mut input_handler = InputHandler::new();
    let mut transform = Transform::new();
    let mut renderer = block_on(Renderer::new(&window)).unwrap();

    renderer.update_geometry(|g| {
        *g = geometry;
    });

    event_loop.run(move |event, _, control_flow| match event {
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
            input_handler.handle_keyboard_input(input, control_flow);
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
            input_handler.handle_mouse_input(state, button);
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
            renderer.draw(&transform).unwrap();
        }
        _ => {}
    })
}
