pub mod camera;
pub mod common;
pub mod consts;
pub mod scene_object;
pub mod teapot;
pub mod types;

pub use camera::Camera;
pub use common::*;
pub use consts::*;
pub use glium::{glutin, Display, DrawParameters, Program, ProgramCreationError, Frame, IndexBuffer, Surface, VertexBuffer};
pub use glium::{implement_vertex, uniform};
pub use glutin::dpi::LogicalSize;
pub use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
pub use glutin::event_loop::{ControlFlow, EventLoop};
pub use glutin::window::WindowBuilder;
pub use scene_object::SceneObject;
pub use std::time::{Duration, Instant};
pub use types::*;


pub fn setup(
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
    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(depth_buffer)
        .with_vsync(true)
        .with_multisampling(4);

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
pub fn define_shaders(display: &glium::Display) -> Result<Program, ProgramCreationError> {
    let v_shader = include_str!(r"shaders/vertex_shader.glsl");
    let f_shader = include_str!(r"shaders/fragment_shader.glsl");
    Program::from_source(display, &v_shader, &f_shader, None)
}

/// Initalize a glium Frame object with a specified background Color
pub fn init_frame(display: &Display, color: Color) -> Frame {
    let mut frame = display.draw();
    frame.clear_color_and_depth((color.r, color.g, color.b, 1.0), 1.0);
    frame
}

pub fn render(
    frame: &mut Frame,
    object_list: &[Box<SceneObject>],
    camera: &Camera,
    program: &Program,
    draw_params: &DrawParameters,
) {
    // Camera stuff:
    let perspective = camera.get_perspective_matrix();
    let view = camera.get_view_matrix();

    // Light stuff:
    let light = [-1.0, 0.4, 0.9f32];

    // Object dependent stuff:
    for obj in object_list {
        let model = obj.get_model_matrix();

        let uniforms = glium::uniform! {
            model: model,
            view: view,
            perspective: perspective,
            u_light: light
        };

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
}

