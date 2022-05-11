use piston_window::*;
use tictactoe::Board;

const SIZE: f64 = 500.0;
const MARGIN: f64 = 10.0;
const FONT_SIZE: f64 = 120.0;
const BOX_PLACEMENT: [[f64; 2]; 9] = [
    [SIZE/6. + FONT_SIZE/3.,    SIZE/6. - FONT_SIZE/3.],
    [SIZE/2. + FONT_SIZE/3.,    SIZE/6. - FONT_SIZE/3.],
    [5.*SIZE/6. + FONT_SIZE/3., SIZE/6. - FONT_SIZE/3.],
    [SIZE/6. + FONT_SIZE/3.,    SIZE/2. - FONT_SIZE/3.],
    [SIZE/2. + FONT_SIZE/3.,    SIZE/2. - FONT_SIZE/3.],
    [5.*SIZE/6. + FONT_SIZE/3., SIZE/2. - FONT_SIZE/3.],
    [SIZE/6. + FONT_SIZE/3.,    5.*SIZE/6. - FONT_SIZE/3.],
    [SIZE/2. + FONT_SIZE/3.,    5.*SIZE/6. - FONT_SIZE/3.],
    [5.*SIZE/6. + FONT_SIZE/3., 5.*SIZE/6. - FONT_SIZE/3.],
];

pub fn start() {
    let mut window: PistonWindow = WindowSettings::new("TICTACTOE", [SIZE as u32, SIZE as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_max_fps(30);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let mut glyphs = window.load_font(font).unwrap();


    let mut b = Board::new();
    let mut last_pos = [0.0, 0.0];

    // while loop is needed or else the window closes when the lifetime of the window ends, aka this function ends
    while let Some(event) = window.next() {

         // save the last variable of the mouse curser in a variable and update it when the new one isn't None
         if let Some(pos) = event.mouse_cursor_args() {
            last_pos = pos;
        }
        // update the board with player move
        if let Some(input) = event.press_args() {
            if input == Button::Mouse(MouseButton::Left) {
                let (x, y) = (last_pos[0], last_pos[1]);

                if x < SIZE / 3.0 {
                    if y < SIZE / 3.0 {
                        b.update(1, 0);
                    } else if y < 2.0 * SIZE / 3.0 {
                        b.update(1, 1);
                    } else {
                        b.update(1, 2);
                    }
                } else if x < 2.0 * SIZE / 3.0 {
                    if y < SIZE / 3.0 {
                        b.update(1, 3);
                    } else if y < 2.0 * SIZE / 3.0 {
                        b.update(1, 4);
                    } else {
                        b.update(1, 5);
                    }
                } else {
                    if y < SIZE / 3.0 {
                        b.update(1, 6);
                    } else if y < 2.0 * SIZE / 3.0 {
                        b.update(1, 7);
                    } else {
                        b.update(1, 8);
                    }
                }
            } else if input == Button::Keyboard(Key::Q) {
                b.clear();
            }
        };

        // update board with computer move if it is the computers turn
        if b.turn() == -1 {
            b.update(-1, tictactoe::computer::random::random_move(&b));
        }

        // draw the board
        window.draw_2d(&event, |context, graphics, device| {
            clear([1.0; 4], graphics);

            let rectangle = Rectangle::new_round(color::BLACK, 5.0);

            rectangle.draw_from_to([SIZE / 3.0 - MARGIN / 2.0, 0.0 + MARGIN], [SIZE / 3.0 + MARGIN / 2.0, SIZE - MARGIN],
                &draw_state::DrawState::default(),
                context.transform,
                graphics,
            );
            rectangle.draw_from_to(  [2.0 * SIZE / 3.0 - MARGIN / 2.0, 0.0 + MARGIN], [2.0 * SIZE / 3.0 + MARGIN / 2.0, SIZE - MARGIN],
                &draw_state::DrawState::default(),
                context.transform,
                graphics,
            );
            rectangle.draw_from_to([0.0 + MARGIN, SIZE / 3.0 - MARGIN / 2.0], [SIZE - MARGIN, SIZE / 3.0 + MARGIN / 2.0],
                &draw_state::DrawState::default(),
                context.transform,
                graphics,
            );
            rectangle.draw_from_to([0.0 + MARGIN, 2.0 * SIZE / 3.0 - MARGIN / 2.0], [SIZE - MARGIN, 2.0 * SIZE / 3.0 + MARGIN / 2.0],
                &draw_state::DrawState::default(),
                context.transform,
                graphics,
            );

            for (i, s) in b.board.iter().enumerate() {
                if *s == -1 {
                    text::Text::new_color(color::BLACK, FONT_SIZE as u32).draw(
                        "O",
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(BOX_PLACEMENT[i][1], BOX_PLACEMENT[i][0]), 
                        graphics
                    ).unwrap();
                } else if *s == 1 {
                    text::Text::new_color(color::BLACK, FONT_SIZE as u32).draw(
                        "X",
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(BOX_PLACEMENT[i][1], BOX_PLACEMENT[i][0]), 
                        graphics
                    ).unwrap();
                }
            }
        });
    }
}
