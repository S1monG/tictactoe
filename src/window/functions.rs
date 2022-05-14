use crate::window::{BOX_PLACEMENT, FONT_SIZE, MARGIN, SIZE};
use piston_window::*;
use tictactoe::Board;

pub fn draw_board(window: &mut PistonWindow, _event: &Event) {
    window.draw_2d(_event, |context, graphics, _device| {
        clear([1.0; 4], graphics);
        let rectangle = Rectangle::new_round(color::BLACK, 5.0);

        rectangle.draw_from_to(
            [SIZE / 3.0 - MARGIN / 2.0, 0.0 + MARGIN],
            [SIZE / 3.0 + MARGIN / 2.0, SIZE - MARGIN],
            &draw_state::DrawState::default(),
            context.transform,
            graphics,
        );
        rectangle.draw_from_to(
            [2.0 * SIZE / 3.0 - MARGIN / 2.0, 0.0 + MARGIN],
            [2.0 * SIZE / 3.0 + MARGIN / 2.0, SIZE - MARGIN],
            &draw_state::DrawState::default(),
            context.transform,
            graphics,
        );
        rectangle.draw_from_to(
            [0.0 + MARGIN, SIZE / 3.0 - MARGIN / 2.0],
            [SIZE - MARGIN, SIZE / 3.0 + MARGIN / 2.0],
            &draw_state::DrawState::default(),
            context.transform,
            graphics,
        );
        rectangle.draw_from_to(
            [0.0 + MARGIN, 2.0 * SIZE / 3.0 - MARGIN / 2.0],
            [SIZE - MARGIN, 2.0 * SIZE / 3.0 + MARGIN / 2.0],
            &draw_state::DrawState::default(),
            context.transform,
            graphics,
        );
    });
}

pub fn draw_symbols(window: &mut PistonWindow, event: &Event, b: &Board, glyphs: &mut Glyphs) {

    for (i, s) in b.board.iter().enumerate() {
        match *s {

            1 => window.draw_2d(event, |context, graphics, _device| {
                text::Text::new(FONT_SIZE as u32).draw(
                "X",
                glyphs,
                &context.draw_state,
                context
                    .transform
                    .trans(BOX_PLACEMENT[i][0], BOX_PLACEMENT[i][1]),
                graphics,
            )
            .unwrap()
            }),

            -1 => window.draw_2d(event, |context, graphics, _device| {
                text::Text::new(FONT_SIZE as u32).draw(
                "O",
                glyphs,
                &context.draw_state,
                context
                    .transform
                    .trans(BOX_PLACEMENT[i][0], BOX_PLACEMENT[i][1]),
                graphics,
            )
            .unwrap()
            }),

            _ => Some(()),

        };

    };
}

pub fn update_board_with_pos(b: &mut Board, x: f64, y: f64) {
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
}
