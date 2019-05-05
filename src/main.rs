mod window;

use winit::{EventsLoop, WindowBuilder, Event, WindowEvent};

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .filter_module("triangle", log::LevelFilter::Trace)
        .init();

    let mut event_loop = EventsLoop::new();
    let windows = WindowBuilder::new()
        .with_title("Triangle")
        .build(&event_loop)
        .expect("Failed to create window");

    run(&mut event_loop);



        
}

fn run(event_loop: &mut EventsLoop) {
    log::info!("Application running");
    let mut close  = false;
    loop {
        event_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => close = true,
            _ => (),
        });

        if close {
            log::info!("Close requested");
            break;
        }
    }
}
