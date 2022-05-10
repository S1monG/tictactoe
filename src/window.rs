use piston_window::*;
use tictactoe::Board;

const SIZE: f64 = 500.;
const MARGIN: f64 = 10.;

pub fn start() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [SIZE as u32, SIZE as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_max_fps(30);

    let mut b = Board::new();
    let mut left_mouse = false;
    let mut last_pos = [0.0, 0.0];

// while loop is needed or else the window closes when the lifetime of the window ends, aka this function ends
    while let Some(event) = window.next() {

        // draw the board
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics); 

            let rectangle = Rectangle::new_round(color::BLACK, 5.0);

            rectangle.draw_from_to(
                [SIZE/3.0 - MARGIN/2.0, 0.0 + MARGIN], [SIZE/3.0 + MARGIN/2.0, SIZE - MARGIN], 
                &draw_state::DrawState::default(), context.transform, graphics);
            rectangle.draw_from_to(
                [2.0*SIZE/3.0 - MARGIN/2.0, 0.0 + MARGIN], [2.0*SIZE/3.0 + MARGIN/2.0, SIZE - MARGIN], 
                &draw_state::DrawState::default(), context.transform, graphics);
            rectangle.draw_from_to(
                [0.0 + MARGIN, SIZE/3.0 - MARGIN/2.0], [SIZE - MARGIN, SIZE/3.0 + MARGIN/2.0], 
                &draw_state::DrawState::default(), context.transform, graphics);
            rectangle.draw_from_to(
                [0.0 + MARGIN, 2.0*SIZE/3.0 - MARGIN/2.0], [SIZE - MARGIN, 2.0*SIZE/3.0 + MARGIN/2.0], 
                &draw_state::DrawState::default(), context.transform, graphics);
        });

        // update the board with player move
        if let Some(input) = event.press_args() {
            if input == Button::Mouse(MouseButton::Left) {
                // mouse_cursor_args() does not return the position of the mouse cursor when there's currently a MouseButton event being processed
                // therfore i need to keep track if the left mouse has been pressed like this instead
                left_mouse = true;
                
            } else if input == Button::Keyboard(Key::Q) {
                b.clear();
                continue;
            }
        };

        /* spara sista mouse curser i en variable och uppdatera den när den nya inte är None */
        if let Some(pos) = event.mouse_cursor_args() {
            last_pos = pos;
        }

        if left_mouse {
            let (x, y) = (last_pos[0], last_pos[1]);

            println!("{}   {}", x, y);

            if x < SIZE/3.0 {
                if y < SIZE/3.0 {
                    b.update(1, 0)
                } else if y < 2.0*SIZE/3.0 {
                    b.update(1, 1)
                } else {
                    b.update(1, 2)
                }
            } else if x < 2.0*SIZE/3.0 {
                if y < SIZE/3.0 {
                    b.update(1, 3)
                } else if y < 2.0*SIZE/3.0 {
                    b.update(1, 4)
                } else {
                    b.update(1, 5)
                }
            } else {
                if y < SIZE/3.0 {
                    b.update(1, 6)
                } else if y < 2.0*SIZE/3.0 {
                    b.update(1, 7)
                } else {
                    b.update(1, 8)
                }
            }

            left_mouse = false;
            b.print_board();
        }

        // update board with computer move if it is the computers turn
        if b.turn() == -1 {
            b.update(-1, tictactoe::computer::random::random_move(&b, -1))
        }

    }
}