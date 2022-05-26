// calculates the optimal place for tictactoe using the minimax algorithm
use crate::Board;
use super::random::random_move;
use std::{cmp::{min, max}, os::windows::process};

// symbol is who the algorithm will calculate the optimal solution for.
// the algorith asumes that it is the given symbols turn to move.
pub fn optimal_move(b: &Board, symbol: isize) -> usize {
    if b.is_full() {
        std::process::exit(1)
    }

    let mut best_move: Option<usize> = None;

    if symbol == 1 {
        let mut highest_score = -1;
        for i in 0..9 {
            if b.board[i] == 0 {
                let mut board_clone = b.clone();
                board_clone.update(1, i);
                let score = minimax(board_clone, true);
                if score > highest_score {
                    highest_score = score;
                    best_move = Some(i);
                }
            }
        }  
    } else if symbol == -1 {
        let mut lowest_score = 1;
        for i in 0..9 {
            if b.board[i] == 0 {
                let mut board_clone = b.clone();
                board_clone.update(-1, i);
                let score = minimax(board_clone, false);
                if score < lowest_score {
                    lowest_score = score;
                    best_move = Some(i);
                }
            }
        }  
    } else {
        eprintln!("symbol other than 1 or -1 given to the optimal_move() function"); 
        std::process::exit(1);
    }

    best_move.unwrap_or_else(|| random_move(b))
}

// helper function used by optimal_move
// whos turn it is can be derived from is_mini
fn minimax(mut b: Board, is_mini: bool) -> isize {
    if b.is_win() != 0 {
        b.is_win()
    } else if is_mini {
        let mut lowest_score = 1;
        for i in 0..9 {
            if b.board[i] == 0 { 
                let mut board_clone = b.clone();
                board_clone.update(-1, i);
                let new_score = minimax(board_clone, false);
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
                board_clone.update(1, i);
                let new_score = minimax(board_clone, true);
                if new_score > highest_score {
                    highest_score = new_score;
                }
            }
        }
        highest_score
    }
}




