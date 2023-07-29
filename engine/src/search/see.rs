use crate::board::{
    board_representation::{Board, Piece, NUM_PIECES},
    chess_move::{Flag, Move},
};

pub const SEE_VALS: [i32; NUM_PIECES as usize] = [450, 450, 650, 1250, 100, 0];

impl Move {
    fn gain(self, victim: Piece) -> i32 {
        if self.flag() == Flag::EP {
            return SEE_VALS[Piece::PAWN.as_index()];
        }

        let base = SEE_VALS[victim.as_index()];
        if self.is_promo() {
            base + SEE_VALS[self.promo_piece().as_index()] - SEE_VALS[Piece::PAWN.as_index()]
        } else {
            base
        }
    }

    fn see(self, board: &Board, attacker: Piece, victim: Piece, threshold: i32) {
        let sq = self.to();
        let val = self.gain(victim) - threshold;
    }
}
