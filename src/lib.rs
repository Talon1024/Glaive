// Based on code from:
// https://github.com/rust-windowing/winit/blob/master/examples/web.rs
#![allow(clippy::single_match)]

use winit::{
    event_loop::{EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};
use std::error::Error;
use glow::Context as GLContext;
cfg_if::cfg_if! {
    if #[cfg(target_family = "wasm")] {
        mod wasm;
        mod wasm_run;
        use wasm::{make_gl_context, WindowContext};
    } else {
        mod native;
        use native::{make_gl_context, WindowContext};
    }
}

pub trait HasGLContext {
    fn glc(&self) -> &GLContext;
}

pub struct Window<CE: 'static> {
    pub window: winit::window::Window,
    pub window_context: WindowContext,
    pub event_loop: EventLoop<CE>,
}

impl<CE> Window<CE> {
    pub fn with_user_event() -> Result<Self, Box<dyn Error>> {
        let event_loop = EventLoopBuilder::<CE>::with_user_event().build();

        let window = WindowBuilder::new()
            .with_title("Glaive")
            .build(&event_loop)
            .unwrap();

        let wnc = make_gl_context(&window, &event_loop)?;
        Ok(Window {
            window,
            window_context: wnc,
            event_loop
        })
    }
}

impl Window<()> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Window::<()>::with_user_event()
    }
}
