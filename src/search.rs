use crate::{board_representation::Board, movegen::MoveGenerator, time_management::SearchTimer, chess_move::Move, evaluation::{EvalScore, INF, evaluate}};

pub type Nodes = u64;
pub type Depth = i8;

#[derive(Debug)]
pub struct Searcher {
    timer: SearchTimer,
    node_count: Nodes,
    best_move: Move, // TODO: replace with PV Table
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            timer: SearchTimer::new(0),
            node_count: 0,
            best_move: Move::nullmove(),
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

    fn negamax(&mut self, board: &Board, depth: Depth) -> EvalScore {
        if depth == 0 {
            return evaluate(board);
        }

        self.node_count += 1;

        let mut generator = MoveGenerator::new();

        let mut best_score = -INF;
        let mut best_move = Move::nullmove();
        while let Some(mv) = generator.next(board) {
            let mut next_board = (*board).clone();
            let is_legal = next_board.try_play_move(mv);
            if !is_legal {
                continue;
            }

            let score = -self.negamax(&next_board, depth - 1);

            if score > best_score {
                best_score = score;
                best_move = mv;
            }
        }

        self.best_move = best_move;
        best_score
    }
}
