use piston_window::*;

pub fn start() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [500, 500])
        .exit_on_esc(true)
        .build()
        .unwrap();

// while loop is needed or else the window closes when the lifetime of the window ends, aka this function ends
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}