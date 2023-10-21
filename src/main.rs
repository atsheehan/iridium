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

    let mut world = World::new(20, 20);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

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
                match (state, virtual_keycode) {
                    (ElementState::Pressed, Some(VirtualKeyCode::W)) => {
                        world.start_moving_forward()
                    }
                    (ElementState::Pressed, Some(VirtualKeyCode::S)) => {
                        world.start_moving_backward()
                    }
                    (ElementState::Pressed, Some(VirtualKeyCode::A)) => world.start_moving_left(),
                    (ElementState::Pressed, Some(VirtualKeyCode::D)) => world.start_moving_right(),
                    (ElementState::Pressed, Some(VirtualKeyCode::Space)) => world.start_moving_up(),
                    (ElementState::Pressed, Some(VirtualKeyCode::LControl)) => {
                        world.start_moving_down()
                    }
                    (ElementState::Released, Some(VirtualKeyCode::W)) => {
                        world.stop_moving_forward()
                    }
                    (ElementState::Released, Some(VirtualKeyCode::S)) => {
                        world.stop_moving_backward()
                    }
                    (ElementState::Released, Some(VirtualKeyCode::A)) => world.stop_moving_left(),
                    (ElementState::Released, Some(VirtualKeyCode::D)) => world.stop_moving_right(),
                    (ElementState::Released, Some(VirtualKeyCode::Space)) => world.stop_moving_up(),
                    (ElementState::Released, Some(VirtualKeyCode::LControl)) => {
                        world.stop_moving_down()
                    }
                    (ElementState::Pressed, Some(VirtualKeyCode::Escape)) => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                };
            }
            Event::RedrawRequested(window_id) if window_id == renderer.window_id() => {
                renderer.set_viewport();
            }
            Event::MainEventsCleared => {
                world.update();

                renderer.set_camera(world.camera());
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
