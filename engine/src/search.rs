use std::{
    sync::atomic::{AtomicBool, Ordering, AtomicU64},
    time::Instant,
};

use arrayvec::ArrayVec;

use crate::{
    board_representation::Board,
    chess_move::{Move, MAX_MOVECOUNT},
    evaluation::{evaluate, EvalScore, EVAL_MAX, INF, MATE_THRESHOLD},
    history_table::History,
    killers::Killers,
    late_move_reductions::get_reduction,
    movegen::MoveGenerator,
    pv_table::PvTable,
    time_management::{Milliseconds, SearchTimer},
    transposition_table::{TTFlag, TranspositionTable},
    zobrist::ZobristHash,
    zobrist_stack::ZobristStack,
};

pub type Nodes = u64;
pub type Depth = i8;
pub type Ply = u8;
const MAX_DEPTH: Depth = i8::MAX;
pub const MAX_PLY: Ply = MAX_DEPTH as u8;

static STOP_FLAG: AtomicBool = AtomicBool::new(false);
static NODE_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn write_stop_flag(val: bool) {
    STOP_FLAG.store(val, Ordering::Relaxed);
}

pub fn stop_flag_is_set() -> bool {
    STOP_FLAG.load(Ordering::Relaxed)
}

fn reset_node_count() {
    NODE_COUNT.store(0, Ordering::Relaxed);
}

fn update_node_count(nodes: Nodes) {
    NODE_COUNT.fetch_add(nodes, Ordering::Relaxed);
}

fn node_count() -> Nodes {
    NODE_COUNT.load(Ordering::Relaxed)
}

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
}

#[derive(Debug)]
pub struct Searcher<'a> {
    search_limits: Vec<SearchLimit>,
    zobrist_stack: ZobristStack,
    pv_table: PvTable,
    history: History,
    killers: Killers,
    tt: &'a TranspositionTable,

    node_count: Nodes,
    timer: Option<SearchTimer>,
    seldepth: u8,
}

impl<'a> Searcher<'a> {
    const TIMER_CHECK_FREQ: u64 = 1024;

    pub fn new(
        search_limits: Vec<SearchLimit>,
        zobrist_stack: &ZobristStack,
        history: &History,
        tt: &'a TranspositionTable,
    ) -> Self {
        Self {
            search_limits,
            zobrist_stack: zobrist_stack.clone(),
            history: history.clone(),
            killers: Killers::new(),
            tt,
            pv_table: PvTable::new(),
            node_count: 0,
            timer: None,
            seldepth: 0,
        }
    }

    pub fn search_complete_actions(&self, uci_history: &mut History) {
        *uci_history = self.history.clone();
        uci_history.age_scores();
    }

