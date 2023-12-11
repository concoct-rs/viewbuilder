use crate::{rt::RuntimeGuard, Handle, Runtime};
use kurbo::Point;
use std::collections::HashMap;
use winit::event_loop::EventLoop;
use winit::window::WindowId;

mod window;
pub use window::Window;

pub struct UserInterface {
    event_loop: EventLoop<()>,
    rt: Runtime,
    windows: HashMap<WindowId, (winit::window::Window, Handle<Window>)>,
}

impl UserInterface {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            rt: Runtime::default(),
            windows: HashMap::new(),
        }
    }

    pub fn enter(&self) -> RuntimeGuard {
        self.rt.enter()
    }

    pub fn insert_window(&mut self, handle: Handle<Window>) {
        let window = winit::window::Window::new(&self.event_loop).unwrap();
        self.windows.insert(window.id(), (window, handle));
    }

    pub fn run(self) {
        self.event_loop.run(move |event, _, _| {
            self.rt.try_run();

            match event {
                winit::event::Event::WindowEvent { window_id, event } => match event {
                    winit::event::WindowEvent::CursorMoved { position, .. } => {
                        let handle = &self.windows[&window_id].1;
                        handle
                            .cursor_pos()
                            .emit((Point::new(position.x, position.y),));
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}
