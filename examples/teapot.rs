use opengl_window::*;

// Some Application settings
const WIDTH: u32 = 900;
const HEIGHT: u32 = 700;
const TIME_PER_FRAME: u64 = 16_666_667;
const DEPTH_BUFFER: u8 = 24;
const FOV: u32 = 60;

fn main() {

    // Display setup:
    let (display, event_loop, program, draw_params) =
        setup(WIDTH, HEIGHT, "OpenGl Hello World.", DEPTH_BUFFER);

    // Allocate teapots:
    let teapot1 = SceneObject::teapot(&display);
    let teapot2 = SceneObject::teapot(&display);

    let mut object_list = vec![Box::new(teapot1), Box::new(teapot2)];

    // Make camera:
    let mut camera = Camera::new(
        Vector3::new(0.0, -2.0, 2.0),
        Vector3::new(0.0, 1.0, -1.0),
        (WIDTH, HEIGHT),
        FOV,
    );

    // Main loop:
    event_loop.run(move |event, _, control_flow| {
        // Limit frame rate
        let next_frame_time = Instant::now() + Duration::from_nanos(TIME_PER_FRAME);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Handle events
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(_physical_size) => (),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                    ..
                } => match (virtual_code, state) {
                    (VirtualKeyCode::Escape, _) => *control_flow = ControlFlow::Exit,
                    (VirtualKeyCode::W, ElementState::Pressed) => {
                        println!("Pressed W");
                        object_list[0].scale_up();
                    }
                    (VirtualKeyCode::S, ElementState::Pressed) => {
                        println!("Pressed S");
                        object_list[0].scale_down();
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::RedrawRequested(_) => (),
            _ => return,
        }

        /* Rendering process */

        let mut frame = init_frame(&display, BLACK);

        // If the frame dimensions changed -> update the cameras dimensions.
        camera.update_resolution(frame.get_dimensions());

        // Do the rendering
        render(&mut frame, &object_list, &camera, &program, &draw_params);

        // Do the swapchain
        frame.finish().expect("Couldn't swap buffers.");
    });
}