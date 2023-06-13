use engine::{
    board_representation::{Board, START_FEN},
    chess_move::Move,
    movegen::MoveGenerator,
    search::{Ply, Searcher},
    time_management::{SearchTimer, TimeArgs, TimeManager},
    zobrist::ZobristHash,
    zobrist_stack::{self, ZobristStack},
};

use crate::rng::Rng;

pub struct DataGenerator {
    rng: Rng,
    games_played: u64,

    board: Board,
    zobrist_stack: ZobristStack,
    time_manager: TimeManager,
    time_args: TimeArgs,
}

impl DataGenerator {
    const BASE_RAND_PLY: Ply = 9;
    const WIN: i8 = 1;
    const DRAW: i8 = 0;
    const LOSS: i8 = -1;

    pub fn new(move_time: u128) -> Self {
        let board = Board::from_fen(START_FEN);
        Self {
            rng: Rng::new(),
            games_played: 0,
            board: board.clone(),
            zobrist_stack: ZobristStack::new(&board),
            time_manager: TimeManager::new(0),
            time_args: TimeArgs {
                move_time,
                ..TimeArgs::default()
            },
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
            let mut searcher = Searcher::new(timer, self.zobrist_stack.clone(), None);
            let search_results = searcher.go(&self.board, false);
        }
    }

    pub fn generate_data(&mut self, game_count: u32) {
        for _ in 0..game_count {
            self.set_random_opening();

            self.record_game();

            self.games_played += 1;
        }
    }
}
