fn main() {
    println!("Initializing SDL context.");
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Open GL Window WIP", 900, 700)
        .resizable()
        .build()
        .unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    // Infinite loop to keep the program running
    'main_loop: loop {

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main_loop,
                _ => {}
            }
        }

        // Render window contents

    }
}
