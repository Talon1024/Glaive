use wasm_bindgen::{prelude::*, JsCast};
use winit::{
    window::Window,
    event_loop::EventLoopWindowTarget,
};
use std::{panic, error::Error, sync::Arc};
use glow::Context as GLContext;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlRenderingContext};
use js_sys::Object;
use super::HasGLContext;

pub struct WindowContext {
    glc: Arc<GLContext>
}

impl WindowContext {
    pub fn swap_buffers(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl HasGLContext for WindowContext {
    fn glc(&self) -> &GLContext {
        &self.glc
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    super::start().unwrap();
}

fn insert_canvas(window: &Window) -> HtmlCanvasElement {
    use winit::platform::web::WindowExtWebSys;

    let canvas = window.canvas();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    body.append_child(&canvas).unwrap();
    canvas
}

pub(crate) fn make_gl_context<CE>(window: &Window, _el: &EventLoopWindowTarget<CE>) -> Result<WindowContext, Box<dyn Error>> {
    let canvas = insert_canvas(window);
    canvas.get_context("webgl2").map_err(Object::from).and_then(|ctx| {
        ctx.unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .map(GLContext::from_webgl2_context)
    }).or_else(|_| {
        canvas.get_context("webgl").map_err(Object::from).and_then(|ctx| {
            ctx.unwrap()
                .dyn_into::<WebGlRenderingContext>()
                .map(GLContext::from_webgl1_context)
        })
    }).map(|glc| {
        WindowContext { glc: Arc::new(glc) }
    }).map_err(|e| {
        String::from(e.to_string()).into()
    })
}
