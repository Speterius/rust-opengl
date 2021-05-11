mod consts;
mod types;

pub use cgmath::{Deg, PerspectiveFov, Rad};
pub use consts::*;
pub use glium::uniforms::{EmptyUniforms, UniformsStorage};
pub use glium::{glutin, Display, DrawParameters, Frame, Program, Surface};
pub use glutin::event_loop::EventLoop;
pub use std::time::{Duration, Instant};
pub use types::*;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 700;
const FOV: u32 = 90;
const TIME_PER_FRAME: u64 = 16_666_667;

#[derive(Copy, Clone)]
pub struct Color {
    r: Scalar,
    g: Scalar,
    b: Scalar,
}

/// Initalize a glium Frame object with a specified background Color
fn init_frame(display: &Display, color: Color) -> Frame {
    let mut frame = display.draw();
    frame.clear_color(color.r, color.g, color.b, 1.0);
    frame.clear_depth(1.0);
    frame
}

/// Reads the GLSL files as &str and feeds them to glium::Program
fn define_shaders(display: &glium::Display) -> Program {
    let v_shader = include_str!("vertex_shader.glsl");
    let f_shader = include_str!("fragment_shader.glsl");
    Program::from_source(display, &v_shader, &f_shader, None).expect("Couldn't compile shaders.")
}

/// Builds the frame dependent Uniforms that the shaders use:
/// At the moment it's the matrix4 view transform.
fn build_uniforms(frame: &Frame) -> FrameUniforms {
    let (width, height) = frame.get_dimensions();

    let projection = PerspectiveFov::<Scalar> {
        fovy: Rad::<Scalar>::from(Deg::<Scalar>(FOV as f32)),
        aspect: width as Scalar / height as Scalar,
        near: 1.0,
        far: 1e3,
    };

    let view = Matrix4::from_translation(Vector3::new(0.0, 0.0, -10.0));

    let matrix = (Matrix4::from(projection) * view).into();

    glium::uniform! {
        matrix: matrix
    }
}

fn setup(
    width: u32,
    height: u32,
    title: &str,
    depth_buffer: u8,
) -> (Display, EventLoop<()>, Program, DrawParameters) {
    // Event handling
    let event_loop = glutin::event_loop::EventLoop::new();

    // Application parameter specifier:
    let window_builder = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(width, height))
        .with_title(title);

    // Open GL context specifier:
    let context_builder = glutin::ContextBuilder::new().with_depth_buffer(depth_buffer);

    // Display object: OpenGL window
    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("Failed to build glium display.");

    // Define the shaders:
    let program = define_shaders(&display);

    // Draw parameters:
    let draw_params = DrawParameters {
        backface_culling: glium::BackfaceCullingMode::CullClockwise,
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    (display, event_loop, program, draw_params)
}

fn render(display: &Display) {
    // Render stuff:
    let frame = init_frame(display, BLUE);
    let _uniforms = build_uniforms(&frame);

    // Draw calls:
    // frame
    //     .draw(&cuboid, &cuboid, &program, &uniforms, &draw_params)
    //     .expect("Failed to draw cube.");

    // Swapchain:
    frame.finish().expect("Failed to draw frame");
}

fn main() {
    let (display, event_loop, program, draw_params) =
        setup(WIDTH, HEIGHT, "OpenGl Hello World.", 16);

    event_loop.run(move |ev, _, control_flow| {
        // Limit frame rate
        let next_frame_time = Instant::now() + Duration::from_nanos(TIME_PER_FRAME);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Handle events
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => {}
        }

        // Do the rendering
        render(&display);
    });
}
