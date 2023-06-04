use std::time::Instant;

use crate::{
    board_representation::Board,
    chess_move::Move,
    evaluation::{evaluate, EvalScore, INF},
    movegen::MoveGenerator,
    time_management::SearchTimer,
};

pub type Nodes = u64;
pub type Depth = i8;

#[derive(Debug)]
pub struct Searcher {
    timer: SearchTimer,
    out_of_time: bool,
    node_count: Nodes,
    best_move: Move, // TODO: replace with PV Table
}

fn report_search_info(score: EvalScore, nodes: Nodes, depth: Depth, stopwatch: Instant) {
    let elapsed = stopwatch.elapsed().as_millis();
    let nps = (u128::from(nodes) * 1_000_000) / stopwatch.elapsed().as_micros() ;
    print!("info ");
    println!("score cp {score} nodes {nodes} time {elapsed} nps {nps} depth {depth}");
}

impl Searcher {
    const TIMER_CHECK_FREQ: u64 = 1024;

    pub fn new() -> Self {
        Self {
            timer: SearchTimer::new(0),
            out_of_time: false,
            node_count: 0,
            best_move: Move::nullmove(),
        }
    }

    pub fn go(&mut self, board: &Board, search_timer: SearchTimer) {
        self.timer = search_timer;
        let stopwatch = std::time::Instant::now();
        let mut best_move = MoveGenerator::first_legal_move(board).unwrap();
        let mut depth: Depth = 1;

        loop {
            let score = self.negamax(board, depth);

            if self.out_of_time {
                break;
            }

            best_move = self.best_move;
            report_search_info(score, self.node_count, depth, stopwatch);

            depth += 1;
        }

        assert!(best_move.to() != best_move.from(), "INVALID MOVE");
        println!("bestmove {}", best_move.as_string());

        self.reset();
    }

    fn reset(&mut self) {
        self.out_of_time = false;
        self.node_count = 0;
    }

    fn negamax(&mut self, board: &Board, depth: Depth) -> EvalScore {
        if depth == 0 {
            return evaluate(board);
        }

        if (self.node_count % Self::TIMER_CHECK_FREQ == 0) && self.timer.is_expired() {
            self.out_of_time = true;
            return 0;
        }

        let mut generator = MoveGenerator::new();

        let mut best_score = -INF;
        let mut best_move = Move::nullmove();
        while let Some(mv) = generator.next(board) {
            let mut next_board = (*board).clone();
            let is_legal = next_board.try_play_move(mv);
            if !is_legal {
                continue;
            }
            if self.out_of_time {
                return 0;
            }
            self.node_count += 1;

            let score = -self.negamax(&next_board, depth - 1);

            if score >= best_score {
                // todo: change this to > once we have mate scores
                best_score = score;
                best_move = mv;
            }
        }

        self.best_move = best_move;
        best_score
    }
}
