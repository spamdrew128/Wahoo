use std::time::Instant;

use crate::{
    board_representation::Board,
    evaluation::{evaluate, EvalScore, EVAL_MAX, INF},
    movegen::MoveGenerator,
    pv_table::PvTable,
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
    pv_table: PvTable,

    depth_limit: Option<Depth>,
    out_of_time: bool,
    node_count: Nodes,
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
            pv_table: PvTable::new(),
            depth_limit,
            out_of_time: false,
            node_count: 0,
        }
    }

    fn report_search_info(&self, score: EvalScore, depth: Depth, stopwatch: Instant) {
        let elapsed = stopwatch.elapsed().as_millis();
        let nps = (u128::from(self.node_count) * 1_000_000) / stopwatch.elapsed().as_micros().max(1);
        let nodes = self.node_count;
        let pv_str = self.pv_table.pv_string();

        print!("info ");
        println!("score cp {score} nodes {nodes} time {elapsed} nps {nps} depth {depth} pv{pv_str}");
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
        let mut depth: Depth = 1;

        while depth <= self.depth_limit.unwrap_or(MAX_DEPTH) {
            let score = self.negamax(board, depth, 0, -INF, INF);

            if self.out_of_time {
                break;
            }

            self.report_search_info(score, depth, stopwatch);

            depth += 1;
        }

        let best_move = if depth == 1 {
            // we didn't finish depth 1 search
            MoveGenerator::first_legal_move(board).unwrap()
        } else {
            self.pv_table.best_move()
        };

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
        self.pv_table.set_length(ply);

        let is_root: bool = ply == 0;

        if !is_root
            && (self.zobrist_stack.twofold_repetition(board.halfmoves) || board.fifty_move_draw())
        {
            return 0;
        }

        if depth == 0 {
            return evaluate(board);
        }

        if (self.node_count % Self::TIMER_CHECK_FREQ == 0) && self.timer.is_expired() {
            self.out_of_time = true;
            return 0;
        }

        let mut generator = MoveGenerator::new();

        let mut best_score = -INF;
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

                if score >= beta {
                    break;
                }

                if score > alpha {
                    alpha = score;
                    self.pv_table.update(ply, mv);
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

        best_score
    }
}
