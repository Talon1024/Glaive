// Based on code from:
// https://github.com/rust-windowing/winit/blob/master/examples/web.rs
#![allow(clippy::single_match)]

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::error::Error;
use glow::{Context as GLContext, HasContext};
cfg_if::cfg_if! {
    if #[cfg(target_family = "wasm")] {
        mod wasm;
        use wasm::make_gl_context;
    } else {
        mod native;
        use native::make_gl_context;
    }
}

pub trait HasGLContext {
    fn glc(&self) -> &GLContext;
}

pub fn start() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Glaive")
        .build(&event_loop)
        .unwrap();

    let wnc = make_gl_context(&window, &event_loop)?;

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
