mod math;
mod render;
mod world;

use render::Renderer;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use world::World;

fn main() {
    let options = get_options();

    let event_loop = EventLoop::new();
    let mut renderer = Renderer::new(&event_loop, options.windowed);

    let world = World::new(20, 20);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == renderer.window_id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state,
                                virtual_keycode,
                                ..
                            },
                        ..
                    },
                window_id,
            } if window_id == renderer.window_id() => {
                #[allow(clippy::single_match)]
                match (state, virtual_keycode) {
                    (ElementState::Pressed, Some(VirtualKeyCode::Escape)) => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                };
            }
            Event::RedrawRequested(window_id) if window_id == renderer.window_id() => {
                renderer.set_viewport();
                renderer.clear();

                for position in world.block_positions() {
                    renderer.draw_cube(&position);
                }

                renderer.present();
            }
            _ => (),
        }
    });
}

struct GameOptions {
    windowed: bool,
}

fn get_options() -> GameOptions {
    let args: Vec<String> = std::env::args().collect();
    let windowed = args.iter().any(|arg| arg == "-w" || arg == "--windowed");
    GameOptions { windowed }
}
