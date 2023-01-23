// Based on code from:
// https://github.com/rust-windowing/winit/blob/master/examples/web.rs
#![allow(clippy::single_match)]

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Glaive")
        .build(&event_loop)
        .unwrap();

    #[cfg(target_family = "wasm")]
    {
        let _canvas = wasm::insert_canvas(&window);
    }

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}

#[cfg(target_family = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::*;
    use winit::window::Window;

    #[wasm_bindgen(start)]
    pub fn run() {
        console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

        #[allow(clippy::main_recursion)]
        super::main();
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
}
