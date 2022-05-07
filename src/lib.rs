const WINNING_LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [2, 4, 6],
    [0, 4, 8],
];

pub struct Board {
    board: [Symbol; 9],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [Symbol::None; 9],
        }
    }

    pub fn is_win(&self) -> Symbol {
        for line in WINNING_LINES {
            if self.board[line[0]] == self.board[line[1]]
                && self.board[line[1]] == self.board[line[2]]
            {
                return self.board[line[0]];
            }
        }

        Symbol::None
    }

    pub fn update(&mut self, symbol: Symbol, index: usize) {
        self.board[index] = symbol;
    }

    pub fn print_board(&self) {
        for n in 0..3 {
            for i in (n*3)..(n*3 + 3) {
                if self.board[i] == Symbol::None {
                    print!("_ ");
                } else {
                    print!("{:?} ", self.board[i]);
                }
            }
            println!("");
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Symbol {
    X,
    O,
    None,
}
