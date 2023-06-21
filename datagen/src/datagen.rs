use std::{
    fs::File,
    io::{BufWriter, Write},
};

use engine::{
    board_representation::{Board, Color, START_FEN},
    chess_move::Move,
    evaluation::{evaluate, EvalScore, INF, MATE_THRESHOLD},
    history_table::History,
    movegen::MoveGenerator,
    search::{Ply, SearchLimit, SearchResults, Searcher},
    transposition_table::TranspositionTable,
    zobrist::ZobristHash,
    zobrist_stack::ZobristStack,
};

use crate::rng::Rng;

fn simple_qsearch(board: &Board, mut alpha: EvalScore, beta: EvalScore) -> EvalScore {
    let stand_pat = evaluate(board);
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut generator = MoveGenerator::new();

    let mut best_score = stand_pat;
    while let Some(mv) = generator.simple_next::<false>(board) {
        let mut next_board = board.clone();
        let is_legal = next_board.simple_try_play_move(mv);
        if !is_legal {
            continue;
        }

        let score = -simple_qsearch(&next_board, -beta, -alpha);

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

fn pos_is_quiet(board: &Board) -> bool {
    simple_qsearch(board, -INF, INF) == evaluate(board)
}

pub struct DataGenerator {
    rng: Rng,
    games_played: u64,
    positions_written: u64,

    board: Board,
    zobrist_stack: ZobristStack,
    search_limit: SearchLimit,

    file: BufWriter<File>,
}

impl DataGenerator {
    const BASE_RAND_PLY: Ply = 9;
    const WIN: &str = "1.0";
    const DRAW: &str = "0.5";
    const LOSS: &str = "0.0";

    pub fn new(search_limit: SearchLimit, path: &str) -> Self {
        let board = Board::from_fen(START_FEN);
        Self {
            rng: Rng::new(),
            games_played: 0,
            positions_written: 0,
            board: board.clone(),
            zobrist_stack: ZobristStack::new(&board),
            search_limit,
            file: BufWriter::new(File::create(path).unwrap()),
        }
    }

    fn random_legal_move(&mut self, board: &Board) -> Option<Move> {
        let mut generator = MoveGenerator::new();
        let mut move_list: Vec<Move> = vec![];
        while let Some(mv) = generator.simple_next::<true>(board) {
            let mut board_clone = board.clone();
            if board_clone.simple_try_play_move(mv) {
                move_list.push(mv);
            }
        }

        if move_list.is_empty() {
            return None;
        }

        let index = (self.rng.rand_u64() as usize) % move_list.len();
        Some(move_list[index])
    }

    fn set_random_opening(&mut self) {
        loop {
            let mut board = Board::from_fen(START_FEN);
            let mut zobrist_stack = ZobristStack::new(&board);

            let mut success = true;
            for _ in 0..(Self::BASE_RAND_PLY + ((self.games_played % 2) as u8)) {
                if let Some(mv) = self.random_legal_move(&board) {
                    assert!(board.try_play_move(
                        mv,
                        &mut zobrist_stack,
                        ZobristHash::incremental_update_base(&board)
                    ));
                } else {
                    success = false;
                    break;
                }
            }

            if success && !MoveGenerator::no_legal_moves(&board) {
                self.board = board;
                self.zobrist_stack = zobrist_stack;
                return;
            }
        }
    }

    fn record_game(&mut self) {
        let mut positions: Vec<Board> = vec![];
        let mut result = Self::DRAW;

        let mut history = History::new();
        let tt = TranspositionTable::new(16);
        loop {
            let mut searcher = Searcher::new(self.search_limit, &self.zobrist_stack, &history, &tt);
            let SearchResults { best_move, score } = searcher.go(&self.board, false);
            searcher.search_complete_actions(&mut history);

            if score > MATE_THRESHOLD {
                result = match self.board.color_to_move {
                    Color::White => Self::WIN,
                    Color::Black => Self::LOSS,
                };
                break;
            } else if score < -MATE_THRESHOLD {
                result = match self.board.color_to_move {
                    Color::White => Self::LOSS,
                    Color::Black => Self::WIN,
                };
                break;
            } else if self.zobrist_stack.twofold_repetition(self.board.halfmoves)
                || self.board.insufficient_material_draw()
                || self.board.fifty_move_draw()
            {
                break;
            }

            positions.push(self.board.clone());
            self.board.try_play_move(
                best_move,
                &mut self.zobrist_stack,
                ZobristHash::incremental_update_base(&self.board),
            );

            if MoveGenerator::no_legal_moves(&self.board) {
                break;
            }
        }

        for board in positions {
            if pos_is_quiet(&board) {
                writeln!(&mut self.file, "{} [{}]", board.to_fen(), result).unwrap();
                self.positions_written += 1;
            }
        }
    }

    pub fn generate_data(&mut self, game_count: u32) {
        for _ in 0..game_count {
            self.set_random_opening();

            self.record_game();

            self.games_played += 1;
            println!("{} games played", self.games_played);
            println!("{} positions_saved\n", self.positions_written);
        }

        self.file.flush().unwrap();
    }
}
