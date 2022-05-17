use tictactoe::Board;

const width: u32 = 500;
const height: u32 = 500;

use macroquad::prelude::*;

#[macroquad::main("Life")]
async fn main() {

    println!("{}        {}", w, h);

    let mut image = Image::gen_image_color(width as u16, height as u16, WHITE);

    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(WHITE);

        let w = image.width();
        let h = image.height();

        // update state ...


        //draw state

        for i in 0..buffer.len() {
            cells[i] = buffer[i];

            image.set_pixel(
                (i % w) as u32,
                (i / w) as u32,
                match buffer[i as usize] {
                    CellState::Alive => BLACK,
                    CellState::Dead => WHITE,
                },
            );
        }

        texture.update(&image);

        draw_texture(texture, 0., 0., WHITE);

        next_frame().await;
    }
}