    fn report_search_info(&self, score: EvalScore, depth: Depth, stopwatch: Instant) {
        let elapsed = stopwatch.elapsed();
        let nps = (u128::from(node_count()) * 1_000_000) / elapsed.as_micros().max(1);

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
            "score {score_str} nodes {} time {} nps {nps} depth {depth} seldepth {} hashfull {} pv {}",
            node_count(),
            elapsed.as_millis(),
            self.seldepth,
            self.tt.hashfull(),
            self.pv_table.pv_string()
        );
    }

    fn stop_searching<const IS_PRIMARY: bool>(&self, depth: Depth) -> bool {
        if depth == MAX_DEPTH {
            return true;
        }

        if !IS_PRIMARY {
            return false; // let secondary threads run until stop flag is set by main thread
        }

        let mut result = false;
        for &limit in &self.search_limits {
            result |= match limit {
                SearchLimit::Time(_) => self.timer.unwrap().soft_cutoff_is_expired(),
                SearchLimit::Depth(depth_limit) => depth > depth_limit,
                SearchLimit::Nodes(node_limit) => node_count() > node_limit,
            }
        }

        result
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
        write_stop_flag(false);
        let mut prev_score = 0;
        for d in 1..depth {
            prev_score = self.aspiration_window_search(board, prev_score, d, &mut Move::nullmove());
        }
        write_stop_flag(true);

        self.node_count
    }

    pub fn go<const IS_PRIMARY: bool>(&mut self, board: &Board, report_info: bool) -> SearchResults {
        if IS_PRIMARY {
            for &limit in &self.search_limits {
                if let SearchLimit::Time(t) = limit {
                    self.timer = Some(SearchTimer::new(t));
                }
            }

            reset_node_count();
        }

        let stopwatch = std::time::Instant::now();
        let mut depth: Depth = 1;

        let mut search_results = SearchResults::new(board);
        write_stop_flag(false);
        while !self.stop_searching::<IS_PRIMARY>(depth){
            self.seldepth = 0;
            self.node_count = 0;
            let score = self.aspiration_window_search(board, search_results.score, depth, &mut search_results.best_move);
            update_node_count(self.node_count);

            if stop_flag_is_set() {
                break;
            }

            if report_info {
                self.report_search_info(score, depth, stopwatch);
            }
            search_results.best_move = self.pv_table.best_move();
            search_results.score = score;

            depth += 1;
        }
        write_stop_flag(true);

        assert!(
            search_results.best_move.to() != search_results.best_move.from(),
            "INVALID MOVE"
        );

        if report_info {
            println!("bestmove {}", search_results.best_move.as_string());
        }

        search_results
    }

    fn aspiration_window_search(
        &mut self,
        board: &Board,
        prev_score: EvalScore,
        current_depth: Depth,
        best_move: &mut Move,
    ) -> EvalScore {
        const ASP_WINDOW_MIN_DEPTH: Depth = 7;
        const ASP_WINDOW_INIT_WINDOW: EvalScore = 12;
        const ASP_WINDOW_INIT_DELTA: EvalScore = 16;
        const ASP_WINDOW_FULL_SEARCH_BOUNDS: EvalScore = 3500;

        let mut alpha = -INF;
        let mut beta = INF;
        let mut asp_depth = current_depth;
        let mut delta = ASP_WINDOW_INIT_DELTA;

        if current_depth > ASP_WINDOW_MIN_DEPTH {
            alpha = (prev_score - ASP_WINDOW_INIT_WINDOW).max(-INF);
            beta = (prev_score + ASP_WINDOW_INIT_WINDOW).min(INF);
        }

        loop {
            if alpha < -ASP_WINDOW_FULL_SEARCH_BOUNDS {
                alpha = -INF;
            }
            if beta > ASP_WINDOW_FULL_SEARCH_BOUNDS {
                beta = INF;
            }

            let score = self.negamax::<false>(board, asp_depth, 0, alpha, beta);

            if score <= alpha {
                alpha = (alpha - delta).max(-INF);
                beta = (alpha + 3 * beta) / 4;
            } else if score >= beta {
                if asp_depth == current_depth {
                    *best_move = self.pv_table.best_move();
                }

                beta = (beta + delta).min(INF);
                asp_depth = (asp_depth - 1).max(1);
            } else {
                return score;
            }

            delta += delta * 2 / 3;
        }
    }

    #[rustfmt::skip]
    #[allow(clippy::cognitive_complexity)] // lol
    fn negamax<const DO_NULL_MOVE: bool>(
        &mut self,
        board: &Board,
        mut depth: Depth,
        ply: Ply,
        mut alpha: EvalScore,
        beta: EvalScore,
    ) -> EvalScore {
        self.pv_table.set_length(ply);

        let old_alpha = alpha;
        let in_check = board.in_check();
        let is_pv = beta != alpha + 1;
        let is_root = ply == 0;
        let is_drawn =
            self.zobrist_stack.twofold_repetition(board.halfmoves) || board.fifty_move_draw();

        if !is_root {
            if is_drawn {
                return 0;
            }

            // MATE DISTANCE PRUNING
            let mate_alpha = alpha.max(i32::from(ply) - MATE_THRESHOLD);
            let mate_beta = beta.min(MATE_THRESHOLD - (i32::from(ply) + 1));
            if mate_alpha >= mate_beta {
                return mate_alpha;
            }
        }

        // CHECK EXTENSION
        if in_check { depth += 1 };

        if depth == 0 || ply >= MAX_PLY {
            return self.qsearch(board, ply, alpha, beta);
        }

        if self.is_out_of_time() {
            write_stop_flag(true);
            return 0;
        }

        self.seldepth = self.seldepth.max(ply);

        let hash_base = ZobristHash::incremental_update_base(board);
        let hash = self.zobrist_stack.current_zobrist_hash();

        let tt_move = if let Some(entry) = self.tt.probe(hash) {
            if !is_pv && entry.cutoff_is_possible(alpha, beta, depth) {
                return entry.score_from_tt(ply);
            }

            entry.best_move
        } else {
            // INTERNAL ITERATIVE REDUCTION (IIR)
            const MIN_IIR_DEPTH: Depth = 3;
            if depth >= MIN_IIR_DEPTH {
                depth -= 1;
            }

            Move::nullmove()
        };

        if !is_pv && !in_check {
            let static_eval = evaluate(board);

            // NULL MOVE PRUNING
            const NMP_MIN_DEPTH: Depth = 3;
            if DO_NULL_MOVE && depth >= NMP_MIN_DEPTH && !board.we_only_have_pawns() && static_eval >= beta {
                let mut reduction = 3 + depth / 3 + (3.min((static_eval - beta) / 200) as Depth);
                reduction = reduction.min(depth);

                let mut nmp_board = board.clone();
                nmp_board.play_nullmove(&mut self.zobrist_stack);
                let null_move_score = -self.negamax::<false>(
                    &nmp_board,
                    depth - reduction,
                    ply + 1,
                    -beta,
                    -beta + 1,
                );

                self.zobrist_stack.revert_state();

                if null_move_score >= beta {
                    return null_move_score;
                }
            }

            // REVERSE FUTILITY PRUNING
            const RFP_MAX_DEPTH: Depth = 8;
            const RFP_MARGIN: EvalScore = 120;

            if depth <= RFP_MAX_DEPTH && static_eval >= (beta + RFP_MARGIN * i32::from(depth)) {
                return static_eval;
            }
        }

        let mut generator = MoveGenerator::new();

        let mut best_move = Move::nullmove();
        let mut best_score = -INF;
        let mut moves_played = 0;
        let mut quiets: ArrayVec<Move, MAX_MOVECOUNT> = ArrayVec::new();
        while let Some(mv) =
            generator.next::<true>(board, &self.history, self.killers.killer(ply), tt_move)
        {
            let mut next_board = board.clone();
            let is_legal = next_board.try_play_move(mv, &mut self.zobrist_stack, hash_base);
            if !is_legal {
                continue;
            }

            self.node_count += 1;
            moves_played += 1;

            let mut score = 0;
            if moves_played == 1 {
                score = -self.negamax::<true>(&next_board, depth - 1, ply + 1, -beta, -alpha);
            } else {
                // LATE MOVE REDUCTIONS (heavily inspired by Svart https://github.com/crippa1337/svart/blob/master/src/engine/search.rs)
                const LMR_MIN_DEPTH: Depth = 3;
                let lmr_threshold = if is_pv { 5 } else { 3 };

                let mut do_full_depth_pvs = true;
                if !in_check && depth >= LMR_MIN_DEPTH && moves_played > lmr_threshold {
                    let mut r = get_reduction(depth, moves_played);
                    if !is_pv {
                        r += 1;
                    }

                    if r > 1 {
                        // REDUCED PVS
                        r = r.min(depth - 1);
                        score = -self.negamax::<true>(&next_board, depth - r, ply + 1, -alpha - 1, -alpha);
                        do_full_depth_pvs = score > alpha && score < beta; // we want to try again without reductions if we beat alpha
                    }
                }

                if do_full_depth_pvs {
                    // FULL DEPTH PVS
                    score = -self.negamax::<true>(&next_board, depth - 1, ply + 1, -alpha - 1, -alpha);

                    // if our null-window search beat alpha without failing high, that means we might have a better move and need to re search with full window
                    if score > alpha && score < beta {
                        score = -self.negamax::<true>(&next_board, depth - 1, ply + 1, -beta, -alpha);
                    }
                }
            };

            self.zobrist_stack.revert_state();

            if stop_flag_is_set() {
                return 0;
            }

            let is_quiet = mv.is_quiet();
            if is_quiet {
                quiets.push(mv);
            }

            if score > best_score {
                best_move = mv;
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
            return if in_check {
                -EVAL_MAX + i32::from(ply)
            } else {
                0
            };
        }

        let tt_flag = TTFlag::determine(best_score, old_alpha, alpha, beta);
        self.tt.store(tt_flag, best_score, hash, ply, depth, best_move);
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
            write_stop_flag(true);
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
        while let Some(mv) =
            generator.next::<false>(board, &self.history, Move::nullmove(), Move::nullmove())
        {
            let mut next_board = board.clone();
            let is_legal = next_board.try_play_move(mv, &mut self.zobrist_stack, hash_base);
            if !is_legal {
                continue;
            }

            self.node_count += 1;

            let score = -self.qsearch(&next_board, ply + 1, -beta, -alpha);

            self.zobrist_stack.revert_state();

            if stop_flag_is_set() {
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
