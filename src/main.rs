mod math;
mod render;
mod world;

use render::Renderer;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
};
use world::World;

fn main() {
    let options = get_options();

    let event_loop = EventLoop::new().unwrap();
    let mut renderer = Renderer::new(&event_loop, options.windowed);

    let mut world = World::new(20, 20);

    event_loop.set_control_flow(ControlFlow::Poll);
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
                match (state, physical_key) {
                    (ElementState::Pressed, PhysicalKey::Code(KeyCode::KeyW)) => {
                        world.start_moving_forward()
                    }
                    (ElementState::Pressed, PhysicalKey::Code(KeyCode::KeyS)) => {
                        world.start_moving_backward()
                    }
                    (ElementState::Pressed, PhysicalKey::Code(KeyCode::KeyA)) => {
                        world.start_moving_left()
                    }
                    (ElementState::Pressed, PhysicalKey::Code(KeyCode::KeyD)) => {
                        world.start_moving_right()
                    }
                    (ElementState::Pressed, PhysicalKey::Code(KeyCode::Space)) => {
                        world.start_moving_up()
                    }
                    (ElementState::Pressed, PhysicalKey::Code(KeyCode::ControlLeft)) => {
                        world.start_moving_down()
                    }
                    (ElementState::Released, PhysicalKey::Code(KeyCode::KeyW)) => {
                        world.stop_moving_forward()
                    }
                    (ElementState::Released, PhysicalKey::Code(KeyCode::KeyS)) => {
                        world.stop_moving_backward()
                    }
                    (ElementState::Released, PhysicalKey::Code(KeyCode::KeyA)) => {
                        world.stop_moving_left()
                    }
                    (ElementState::Released, PhysicalKey::Code(KeyCode::KeyD)) => {
                        world.stop_moving_right()
                    }
                    (ElementState::Released, PhysicalKey::Code(KeyCode::Space)) => {
                        world.stop_moving_up()
                    }
                    (ElementState::Released, PhysicalKey::Code(KeyCode::ControlLeft)) => {
                        world.stop_moving_down()
                    }
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
            }
            Event::AboutToWait => {
                world.update();

                renderer.set_camera(world.camera());
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
