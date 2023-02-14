// Based on code from:
// https://github.com/rust-windowing/winit/blob/master/examples/web.rs
#![allow(clippy::single_match)]

use winit::{
    event_loop::{EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};
use std::{error::Error, sync::Arc};
use glow::Context as GLContext;

pub mod platform;

pub trait HasGLContext {
    fn glc(&self) -> &Arc<GLContext>;
}

pub struct Window<CE: 'static> {
    pub window: winit::window::Window,
    pub window_context: platform::WindowContext,
    gl_context: Arc<GLContext>,
    pub event_loop: EventLoop<CE>,
}

impl<CE> HasGLContext for Window<CE> {
    fn glc(&self) -> &Arc<GLContext> {
        &self.gl_context
    }
}

impl<CE> Window<CE> {
    pub fn with_user_event() -> Result<Self, Box<dyn Error>> {
        let event_loop = EventLoopBuilder::<CE>::with_user_event().build();

        let window = WindowBuilder::new()
            .with_title("Glaive")
            .build(&event_loop)
            .unwrap();

        let (wnc, glc) = platform::show_window(&window, &event_loop, platform::DrawingContextRequest::OpenGL);
        Ok(Window {
            window,
            window_context: wnc,
            gl_context: glc,
            event_loop
        })
    }
}

impl Window<()> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Window::<()>::with_user_event()
    }
}
