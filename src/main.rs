mod consts;
mod teapot;
mod types;

pub use cgmath::{Deg, PerspectiveFov, Rad};
pub use consts::*;
pub use glium::{
    glutin, Display, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer,
};
pub use glutin::event_loop::EventLoop;
pub use std::time::{Duration, Instant};
use teapot::{INDICES, NORMALS, VERTICES};
pub use types::*;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 700;
const TIME_PER_FRAME: u64 = 16_666_667;
const DEPTH_BUFFER: u8 = 24;

#[derive(Copy, Clone)]
pub struct Color {
    r: Scalar,
    g: Scalar,
    b: Scalar,
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: (Scalar, Scalar, Scalar),
}

glium::implement_vertex!(Vertex, position);

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    normal: (Scalar, Scalar, Scalar),
}

glium::implement_vertex!(Normal, normal);

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
fn define_shaders(display: &glium::Display) -> Program {
    let v_shader = include_str!("vertex_shader.glsl");
    let f_shader = include_str!("fragment_shader.glsl");
    Program::from_source(display, &v_shader, &f_shader, None).expect("Couldn't compile shaders.")
}

/// Builds the frame specific matrix4 transform:
fn build_view_matrix(width: u32, height: u32, fov_deg: f32) -> [[f32; 4]; 4] {
    let fov = fov_deg * std::f32::consts::PI / 180.0;
    let aspect_ratio = height as Scalar / width as Scalar;
    let zfar = 1024.0;
    let znear = 0.1;
    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    ]
}

/// Initalize a glium Frame object with a specified background Color
fn init_frame(display: &Display, color: Color) -> Frame {
    let mut frame = display.draw();
    frame.clear_color_and_depth((color.r, color.g, color.b, 1.0), 1.0);
    frame
}

fn render(display: &Display, obj: &Object, program: &Program, draw_params: &DrawParameters) {
    // Render stuff:
    let mut frame = init_frame(display, BLACK);
    let (width, height) = frame.get_dimensions();
    let perspective = build_view_matrix(width, height, 60.0);
    let light = [-1.0, 0.4, 0.9f32];

    let u_matrix = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 5.0, 1.0f32],
    ];

    let uniforms = glium::uniform! {
        u_matrix: u_matrix,
        u_light: light,
        perspective: perspective
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

    // Swapchain:
    frame.finish().expect("Failed to draw frame");
}

struct Object {
    vertex_bfr: VertexBuffer<Vertex>,
    normal_bfr: VertexBuffer<Normal>,
    ind_bfr: IndexBuffer<u16>,
}

impl Object {
    fn new(display: &Display, vert: &[Vertex], norm: &[Normal], ind: &[u16]) -> Self {
        Self {
            vertex_bfr: VertexBuffer::new(display, vert).expect("Couldn't allocate Vertex Buffer."),
            normal_bfr: VertexBuffer::new(display, norm)
                .expect("Couldn't allocate Vertex Buffer for normals."),
            ind_bfr: IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, ind)
                .expect("Couldn't allocate Index buffer"),
        }
    }
}

fn main() {
    // Display setup:
    let (display, event_loop, program, draw_params) =
        setup(WIDTH, HEIGHT, "OpenGl Hello World.", DEPTH_BUFFER);

    // Allocate teapot:
    let teapot = Object::new(&display, &VERTICES, &NORMALS, &INDICES);

    // Main loop:
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
        render(&display, &teapot, &program, &draw_params);
    });
}
