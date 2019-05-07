use winit::{EventsLoop, WindowBuilder, Event, WindowEvent, Window, CreationError};


pub const WINDOW_NAME: &str = "Hello Window";

#[derive(Debug)]
pub struct WindowState {
    pub event_loop: EventsLoop,
    pub window: Window,
}

impl WindowState {
    pub fn new<T: Into<String>>(title: T, size: (u32, u32)) -> Result<Self, CreationError>{
        let mut event_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title("Triangle")
            .with_dimensions(size.into())
            .build(&event_loop)?;

        Ok(Self {
            event_loop,
            window,
        })
    }

    pub fn run(&mut self) {
        log::info!("Application running");
        let mut close  = false;
        loop {
            self.event_loop.poll_events(|event| match event {
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
}

impl Default for WindowState {
    fn default() -> Self {
        Self::new(
            WINDOW_NAME,
            (800, 600),
        )
        .expect("Failed to create window")
    }
}