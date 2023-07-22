mod math;
mod render;
mod world;

use render::Renderer;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};
use world::World;

fn main() {
    let options = get_options();

    let event_loop = EventLoop::new().unwrap();
    let mut renderer = Renderer::new(&event_loop, options.windowed);

    let world = World::new(20, 20);

    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == renderer.window_id() => {
                window_target.exit();
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state,
                                physical_key,
                                ..
                            },
                        ..
                    },
                window_id,
            } if window_id == renderer.window_id() => {
                #[allow(clippy::single_match)]
                match (state, physical_key) {
                    (ElementState::Pressed, PhysicalKey::Code(KeyCode::Escape)) => {
                        window_target.exit();
                    }
                    _ => {}
                };
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                window_id,
            } if window_id == renderer.window_id() => {
                renderer.set_viewport();
                renderer.clear();

                for position in world.block_positions() {
                    renderer.draw_cube(&position);
                }

                renderer.present();
            }
            _ => (),
        })
        .unwrap();
}

struct GameOptions {
    windowed: bool,
}

fn get_options() -> GameOptions {
    let args: Vec<String> = std::env::args().collect();
    let windowed = args.iter().any(|arg| arg == "-w" || arg == "--windowed");
    GameOptions { windowed }
}
