use std::time::Instant;

use crate::{
    board_representation::Board,
    evaluation::{evaluate, EvalScore, EVAL_MAX, INF, MATE_THRESHOLD},
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
    seldepth: u8,
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
            seldepth: 0,
        }
    }

    fn report_search_info(&self, score: EvalScore, depth: Depth, stopwatch: Instant) {
        let elapsed = stopwatch.elapsed();
        let nps = (u128::from(self.node_count) * 1_000_000) / elapsed.as_micros().max(1);

        let score_str = if score >= MATE_THRESHOLD {
            let ply = EVAL_MAX - score;
            let score_value = (ply + 1) / 2;

            format!("mate {score_value}")
        } else if score <= -MATE_THRESHOLD {
            let ply = EVAL_MAX + score;
            let score_value = (ply + 1) / 2;

            format!("mate -{score_value}")
        } else {
            format!("cp {score}")
        };

        print!("info ");
        println!(
            "score {score_str} nodes {} time {} nps {nps} depth {depth} seldepth {} pv {}",
            self.node_count,
            elapsed.as_millis(),
            self.seldepth,
            self.pv_table.pv_string()
        );
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

        let mut best_move = MoveGenerator::first_legal_move(board).unwrap();
        while depth <= self.depth_limit.unwrap_or(MAX_DEPTH) {
            self.seldepth = 0;

            let score = self.negamax(board, depth, 0, -INF, INF);

            if self.out_of_time {
                break;
            }

            self.report_search_info(score, depth, stopwatch);
            best_move = self.pv_table.best_move();

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
        self.pv_table.set_length(ply);

        let is_root: bool = ply == 0;
        let is_drawn: bool =
            self.zobrist_stack.twofold_repetition(board.halfmoves) || board.fifty_move_draw();

        if !is_root && is_drawn {
            return 0;
        }

        if depth == 0 {
            return self.qsearch(board, ply, alpha, beta);
        }

        if (self.node_count % Self::TIMER_CHECK_FREQ == 0) && self.timer.is_expired() {
            self.out_of_time = true;
            return 0;
        }

        self.seldepth = ply;

        let mut generator = MoveGenerator::new();

        let mut best_score = -INF;
        let mut moves_played = 0;
        while let Some(mv) = generator.next::<true>(board) {
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

    fn qsearch(
        &mut self,
        board: &Board,
        ply: Ply,
        mut alpha: EvalScore,
        beta: EvalScore,
    ) -> EvalScore {
        if (self.node_count % Self::TIMER_CHECK_FREQ == 0) && self.timer.is_expired() {
            self.out_of_time = true;
            return 0;
        }

        self.seldepth = ply;

        let stand_pat = evaluate(board);
        if stand_pat >= beta {
            return stand_pat;
        }

        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let mut generator = MoveGenerator::new();

        let mut best_score = stand_pat;
        while let Some(mv) = generator.next::<false>(board) {
            let mut next_board = (*board).clone();
            let is_legal = next_board.try_play_move(mv, &mut self.zobrist_stack);
            if !is_legal {
                continue;
            }
            if self.out_of_time {
                return 0;
            }
            self.node_count += 1;

            let score = -self.qsearch(&next_board, ply + 1, -beta, -alpha);

            self.zobrist_stack.revert_state();

            if score > best_score {
                best_score = score;

                if score >= beta {
                    break;
                }

                if score > alpha {
                    alpha = score;
                }
            }
        }

        best_score
    }
}
