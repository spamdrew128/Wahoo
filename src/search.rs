use crate::{board_representation::Board, movegen::MoveGenerator, time_management::SearchTimer};

pub type Nodes = u64;

#[derive(Debug)]
pub struct Searcher {
    timer: SearchTimer,
    node_count: Nodes,
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            timer: SearchTimer::new(0),
            node_count: 0,
        }
    }

    pub fn go(&mut self, board: &mut Board, search_timer: SearchTimer) {
        self.timer = search_timer;
        let best_move = MoveGenerator::first_legal_move(board).unwrap();

        // search stuff

        self.reset();
    }

    fn reset(&mut self) {
        self.node_count = 0;
    }
}
