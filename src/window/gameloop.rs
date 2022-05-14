use super::{functions, BOX_PLACEMENT, FONT_SIZE, MARGIN, SIZE};
use piston_window::*;
use piston_window::Loop::Idle;
use tictactoe::Board;

pub fn start() {
    let mut window: PistonWindow = WindowSettings::new("TICTACTOE", [SIZE as u32, SIZE as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_lazy(true);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let mut glyphs = window.load_font(font).unwrap();

    let mut b = Board::new();
    let mut last_pos = [0.0, 0.0];


    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        
        e.mouse_cursor(|xs| last_pos = xs);

        if let Some(input) = e.press_args() {
            if input == Button::Mouse(MouseButton::Left) {
                let (x, y) = (last_pos[1], last_pos[0]);
                functions::update_board_with_pos(&mut b, x, y);
            } else if input == Button::Keyboard(Key::Q) {
                b.clear();
            }
        };

        if b.turn() == -1 {
            b.update(-1, tictactoe::computer::random::random_move(&b));
        }
        
        functions::draw_board(&mut window, &e);
        functions::draw_symbols(&mut window, &e, &b, &mut glyphs);

        b.print_board();

    }

    /* // while loop is needed or else the window closes when the lifetime of the window ends, aka this function ends
    while let Some(event) = window.next() {
        // save the last variable of the mouse curser in a variable and update it when the new one isn't None
        if let Some(pos) = event.mouse_cursor_args() {
            last_pos = pos;
        }
        // update the board with player move
        if let Some(input) = event.press_args() {
            if input == Button::Mouse(MouseButton::Left) {
                let (x, y) = (last_pos[1], last_pos[0]);
                functions::update_board_with_pos(&mut b, x, y);
            } else if input == Button::Keyboard(Key::Q) {
                b.clear();
            }
        };

        // update board with computer move if it is the computers turn
        if b.turn() == -1 {
            b.update(-1, tictactoe::computer::random::random_move(&b));
            b.print_board();
        }

        
        functions::draw_board(&mut window, &event)
        //functions::draw_symbols(&mut window, &event, &b, &mut glyphs);
    } */
}
