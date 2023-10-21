use std::ffi::CString;

use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version},
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder, WindowId},
};

pub(crate) struct Renderer {
    window: Window,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
}

impl Renderer {
    pub(crate) fn new(event_loop: &EventLoop<()>) -> Self {
        let window_builder = WindowBuilder::new().with_title("iridium");

        let config_template = ConfigTemplateBuilder::default();
        let (window, config) = DisplayBuilder::new()
            .with_window_builder(Some(window_builder))
            .build(event_loop, config_template, |mut configs| {
                configs.next().unwrap()
            })
            .unwrap();
        let window = window.unwrap();
        let display = config.display();

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(window.raw_window_handle()));

        let surface_attributes =
            window.build_surface_attributes(SurfaceAttributesBuilder::default());
        let surface = unsafe {
            display
                .create_window_surface(&config, &surface_attributes)
                .unwrap()
        };

        let context = unsafe {
            display
                .create_context(&config, &context_attributes)
                .unwrap()
                .make_current(&surface)
                .unwrap()
        };

        gl::load_with(|s| display.get_proc_address(&CString::new(s).unwrap()));

        unsafe {
            gl::ClearColor(0.6, 0.4, 0.8, 1.0);
        }

        Self {
            window,
            surface,
            context,
        }
    }

    pub(crate) fn window_id(&self) -> WindowId {
        self.window.id()
    }

    pub(crate) fn clear(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub(crate) fn present(&mut self) {
        self.surface.swap_buffers(&self.context).unwrap();
    }
}
