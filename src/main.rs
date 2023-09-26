mod math;
mod render;
mod time;
mod world;

use std::time::{Duration, Instant};

use render::Renderer;
use time::FrameCounter;
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use world::World;

const FRAMES_PER_SECOND: u64 = 60;
const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;
const FRAME_DURATION: Duration = Duration::from_nanos(NANOSECONDS_PER_SECOND / FRAMES_PER_SECOND);

fn main() {
    let options = get_options();

    let event_loop = EventLoop::new();
    let mut renderer = Renderer::new(&event_loop, options.windowed);

    let mut world = World::new(100, 25, 100);
    renderer.update_block_cache(world.block_positions());

    let mut last_instant = Instant::now();
    let mut fps_counter = FrameCounter::new(last_instant);

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
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta: (dx, dy) },
                ..
            } => {
                world.update_camera_direction(dx as f32, dy as f32);
            }
            Event::RedrawRequested(window_id) if window_id == renderer.window_id() => {
                renderer.set_viewport();
            }
            Event::MainEventsCleared => {
                let mut current_instant = Instant::now();
                while current_instant - last_instant > FRAME_DURATION {
                    world.update();
                    last_instant += FRAME_DURATION;
                    current_instant = Instant::now();
                }

                renderer.set_camera(world.camera());
                renderer.clear();

                renderer.draw_cubes();
                renderer.draw_skybox();

                renderer.present();
                fps_counter.finish_frame(current_instant);
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
