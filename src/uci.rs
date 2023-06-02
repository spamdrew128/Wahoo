use crate::board_representation::{Board, START_FEN};

pub enum ProgramStatus {
    Run,
    Quit,
}

pub struct UciHandler {
    board: Board,
}

impl UciHandler {
    pub fn new() -> Self {
        Self {
            board: Board::from_fen(START_FEN),
        }
    }

    pub fn execute_instructions(&self) -> ProgramStatus {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("UCI Input Failure");

        ProgramStatus::Run
    }
}
