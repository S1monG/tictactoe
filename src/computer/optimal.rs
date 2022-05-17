// calculates the optimal place for tictactoe using the minimax algorithm
use crate::Board;
use std::cmp::{min, max};

pub fn optimal_move(b: &Board, turn: isize) -> usize {
    let mut best_move = 0;
    let mut best_score;
    let is_mini;
    if turn == 1 {
        best_score = -1;
        is_mini = false;
    } else {
        best_score = 1;
        is_mini = true;
    }

    for i in 0..9 {
        if b.board[i] == 0 {
            let mut clone = b.clone();
            clone.update(turn, i);
            let score = minimax(clone, !is_mini);
            // greater or equal to eliminate the situation where there is no good move and best_move never is changed
            if (is_mini && score <= best_score) || (!is_mini && score >= best_score){
                best_score = score;
                best_move = i;
            }
        }
    }

    best_move
}

fn minimax(mut b: Board, is_mini: bool) -> isize {
    let score = b.is_win();
    if score != 0 {
        score
    } else if b.is_full() {
        0
    } else if is_mini {
        let mut best_score = 1;
        for i in 0..9 {
            if b.board[i] == 0 {
                b.update(-1, i);
                let score = minimax(b.clone(), !is_mini);
                if score <= best_score {
                    best_score = score;
                }
            }
        }
        best_score
    } else {
        let mut best_score = -1;
        for i in 0..9 {
            if b.board[i] == 0 {
                b.update(1, i);
                let score = minimax(b.clone(), is_mini);
                if score >= best_score {
                    best_score = score;
                }
            }
        }
        best_score
    }
}



