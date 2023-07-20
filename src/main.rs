mod render;

use render::Renderer;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

fn main() {
    let event_loop = EventLoop::new();
    let mut renderer = Renderer::new(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == renderer.window_id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(window_id) if window_id == renderer.window_id() => {
                renderer.clear();
                renderer.draw_triangle();
                renderer.present();
            }
            _ => (),
        }
    });
}
