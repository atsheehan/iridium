mod math;
mod render;

use math::Vec3;
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
                renderer.set_viewport();
                renderer.clear();
                renderer.draw_cube(&Vec3(2.0, 3.0, 6.0));
                renderer.draw_cube(&Vec3(-4.0, 0.0, 6.0));
                renderer.draw_cube(&Vec3(5.0, 10.0, 30.0));
                renderer.draw_cube(&Vec3(20.0, -23.0, 30.0));
                renderer.present();
            }
            _ => (),
        })
        .unwrap();
}
