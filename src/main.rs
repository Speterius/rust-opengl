fn main() {
    println!("Initializing SDL window context.");
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("Open GL Window WIP", 900, 700)
        .opengl()                                           // Add openGL Flag
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();


    println!("Initializing OpenGL context.");
    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(0.133, 0.2353, 0.3019, 1.0);
    }

    // Infinite loop to keep the program running
    'main_loop: loop {

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    println!("Quit Event. Closing main loop.");
                    break 'main_loop;
                },
                _ => {}
            }
        }

        // Render window contents
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }

}
