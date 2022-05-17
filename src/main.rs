use macroquad::prelude::*;
use tictactoe::Board;
use tictactoe::computer::{random::random_move, optimal::optimal_move};

const WIDTH: f32 = 500.;
const HEIGHT: f32 = 500.;
const THICKNESS: f32 = 10.;
const FONT_SIZE: f32 = 100.;

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

        // update state ...
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            println!("Bye for know!");
            std::process::exit(0);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let index = get_box(x, y);
            if b.board[index] == 0 {
                b.update(1, index);
            }
        }

        if b.turn() == -1 {
            b.update(-1, random_move(&b));
        }


        //draw state
        clear_background(WHITE);
        draw_line(WIDTH / 3., 0., WIDTH / 3., HEIGHT, THICKNESS, BLACK);
        draw_line(2. * WIDTH / 3., 0., 2. * WIDTH / 3., HEIGHT, THICKNESS, BLACK);
        draw_line(0., HEIGHT / 3., WIDTH, HEIGHT / 3., THICKNESS, BLACK);
        draw_line(0., 2. * HEIGHT / 3., WIDTH, 2. * HEIGHT / 3., THICKNESS, BLACK);

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

        //check win
        match b.is_win() {
            1 => draw_text("You Win!", WIDTH/2., HEIGHT/2., FONT_SIZE*3., BLACK),
            -1 => draw_text("Computer Win!", WIDTH/2., HEIGHT/2., FONT_SIZE*3., BLACK),
            _ => (),
        }

        next_frame().await;
    }
}

fn get_box(x: f32, y: f32) -> usize {
    if y < HEIGHT/3. {
        if x < WIDTH/3. {
            0
        } else if x < 2.*WIDTH/3. {
            1
        } else {
            2
        }
    } else if y < 2.*HEIGHT/3. {
        if x < WIDTH/3. {
            3
        } else if x < 2.*WIDTH/3. {
            4
        } else {
            5
        }
    } else {
        if x < WIDTH/3. {
            6
        } else if x < 2.*WIDTH/3. {
            7
        } else {
            8
        }
    }
}