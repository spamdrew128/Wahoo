use std::time::Instant;

use crate::{
    board_representation::Board,
    chess_move::Move,
    evaluation::{evaluate, EvalScore, EVAL_MAX, INF},
    movegen::MoveGenerator,
    time_management::SearchTimer,
    zobrist_stack::ZobristStack,
};

pub type Nodes = u64;
pub type Depth = i8;
pub type Ply = u8;
const MAX_DEPTH: Depth = i8::MAX;
pub const MAX_PLY: Ply = MAX_DEPTH as u8;

#[derive(Debug)]
pub struct Searcher {
    timer: SearchTimer,
    zobrist_stack: ZobristStack,

    depth_limit: Option<Depth>,
    out_of_time: bool,
    node_count: Nodes,
    best_move: Move, // TODO: replace with PV Table
}

fn report_search_info(score: EvalScore, nodes: Nodes, depth: Depth, stopwatch: Instant) {
    let elapsed = stopwatch.elapsed().as_millis();
    let nps = (u128::from(nodes) * 1_000_000) / stopwatch.elapsed().as_micros().max(1);
    print!("info ");
    println!("score cp {score} nodes {nodes} time {elapsed} nps {nps} depth {depth}");
}

impl Searcher {
    const TIMER_CHECK_FREQ: u64 = 1024;

    pub const fn new(
        timer: SearchTimer,
        zobrist_stack: ZobristStack,
        depth_limit: Option<Depth>,
    ) -> Self {
        Self {
            timer,
            zobrist_stack,
            depth_limit,
            out_of_time: false,
            node_count: 0,
            best_move: Move::nullmove(),
        }
    }

    pub fn bench(&mut self, board: &Board, depth: Depth) -> Nodes {
        self.timer = SearchTimer::new(999999999); // just some big number idc

        for d in 1..depth {
            self.negamax(board, d, 0, -INF, INF);
        }

        self.node_count
    }

    pub fn go(&mut self, board: &Board) {
        let stopwatch = std::time::Instant::now();
        let mut best_move = MoveGenerator::first_legal_move(board).unwrap();
        let mut depth: Depth = 1;

        while depth <= self.depth_limit.unwrap_or(MAX_DEPTH) {
            let score = self.negamax(board, depth, 0, -INF, INF);

            if self.out_of_time {
                break;
            }

            best_move = self.best_move;
            report_search_info(score, self.node_count, depth, stopwatch);

            depth += 1;
        }

        assert!(best_move.to() != best_move.from(), "INVALID MOVE");
        println!("bestmove {}", best_move.as_string());
    }

    fn negamax(
        &mut self,
        board: &Board,
        depth: Depth,
        ply: Ply,
        mut alpha: EvalScore,
        beta: EvalScore,
    ) -> EvalScore {
        if depth == 0 {
            return evaluate(board);
        }

        if (self.node_count % Self::TIMER_CHECK_FREQ == 0) && self.timer.is_expired() {
            self.out_of_time = true;
            return 0;
        }

        let is_root: bool = ply == 0;

        if !is_root
            && (self.zobrist_stack.twofold_repetition(board.halfmoves) || board.fifty_move_draw())
        {
            return 0;
        }

        let mut generator = MoveGenerator::new();

        let mut best_score = -INF;
        let mut best_move = Move::nullmove();
        let mut moves_played = 0;
        while let Some(mv) = generator.next(board) {
            let mut next_board = (*board).clone();
            let is_legal = next_board.try_play_move(mv, &mut self.zobrist_stack);
            if !is_legal {
                continue;
            }
            if self.out_of_time {
                return 0;
            }
            self.node_count += 1;
            moves_played += 1;

            let score = -self.negamax(&next_board, depth - 1, ply + 1, -beta, -alpha);

            self.zobrist_stack.revert_state();

            if score > best_score {
                best_score = score;
                best_move = mv;

                if score >= beta {
                    break;
                }

                if score > alpha {
                    alpha = score;
                }
            }
        }

        if moves_played == 0 {
            // either checkmate or stalemate
            return if board.king_sq().is_attacked(board) {
                -EVAL_MAX + i16::from(ply)
            } else {
                0
            };
        }

        self.best_move = best_move;
        best_score
    }
}
