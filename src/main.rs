use tictactoe::{Board, Symbol};

fn main() {
    println!("Hello, world!");

    let mut b = Board::new();
    println!("Expect None, got {:?}", b.is_win());
    b.update(Symbol::X, 0);
    b.update(Symbol::X, 4);
    b.update(Symbol::X, 8);
    b.update(Symbol::O, 1);
    println!("Expect X, got {:?}", b.is_win());

    b.print_board();
}
