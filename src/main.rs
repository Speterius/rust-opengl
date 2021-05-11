mod consts;
mod scene_object;
mod teapot;
mod types;
mod camera;
mod common;

pub use consts::*;
pub use common::*;
pub use camera::Camera;
pub use glium::{glutin, Display, DrawParameters, Program, ProgramCreationError};
pub use glium::{implement_vertex, uniform};
pub use glium::{Frame, IndexBuffer, Surface, VertexBuffer};
pub use glutin::dpi::LogicalSize;
pub use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{ElementState, Event, WindowEvent, KeyboardInput, VirtualKeyCode};
pub use glutin::window::WindowBuilder;
pub use scene_object::SceneObject;
pub use std::time::{Duration, Instant};
pub use types::*;

// Some Application settings
const WIDTH: u32 = 900;
const HEIGHT: u32 = 700;
const TIME_PER_FRAME: u64 = 16_666_667;
const DEPTH_BUFFER: u8 = 24;
const FOV: u32 = 60;

fn setup(
    width: u32,
    height: u32,
    title: &str,
    depth_buffer: u8,
) -> (Display, EventLoop<()>, Program, DrawParameters) {
    // Event handling
    let event_loop = EventLoop::new();

    // Application parameter specifier:
    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(width, height))
        .with_title(title);

    // Open GL context specifier:
    let context_builder = glutin::ContextBuilder::new().with_depth_buffer(depth_buffer);

    // Display object: OpenGL window
    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("Failed to build GLIUM display.");

    // Define the shaders:
    let program = define_shaders(&display).expect("Failed to compile shaders.");

    // Draw parameters:
    let draw_params = DrawParameters {
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    (display, event_loop, program, draw_params)
}

/// Reads the GLSL files as &str and feeds them to glium::Program
fn define_shaders(display: &glium::Display) -> Result<Program, ProgramCreationError> {
    let v_shader = include_str!(r"shaders/vertex_shader.glsl");
    let f_shader = include_str!(r"shaders/fragment_shader.glsl");
    Program::from_source(display, &v_shader, &f_shader, None)
}

/// Initalize a glium Frame object with a specified background Color
fn init_frame(display: &Display, color: Color) -> Frame {
    let mut frame = display.draw();
    frame.clear_color_and_depth((color.r, color.g, color.b, 1.0), 1.0);
    frame
}

fn render(frame: &mut Frame, obj: &SceneObject, camera: &Camera, program: &Program, draw_params: &DrawParameters) {

    // Generate the model, view and perspective matrices:
    let perspective = camera.get_perspective_matrix();
    let view = camera.get_view_matrix();
    let model = obj.get_model_matrix();

    // Add a light source:
    let light = [-1.0, 0.4, 0.9f32];

    let uniforms = glium::uniform! {
        model: model,
        view: view,
        perspective: perspective,
        u_light: light
    };

    // Draw calls:
    frame
        .draw(
            (&obj.vertex_bfr, &obj.normal_bfr),
            &obj.ind_bfr,
            program,
            &uniforms,
            draw_params,
        )
        .expect("Failed to draw object");
}

fn main() {
    // Display setup:
    let (display, event_loop, program, draw_params) =
        setup(WIDTH, HEIGHT, "OpenGl Hello World.", DEPTH_BUFFER);

    // Allocate teapot:
    let teapot = SceneObject::teapot(&display);

    // Make camera:
    let mut camera = Camera::new(Vector3::new(0.0, -2.0, 2.0), Vector3::new(0.0, 1.0, -1.0), (WIDTH, HEIGHT), FOV);

    // Main loop:
    event_loop.run(move |event, _, control_flow| {
        // Limit frame rate
        let next_frame_time = Instant::now() + Duration::from_nanos(TIME_PER_FRAME);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Handle events
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, .. } => match event {
                WindowEvent::Resized(physical_size) => println!("Resize event"),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
                    .. } => match (virtual_code, state) {
                        (VirtualKeyCode::Escape, _) => *control_flow = ControlFlow::Exit,
                        (VirtualKeyCode::W, ElementState::Pressed) => println!("Pressed W!"),
                        _ => (),
                    },
                _ => ()
            },
            Event::RedrawRequested(_) => (),
            _ => return,
        }

        let mut frame = init_frame(&display, BLACK);

        // If the frame dimensions changed -> update the cameras dimensions.
        camera.update_resolution(frame.get_dimensions());

        // Do the rendering
        render(&mut frame, &teapot, &camera, &program, &draw_params);

        // Do the swapchain
        frame.finish().expect("Couldn't swap buffers.");
    });
}
