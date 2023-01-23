# Glaive

A small template project for [winit](https://github.com/rust-windowing/winit), which provides a setup compatible with native applications and WebAssembly.

To build your native app:

    cargo build

To build the WebAssembly version:

    wasm-pack build --target web

To test the WebAssembly version in a browser, run `simple-http-server` (get it [here][shs]) in the project root directory, and open `http://localhost:8000` in your browser.

[shs]: https://crates.io/crates/simple-http-server
