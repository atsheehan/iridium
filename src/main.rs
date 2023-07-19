use std::ffi::CString;

use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, Version},
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContextSurfaceAccessor},
    surface::{GlSurface, SurfaceAttributesBuilder},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("iridium");

    let config_template = ConfigTemplateBuilder::default();
    let (window, config) = DisplayBuilder::new()
        .with_window_builder(Some(window_builder))
        .build(&event_loop, config_template, |mut configs| {
            configs.next().unwrap()
        })
        .unwrap();
    let window = window.unwrap();
    let display = config.display();

    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 0))))
        .build(Some(window.raw_window_handle()));

    let surface_attributes = window.build_surface_attributes(SurfaceAttributesBuilder::default());
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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                surface.swap_buffers(&context).unwrap();
            }
            _ => (),
        }
    });
}
