

struct Board {
    mut board: [9; Symbol]
}

impl Board {
    fn new() -> Self {
        Board {board : [9; Symbol::None]}
    }
}

enum Symbol {
    X,
    O,
    None,
}


