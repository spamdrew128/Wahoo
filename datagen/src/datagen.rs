use engine::{board_representation::Board, movegen::MoveGenerator, chess_move::Move};

use crate::rng::Rng;

struct DataGenerator {
    rng: Rng
}

impl DataGenerator {
    fn new() -> Self {
        Self {
            rng: Rng::new()
        }
    }

    fn random_move(&mut self, board: &Board) -> Option<Move> {
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
}

