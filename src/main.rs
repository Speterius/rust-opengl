use glium::{glutin, Surface};
use std::time::{Instant, Duration};

const WIDTH: u32 = 900;
const HEIGHT: u32 = 700;
const TIME_PER_FRAME: u64 = 16_666_667;


fn main() {
    println!("Initializing GLIUM window context.");

    // Event handling
    let mut event_loop = glutin::event_loop::EventLoop::new();

    // Application parameter specifier:
    let window_builder = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .with_title("Open GL Hello World!");

    // Open GL context specifier:
    let context_builder = glutin::ContextBuilder::new();

    // Display object: OpenGL window
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    // Infinite loop to keep the program running
    event_loop.run(move |ev, _, control_flow| {

        // Render stuff:
        let mut target = display.draw();
        target.clear_color(0.1, 0.19, 0.43, 1.0);
        target.finish().unwrap();

        // Limit flow to 60 FPS
        let next_frame_time = Instant::now() + Duration::from_nanos(TIME_PER_FRAME);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Handle events
        match ev {
            glutin::event::Event::WindowEvent {event, ..} => {
                match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    _ => return
                }
            },
            _ => {},
        }
    });
}
