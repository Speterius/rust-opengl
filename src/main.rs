use glium::{glutin, Surface, Program, Frame};
use std::time::{Duration, Instant};

const WIDTH: u32 = 900;
const HEIGHT: u32 = 700;
const TIME_PER_FRAME: u64 = 16_666_667;

type Scalar = f32;

#[derive(Copy, Clone)]
struct Vertex {
    position: [Scalar; 2],
}

#[derive(Copy, Clone)]
struct Color {
    r: Scalar,
    g: Scalar,
    b: Scalar,
}

const WHITE: Color = Color {r: 1.0, g: 1.0, b: 1.0};
const BLACK: Color = Color {r: 0.0, g: 0.0, b: 0.0};
const BLUE: Color = Color {r: 0.2, g: 0.2, b: 0.7};

fn clear_target(target: &mut Frame, color: Color) {
    target.clear_color(color.r, color.g, color.b, 1.0);
}

glium::implement_vertex!(Vertex, position);

/// Reads the GLSL files as &str and feeds them to glium::Program
fn define_shaders(display: &glium::Display) -> Program {
    let v_shader = include_str!("vertex_shader.glsl");
    let f_shader = include_str!("fragment_shader.glsl");
    Program::from_source(display, &v_shader, &f_shader, None).unwrap()
}

fn main() {
    println!("Initializing GLIUM window context.");

    // Event handling
    let event_loop = glutin::event_loop::EventLoop::new();

    // Application parameter specifier:
    let window_builder = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .with_title("Open GL Hello World!");

    // Open GL context specifier:
    let context_builder = glutin::ContextBuilder::new();

    // Display object: OpenGL window
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    // Define triangle
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Some animation variables
    let start = Instant::now();

    // Define the shaders:
    let program = define_shaders(&display);

    // Infinite loop to keep the program running
    event_loop.run(move |ev, _, control_flow| {

        // Limit frame rate
        let next_frame_time = Instant::now() + Duration::from_nanos(TIME_PER_FRAME);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Change animation variable:
        let t = Instant::now().duration_since(start).as_secs_f32();
        let dx = 0.5 * (0.5 * t * 2.0 * std::f32::consts::PI).sin();

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

        // Render stuff:
        let mut target = display.draw();

        // Clear screen to background color:
        clear_target(&mut target, BLACK);

        // Draw call:
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniform! {dx: dx},
                &Default::default(),
            )
            .unwrap();

        // Swapchain:
        target.finish().unwrap();
    });
}
