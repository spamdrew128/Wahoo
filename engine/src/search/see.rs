use crate::board::{
    board_representation::{Board, Piece, NUM_PIECES},
    chess_move::{Flag, Move},
};

pub const SEE_VALS: [i32; NUM_PIECES as usize] = [450, 450, 650, 1250, 100, 0];

impl Board {
    fn gain(&self, mv: Move, victim: Piece) -> i32 {
        if mv.flag() == Flag::EP {
            return SEE_VALS[Piece::PAWN.as_index()];
        }

        let base = SEE_VALS[victim.as_index()];
        if mv.is_promo() {
            base + SEE_VALS[mv.promo_piece().as_index()] - SEE_VALS[Piece::PAWN.as_index()]
        } else {
            base
        }
    }

    fn see(&self, mv: Move, attacker: Piece, victim: Piece) {}
}
