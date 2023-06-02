use crate::board_representation::{Board, START_FEN};

struct Uci {
    board: Board,
}

impl Uci {
    fn new() -> Self {
        Self { board: Board::from_fen(START_FEN) }
    }

    fn await_instructions(&self) {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("UCI Input Failure");
    }
}
