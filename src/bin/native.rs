use std::error::Error;
use winit::event::{Event, WindowEvent};
use glaive::{Window, HasGLContext};
use glow::HasContext;

fn main() -> Result<(), Box<dyn Error>> {
    let Window {window, window_context: wnc, event_loop} = Window::new()?;
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                unsafe {
                    wnc.glc().clear_color(0.25, 0.0, 0.0, 1.0);
                    wnc.glc().clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                }
                if let Err(_e) = wnc.swap_buffers() {
                    // Log error
                }
            }
            _ => (),
        }
    });
}
