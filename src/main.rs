use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();

    event_loop.run(move |event, _, _| {
        println!("{:?}", event);
    })
}
