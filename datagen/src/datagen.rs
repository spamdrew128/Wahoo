use std::{
    fs::File,
    io::{BufWriter, Write},
};

use engine::{
    board_representation::{Board, Color, START_FEN},
    chess_move::Move,
    evaluation::MATE_THRESHOLD,
    movegen::MoveGenerator,
    search::{Depth, Ply, SearchResults, Searcher},
    time_management::{Milliseconds, TimeArgs, TimeManager},
    zobrist::ZobristHash,
    zobrist_stack::ZobristStack,
};

use crate::rng::Rng;

pub struct DataGenerator {
    rng: Rng,
    games_played: u64,

    board: Board,
    zobrist_stack: ZobristStack,
    time_manager: TimeManager,
    time_args: TimeArgs,
    depth_limit: Option<Depth>,

    file: BufWriter<File>,
}

impl DataGenerator {
    const BASE_RAND_PLY: Ply = 9;
    const WIN: i8 = 1;
    const DRAW: i8 = 0;
    const LOSS: i8 = -1;

    pub fn new(move_time: Milliseconds, depth_limit: Option<Depth>, path: &str) -> Self {
        let board = Board::from_fen(START_FEN);
        let time_args = if depth_limit.unwrap_or(0) > 0 {
            TimeArgs {
                infinite_mode: true,
                ..TimeArgs::default()
            }
        } else {
            TimeArgs {
                move_time,
                ..TimeArgs::default()
            }
        };
        Self {
            rng: Rng::new(),
            games_played: 0,
            board: board.clone(),
            zobrist_stack: ZobristStack::new(&board),
            time_manager: TimeManager::new(0),
            time_args,
            depth_limit,
            file: BufWriter::new(File::create(path).unwrap()),
        }
    }

    fn random_legal_move(&mut self, board: &Board) -> Option<Move> {
        let mut generator = MoveGenerator::new();
        let mut move_list: Vec<Move> = vec![];
        while let Some(mv) = generator.next::<true>(board) {
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

            let mut generator = MoveGenerator::new();
            if generator.next::<true>(&board).is_none() {
                success = false;
            }

            if success {
                self.board = board;
                self.zobrist_stack = zobrist_stack;
                return;
            }
        }
    }

    fn record_game(&mut self) {
        let mut positions: Vec<Board> = vec![];
        let mut result = Self::DRAW;

        loop {
            let timer = self
                .time_manager
                .construct_search_timer(self.time_args, self.board.color_to_move);
            let mut searcher = Searcher::new(timer, self.zobrist_stack.clone(), self.depth_limit);
            let SearchResults { best_move, score } = searcher.go(&self.board, false);

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
        }

        for board in positions {
            write!(&mut self.file, "{} [{}]\n", board.to_fen(), result).unwrap();
        }
    }

    pub fn generate_data(&mut self, game_count: u32) {
        for _ in 0..game_count {
            self.set_random_opening();

            self.record_game();

            self.games_played += 1;
            println!("{} games played", self.games_played);
        }

        self.file.flush().unwrap();
    }
}
