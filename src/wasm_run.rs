use wasm_bindgen::prelude::*;
use crate::HasGLContext;
use winit::event::{Event, WindowEvent};
use glow::HasContext;
use std::panic;

#[wasm_bindgen(start)]
pub fn run() {
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let crate::Window {window, window_context: wnc, event_loop} = crate::Window::new().unwrap();
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
