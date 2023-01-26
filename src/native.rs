use glutin::{
	surface::{Surface, GlSurface, WindowSurface},
};

pub struct WindowContext {
    wc: <Surface<WindowSurface> as GlSurface<WindowSurface>>::Context,
    surf: Surface<WindowSurface>,
}

impl WindowContext {
    pub fn swap_buffers(&self) -> Result<(), glutin::error::Error> {
        self.surf.swap_buffers(&self.wc)
    }
}
