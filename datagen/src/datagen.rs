use engine::{board_representation::{Board, START_FEN, Color}, movegen::MoveGenerator, chess_move::Move, search::Ply, zobrist_stack::ZobristStack, zobrist::ZobristHash, time_management::{SearchTimer, TimeManager, TimeArgs}};

use crate::rng::Rng;

struct DataGenerator {
    rng: Rng,
    games_played: u64,
    startpos: Board,

    board: Board,
    zobrist_stack: ZobristStack,
    time_args: TimeArgs,
}

impl DataGenerator {
    const BASE_RAND_PLY: Ply = 8;

    fn new(move_time: u128) -> Self {
        let board = Board::from_fen(START_FEN);
        Self {
            rng: Rng::new(),
            games_played: 0,
            startpos: board.clone(),
            board: board.clone(),
            zobrist_stack: ZobristStack::new(&board),
            time_args: TimeArgs {move_time, ..TimeArgs::default()},
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
            let mut board = self.startpos.clone();
            let mut success = true;
            for _ in 0..(Self::BASE_RAND_PLY + ((self.games_played % 2) as u8)) {
                if let Some(mv) = self.random_legal_move(&board) {
                    assert!(board.try_play_move(mv, &mut self.zobrist_stack, ZobristHash::incremental_update_base(&board)));
                } else {
                    success = false;
                    break;
                }
            }
            
            if success {
                self.board = board;
                return;
            }
        }
    }

    fn generate_data(&mut self, game_count: u32) {
        for _ in 0..game_count {
            self.set_random_opening();

            

            self.games_played += 1;
        }
    }
}

