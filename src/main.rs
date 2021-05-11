use glium::{glutin, Surface};
use std::time::{Duration, Instant};

const WIDTH: u32 = 900;
const HEIGHT: u32 = 700;
const TIME_PER_FRAME: u64 = 16_666_667;

type Scalar = f64;

#[derive(Copy, Clone)]
struct Vertex {
    position: [Scalar; 2],
}

glium::implement_vertex!(Vertex, position);

fn define_shaders() -> (String, String) {
    let vertex_shader: String = String::from(
        r#"
        #version 140

        in vec2 position;

        uniform float dx;

        void main() {

            vec2 pos = position;
            gl_Position = vec4(pos.x+dx, pos.y, 0.0, 1.0);

        }
    "#,
    );

    let fragment_shader: String = String::from(
        r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#,
    );

    (vertex_shader, fragment_shader)
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

    // Infinite loop to keep the program running
    event_loop.run(move |ev, _, control_flow| {
        // Limit flow to 60 FPS
        let next_frame_time = Instant::now() + Duration::from_nanos(TIME_PER_FRAME);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Change animation variable:
        let t = Instant::now().duration_since(start).as_secs_f32();
        let x = 0.5 * (0.5 * t * 2.0 * std::f32::consts::PI).sin();

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

        // Clear screen to blue:
        target.clear_color(0.1, 0.19, 0.43, 1.0);

        // Shaders:
        let (vertex_shader, fragment_shader) = define_shaders();
        let program =
            glium::Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();

        // Draw call:
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniform! {dx: x},
                &Default::default(),
            )
            .unwrap();

        // Swapchain:
        target.finish().unwrap();
    });
}
