use glutin::{
    config::{ConfigTemplateBuilder, Api},
    surface::{Surface, GlSurface, WindowSurface, SurfaceAttributesBuilder},
    context::{ContextAttributesBuilder, ContextApi, GlProfile, Robustness, Version, NotCurrentGlContextSurfaceAccessor},
    display::{GlDisplay, GetGlDisplay}
};
use raw_window_handle::HasRawWindowHandle;
use glutin_winit::{DisplayBuilder, ApiPrefence};
use glow::Context as GLContext;
use winit::{
    window::Window,
    event_loop::EventLoopWindowTarget, dpi::PhysicalSize,
};
use std::{error::Error, sync::Arc, num::NonZeroU32, ffi::CStr};
use super::HasGLContext;

pub struct WindowContext {
    glc: Arc<GLContext>,
    wc: <Surface<WindowSurface> as GlSurface<WindowSurface>>::Context,
    surf: Surface<WindowSurface>,
}

impl WindowContext {
    pub fn swap_buffers(&self) -> Result<(), glutin::error::Error> {
        self.surf.swap_buffers(&self.wc)
    }
}

impl HasGLContext for WindowContext {
    fn glc(&self) -> &GLContext {
        &self.glc
    }
}

pub fn make_gl_context<CE>(window: &Window, el: &EventLoopWindowTarget<CE>) -> Result<WindowContext, Box<dyn Error>> {
    let ctb = ConfigTemplateBuilder::new()
        .with_api(Api::all())
        .prefer_hardware_accelerated(Some(true));
    let (_win, cfg) = DisplayBuilder::new()
        .with_preference(ApiPrefence::PreferEgl)
        .build(el, ctb, |mut c| {
            /* #[cfg(debug_assertions)]
            {
                let first = c.next().expect("Could not find an appropriate configuration");
                dbg!(&first);
                c.for_each(|conf| {dbg!(&conf);});
                first
            }
            #[cfg(not(debug_assertions))]
            {
                c.next().expect("Could not find an appropriate configuration")
            } */
            c.next().expect("Could not find an appropriate configuration")
        })
        .expect("Could not build the display");

    let ca = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version {
            major: 3,
            minor: 3,
        })))
        .with_profile(GlProfile::Core)
        .with_robustness(if cfg!(debug_assertions) {
            Robustness::RobustNoResetNotification
        } else {
            Robustness::NoError
        })
        .build(None);

    let sa = {
            let PhysicalSize { width, height } = window.inner_size();
            SurfaceAttributesBuilder::<WindowSurface>::new().with_srgb(None).build(
            window.raw_window_handle(),
            unsafe { NonZeroU32::new_unchecked(width) },
            unsafe { NonZeroU32::new_unchecked(height) },
        )
    };

    let dsp = cfg.display();
    let wc = unsafe { dsp.create_context(&cfg, &ca) }
        .expect("Could not create context");
    let surf = unsafe { dsp.create_window_surface(&cfg, &sa) }
        .expect("Could not create surface on window");
    let wc = wc.make_current(&surf).expect("Could not make context current");
    let glc = Arc::new(unsafe {
        GLContext::from_loader_function(|name| {
            let name = CStr::from_ptr(name.as_ptr() as *const i8);
            dsp.get_proc_address(name)
        })
    });
    Ok(WindowContext { glc, wc, surf })
}
