use super::Board;
use macroquad::prelude::*;
use std::time::Duration;

pub const WIDTH: f32 = 500.;
pub const HEIGHT: f32 = 500.;
pub const THICKNESS: f32 = 10.;
pub const FONT_SIZE: f32 = 100.;

pub fn get_box(x: f32, y: f32) -> usize {
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

pub async fn check_win(b: &mut Board) {
    match b.is_win() {
        1 => {
            clear_background(WHITE); 
            draw_text("You Win!", 20., HEIGHT/2., FONT_SIZE, BLACK);
            next_frame().await;
            std::thread::sleep(Duration::new(2, 0));
            b.clear();
        },
        -1 => {
            clear_background(WHITE); 
            draw_text("Computer Win!", 20., HEIGHT/2., FONT_SIZE, BLACK);
            next_frame().await;
            std::thread::sleep(Duration::new(2, 0));
            b.clear();
        },
        _ => (),
    }
}