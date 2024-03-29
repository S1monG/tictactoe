use macroquad::prelude::*;
use tictactoe::computer::{optimal::optimal_move, random::random_move};
use tictactoe::functions::*;
use tictactoe::Board;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Tic Tac Toe"),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut b: Board = Board::new();

    loop {
        if b.is_full() {
            clear_background(WHITE);
            draw_text("draw!", 20., HEIGHT / 2., FONT_SIZE, BLACK);
            b.clear();
            continue;
        }

        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            println!("Bye for now!");
            std::process::exit(0);
        }
        if is_key_pressed(KeyCode::C) {
            b.clear();
        }
        //player X move
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let index = get_box(x, y);
            if b.board[index] == 0 {
                b.update(1, index);

                //check for win
                check_win(&mut b).await;
            }
        }

        // player O move
        if b.turn() == -1 && !b.is_full() {
            let index = optimal_move(&b, -1);
            //b.update(-1, random_move(&b));
            b.update(-1, index);

            // check for win
            check_win(&mut b).await;
        }

        //draw state
        clear_background(WHITE);
        draw_line(WIDTH / 3., 0., WIDTH / 3., HEIGHT, THICKNESS, BLACK);
        draw_line(
            2. * WIDTH / 3.,
            0.,
            2. * WIDTH / 3.,
            HEIGHT,
            THICKNESS,
            BLACK,
        );
        draw_line(0., HEIGHT / 3., WIDTH, HEIGHT / 3., THICKNESS, BLACK);
        draw_line(
            0.,
            2. * HEIGHT / 3.,
            WIDTH,
            2. * HEIGHT / 3.,
            THICKNESS,
            BLACK,
        );

        for i in 0..3 {
            for j in 0..3 {
                let symbol = b.board[j * 3 + i];
                if symbol == 1 {
                    draw_text(
                        "X",
                        i as f32 * WIDTH / 3. + FONT_SIZE / 2.,
                        j as f32 * WIDTH / 3. + FONT_SIZE,
                        FONT_SIZE,
                        BLACK,
                    );
                } else if symbol == -1 {
                    draw_text(
                        "O",
                        i as f32 * WIDTH / 3. + FONT_SIZE / 2.,
                        j as f32 * WIDTH / 3. + FONT_SIZE,
                        FONT_SIZE,
                        BLACK,
                    );
                }
            }
        }

        next_frame().await;
    }
}
