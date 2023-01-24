use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{WindowEvent, Event}};

/// Stores a Window and the Event Loop.
pub struct Window {
    pub(crate) event_loop: EventLoop<()>,
    pub(crate) window: winit::window::Window,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        Window {
            event_loop,
            window,
        }
    }

    pub fn block<F>(self, mut handler: F) -> ! where F: 'static + FnMut(crate::engine::Event) {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            handler(crate::engine::Event::Resize(*physical_size))
                        },
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => (),
                        _ => {}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    handler(crate::engine::Event::Draw)
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                },
                _ => ()
            }
        })
    }
}