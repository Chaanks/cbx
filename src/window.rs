use winit::{EventsLoop, WindowBuilder, Event, WindowEvent, Window, LogicalSize, CreationError};

#[Derive(Debug)]
pub struct WindowState {
    pub event_loop: EventsLoop,
    pub window: Window,
}

impl WindowState {
    pub fn new<T: into<String>>(title: T, size: LogicalSize) -> Result<Self, CreationError>{
        let mut event_loop = EventsLoop::new();
        let windows = WindowBuilder::new()
            .with_title("Triangle")
            .with_dimensions(size)
            .build(&event_loop)?;

        Ok(Self {
            event_loop,
            window,
        })
    }
}