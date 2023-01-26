// Based on code from:
// https://github.com/rust-windowing/winit/blob/master/examples/web.rs
#![allow(clippy::single_match)]

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use glow::Context as GLContext;

cfg_if::cfg_if! {
    if #[cfg(target_family = "wasm")] {
        mod wasm;
        pub use wasm::WindowContext;
        use wasm_bindgen::JsCast;
        use web_sys::{WebGlRenderingContext, WebGl2RenderingContext};
    } else {
        mod native;
        pub use native::WindowContext;
    }
}

pub fn start() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Glaive")
        .build(&event_loop)
        .unwrap();

    #[cfg(target_family = "wasm")]
    {
        let canvas = wasm::insert_canvas(&window);
        let _glc = canvas.get_context("webgl2").and_then(|ctx| {
            ctx.unwrap()
                .dyn_into::<WebGl2RenderingContext>()
                .map(GLContext::from_webgl2_context)
                .map_err(wasm_bindgen::JsValue::from)
        }).or_else(|_| {
            canvas.get_context("webgl").and_then(|ctx| {
                ctx.unwrap()
                    .dyn_into::<WebGlRenderingContext>()
                    .map(GLContext::from_webgl1_context)
                    .map_err(wasm_bindgen::JsValue::from)
            })
        });
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
