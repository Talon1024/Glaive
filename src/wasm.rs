use wasm_bindgen::prelude::*;
use winit::window::Window;
use std::panic;

pub struct WindowContext {}

impl WindowContext {
    pub fn swap_buffers(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    super::start();
}

pub fn insert_canvas(window: &Window) -> web_sys::HtmlCanvasElement {
    use winit::platform::web::WindowExtWebSys;

    let canvas = window.canvas();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    body.append_child(&canvas).unwrap();
    canvas
}
