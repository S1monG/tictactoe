// calculates the optimal place for tictactoe using the minimax algorithm
use crate::Board;
use std::{cmp::{min, max}, os::windows::process};

// symbol is who the algorithm will calculate the optimal solution for.
// the algorith asumes that it is the given symbols turn to move.
pub fn optimal_move(b: &Board, symbol: isize) -> usize {
    if b.is_full() {
        std::process::exit(1)
    }

    match symbol {
        1 => optimal_move_x(b),
        -1 => optimal_move_o(b),
        _ => {eprintln!("symbol other than 1 or -1 given to the optimal_move function"); 
                std::process::exit(1)},
    }
}

fn optimal_move_x(b: &Board) -> usize {
    let mut best_move = 0;
    let mut best_score = -1;
    for i in 0..9 {
        // check if the spot is empty
        if b.board[i] == 0 {
            let mut board_clone = b.clone();
            board_clone.update(1, i);
            let new_score = minimax(board_clone, true, 1);
            // update score if it is better
            if new_score > best_score {
                best_score = new_score;
                best_move = i;
            }
        }
    }

    best_move
}

fn optimal_move_o(b: &Board) -> usize {
    let mut best_move = 0;
    let mut best_score = 1;
    for i in 0..9 {
        let mut board_clone = b.clone();
        board_clone.update(-1, i);
        let new_score = minimax(board_clone, false, -1);
        if new_score < best_score {
            best_score = new_score;
            best_move = i;
        }
    }

    best_move
}

// helper function used by optimal_move
fn minimax(mut b: Board, is_mini: bool, symbol: isize) -> isize {
    if b.is_win() != 0 {
        b.is_win()
    } else if is_mini {
        let mut lowest_score = 1;
        for i in 0..9 {
            if b.board[i] == 0 { 
                let mut board_clone = b.clone();
                board_clone.update(symbol, i);
                let new_score = minimax(board_clone, false, symbol);
                if new_score < lowest_score {
                    lowest_score = new_score;
                }
            }
        }
        lowest_score
    } else {
        let mut highest_score = -1;
        for i in 0..9 {
            if b.board[i] == 0 { 
                let mut board_clone = b.clone();
                board_clone.update(-1, i);
                let new_score = minimax(board_clone, true, symbol);
                if new_score > highest_score {
                    highest_score = new_score;
                }
            }
        }
        highest_score
    }
}



