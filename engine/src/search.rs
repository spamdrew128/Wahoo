use std::time::Instant;

use arrayvec::ArrayVec;

use crate::{
    board_representation::Board,
    chess_move::{Move, MAX_MOVECOUNT},
    evaluation::{evaluate, EvalScore, EVAL_MAX, INF, MATE_THRESHOLD},
    history_table::History,
    killers::Killers,
    movegen::MoveGenerator,
    pv_table::PvTable,
    time_management::{Milliseconds, SearchTimer},
    zobrist::ZobristHash,
    zobrist_stack::ZobristStack,
};

pub type Nodes = u64;
pub type Depth = i8;
pub type Ply = u8;
const MAX_DEPTH: Depth = i8::MAX;
pub const MAX_PLY: Ply = MAX_DEPTH as u8;

pub struct SearchResults {
    pub best_move: Move,
    pub score: EvalScore,
}

impl SearchResults {
    fn new(board: &Board) -> Self {
        Self {
            best_move: MoveGenerator::first_legal_move(board).unwrap(),
            score: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SearchLimit {
    Time(Milliseconds),
    Depth(Depth),
    Nodes(Nodes),
    None,
}

#[derive(Debug)]
pub struct Searcher {
    search_limit: SearchLimit,
    zobrist_stack: ZobristStack,
    pv_table: PvTable,
    history: History,
    killers: Killers,

    timer: Option<SearchTimer>,
    out_of_time: bool,
    node_count: Nodes,
    seldepth: u8,
}

impl Searcher {
    const TIMER_CHECK_FREQ: u64 = 1024;

    pub fn new(search_limit: SearchLimit, zobrist_stack: &ZobristStack, history: &History) -> Self {
        Self {
            search_limit,
            zobrist_stack: zobrist_stack.clone(),
            history: history.clone(),
            killers: Killers::new(),
            pv_table: PvTable::new(),
            timer: None,
            out_of_time: false,
            node_count: 0,
            seldepth: 0,
        }
    }

    pub fn search_complete_actions(&self, uci_history: &mut History) {
        *uci_history = self.history.clone();
        uci_history.age_scores();
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

    const fn stop_searching(&self, depth: Depth) -> bool {
        if depth == MAX_DEPTH {
            return true;
        }

        match self.search_limit {
            SearchLimit::Time(_) => self.out_of_time,
            SearchLimit::Depth(depth_limit) => depth > depth_limit,
            SearchLimit::Nodes(node_limit) => self.node_count > node_limit,
            SearchLimit::None => true,
        }
    }

    fn is_out_of_time(&self) -> bool {
        if self.node_count % Self::TIMER_CHECK_FREQ == 0 {
            if let Some(timer) = self.timer {
                return timer.is_expired();
            }
        }

        false
    }

    pub fn bench(&mut self, board: &Board, depth: Depth) -> Nodes {
        for d in 1..depth {
            self.negamax(board, d, 0, -INF, INF);
        }

        self.node_count
    }

    pub fn go(&mut self, board: &Board, report_info: bool) -> SearchResults {
        if let SearchLimit::Time(limit) = self.search_limit {
            self.timer = Some(SearchTimer::new(limit));
        }

        let stopwatch = std::time::Instant::now();
        let mut depth: Depth = 1;

        let mut search_results = SearchResults::new(board);
        while !self.stop_searching(depth) {
            self.seldepth = 0;

            let score = self.negamax(board, depth, 0, -INF, INF);

            if self.out_of_time {
                break;
            }

            if report_info {
                self.report_search_info(score, depth, stopwatch);
            }
            search_results.best_move = self.pv_table.best_move();
            search_results.score = score;

            depth += 1;
        }

        assert!(
            search_results.best_move.to() != search_results.best_move.from(),
            "INVALID MOVE"
        );

        if report_info {
            println!("bestmove {}", search_results.best_move.as_string());
        }

        search_results
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

        if self.is_out_of_time() {
            self.out_of_time = true;
            return 0;
        }

        self.seldepth = self.seldepth.max(ply);

        let hash_base = ZobristHash::incremental_update_base(board);

        let mut generator = MoveGenerator::new();

        let mut best_score = -INF;
        let mut moves_played = 0;
        let mut quiets: ArrayVec<Move, MAX_MOVECOUNT> = ArrayVec::new();
        while let Some(mv) = generator.next::<true>(board, &self.history, self.killers.killer(ply))
        {
            let mut next_board = (*board).clone();
            let is_legal = next_board.try_play_move(mv, &mut self.zobrist_stack, hash_base);
            if !is_legal {
                continue;
            }

            self.node_count += 1;
            moves_played += 1;

            let score = -self.negamax(&next_board, depth - 1, ply + 1, -beta, -alpha);

            self.zobrist_stack.revert_state();

            if self.out_of_time {
                return 0;
            }

            let is_quiet = generator.is_quiet_stage();
            // I am well aware that this does not include killer moves, but for
            // some reason it loses Elo when I include them. Screw engine development.
            if is_quiet {
                quiets.push(mv);
            }

            if score > best_score {
                best_score = score;

                if score >= beta {
                    if is_quiet {
                        self.killers.update(mv, ply);
                        self.history.update(board, quiets.as_slice(), depth);
                    }
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
        if self.is_out_of_time() {
            self.out_of_time = true;
            return 0;
        }

        self.seldepth = self.seldepth.max(ply);

        let stand_pat = evaluate(board);
        if stand_pat >= beta {
            return stand_pat;
        }

        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let hash_base = ZobristHash::incremental_update_base(board);

        let mut generator = MoveGenerator::new();

        let mut best_score = stand_pat;
        while let Some(mv) = generator.next::<false>(board, &self.history, Move::nullmove()) {
            let mut next_board = (*board).clone();
            let is_legal = next_board.try_play_move(mv, &mut self.zobrist_stack, hash_base);
            if !is_legal {
                continue;
            }

            self.node_count += 1;

            let score = -self.qsearch(&next_board, ply + 1, -beta, -alpha);

            self.zobrist_stack.revert_state();

            if self.out_of_time {
                return 0;
            }

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
