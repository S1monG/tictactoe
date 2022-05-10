use tictactoe::{Board};

mod window;

fn main() {
    println!("Hello, world!");

    let mut b = Board::new();
    println!("Expect 0, got {}", b.is_win());
    b.update(1, 0);
    b.update(1, 4);
    b.update(1, 8);
    b.update(-1, 1);
    println!("Expect 1, got {}", b.is_win());

    b.print_board();

    window::start();

}
