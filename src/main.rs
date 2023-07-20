mod render;

use render::Renderer;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut renderer = Renderer::new(&event_loop);

    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == renderer.window_id() => {
                window_target.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                window_id,
            } if window_id == renderer.window_id() => {
                renderer.clear();
                renderer.draw_cube();
                renderer.present();
            }
            _ => (),
        })
        .unwrap();
}
