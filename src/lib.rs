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

pub mod computer;

pub struct Board {
    board: [isize; 9],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [0; 9],
        }
    }

    pub fn is_win(&self) -> isize {
        for line in WINNING_LINES {
            if self.board[line[0]] == self.board[line[1]]
                && self.board[line[1]] == self.board[line[2]]
            {
                return self.board[line[0]];
            }
        }

        0
    }

    pub fn turn(&self) -> isize {
        let s: isize = self.board.iter().sum();
        if s <= 0 {
            1
        } else {
            -1
        }
    }

    pub fn update(&mut self, symbol: isize, index: usize) {
        self.board[index] = symbol;
    }

    pub fn clear(&mut self) {
        self.board = [0; 9];
    }

    pub fn print_board(&self) {
        for n in 0..3 {
            for i in (n*3)..(n*3 + 3) {
                if self.board[i] == 0 {
                    print!("_ ");
                } else {
                    if self.board[i] == 1 {
                        print!("X ");
                    } else {
                        print!("O ");
                    }
                }
            }
            println!("");
        }
    }
}

/* X is represented by 1
   O is represented by -1
and empty is represented by 0 
I could use an enum to represent X O and empty but it would create slower calculations and more overhead
considering i am going to find the optimal solution i want it to be efficient*/