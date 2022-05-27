use crate::Board;
use rand::seq::SliceRandom;

pub fn random_move(b: &Board) -> usize {
    let mut empty_boxes: Vec<usize> = Vec::new();
    for (i, t) in b.board.iter().enumerate() {
        if *t == 0 {
            empty_boxes.push(i);
        }
    }

    *empty_boxes
        .choose(&mut rand::thread_rng())
        .expect("No empty places left")
}
